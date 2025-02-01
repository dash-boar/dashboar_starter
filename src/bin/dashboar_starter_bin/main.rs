use dashboar::{json_patch, DashboarRx, Layout};
use dashboar_starter::data::{starting_gui_state, NewGuiState};
use dashboar_starter::layout::layout;
use dashboar_starter::msg::GuiTx;
use dashboar_starter::randomly_flip_off_servers;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use serde_json::{from_str, json, to_string, to_value, Value};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use std::{collections::HashMap, env, io::Error as IoError, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

pub struct Server {
    clients: HashMap<SocketAddr, UnboundedSender<Message>>,
    state: NewGuiState,
    prev_sent_state: Value,
    was_flag_enabled: bool,
}

async fn handle_connection(server: Rc<RefCell<Server>>, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    server.borrow_mut().clients.insert(addr, tx.clone());

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        match msg {
            Message::Text(msg) => {
                println!("Received a message from {}: {}", addr, msg);

                let msg: GuiTx = from_str(&msg).unwrap();

                match msg {
                    GuiTx::Hello => {}
                    GuiTx::ConnectionFlag(msg) => {
                        let mut server = server.borrow_mut();
                        let to_update =
                            server.state.connections.iter_mut().find(|x| x.id == msg.id);
                        if let Some(to_update) = to_update {
                            to_update.flag = msg.connect_flag;

                            match msg.connect_flag {
                                true => {
                                    server.was_flag_enabled = true;
                                }
                                false => {
                                    // also take down the status
                                    to_update.status = false;
                                }
                            }
                        }
                    }
                    GuiTx::ChangeName(msg) => {
                        let mut server = server.borrow_mut();
                        let to_update =
                            server.state.connections.iter_mut().find(|x| x.id == msg.id);
                        if let Some(to_update) = to_update {
                            to_update.name = msg.new_name;
                        }
                    }
                }
            }
            Message::Binary(_) => {}
            Message::Ping(_) => {}
            Message::Pong(_) => {}
            Message::Close(_) => {}
            Message::Frame(_) => {}
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    let layout_msg = Layout::V0(layout());
    let layout_msg = DashboarRx::Layout(layout_msg);
    let layout_msg = to_string(&layout_msg).unwrap();
    tx.unbounded_send(Message::Text(layout_msg)).unwrap();
    println!("Sent layout message: {}", addr);

    let data_msg = DashboarRx::DataSnapshot(server.borrow().prev_sent_state.clone());
    let data_msg = to_string(&data_msg).unwrap();
    tx.unbounded_send(Message::Text(data_msg)).unwrap();
    println!("Sent data snapshot message: {}", addr);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    server.borrow_mut().clients.remove(&addr);
}

async fn broadcast_data_every_500ms(server: Rc<RefCell<Server>>) {
    let mut interval = tokio::time::interval(Duration::from_millis(500));

    loop {
        interval.tick().await;

        if server.borrow().clients.is_empty() {
            continue;
        }

        // update GUI state
        // if a flag was just turned on, don't flip it off
        // or it will look like it didn't get turned on
        let was_flag_enabled = server.borrow().was_flag_enabled;
        if !was_flag_enabled {
            randomly_flip_off_servers(&mut server.borrow_mut().state);
        }

        let new_gui_state = to_value(&server.borrow().state).unwrap();

        let diff = json_patch::diff(&server.borrow().prev_sent_state, &new_gui_state);
        if diff == json_patch::Patch(vec![]) {
            continue;
        }

        let dashboar_msg = DashboarRx::DataPatch(diff);
        let dashboar_msg = to_string(&dashboar_msg).unwrap();

        for (_, tx) in server.borrow().clients.iter() {
            tx.unbounded_send(Message::Text(dashboar_msg.clone()))
                .unwrap();
        }

        server.borrow_mut().prev_sent_state = new_gui_state;
        server.borrow_mut().was_flag_enabled = false;
    }
}

async fn run() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let server = Rc::new(RefCell::new(Server {
        clients: HashMap::new(),
        state: starting_gui_state(),
        prev_sent_state: json!({}),
        was_flag_enabled: false,
    }));

    tokio::task::spawn_local(broadcast_data_every_500ms(server.clone()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::task::spawn_local(handle_connection(server.clone(), stream, addr));
    }

    Ok(())
}

fn main() -> Result<(), IoError> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    tokio::task::LocalSet::new().block_on(&rt, run())
}
