# DashBoar Starter Backend

Use this as a starting point or reference for creating your own Backend and Custom Boards on [https://dashboar.net](https://dashboar.net).

Want to see a Demo, but don't want to run your own Websocket Server? Check out the [DashBoar Demo](https://dashboar.net/demo).

Running this Starter Backend and connecting the [Starter Board](https://dashboar.net/starter) will recreate the [DashBoar Demo](https://dashboar.net/demo) using real Websockets.

## Pre-Requisites

Install Rust if you haven't already. Following the instructions at: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Setup

1. Clone (or Download) the `dashboar` Core Library (containing all of the UI Components): [https://github.com/dash-boar/dashboar](https://github.com/dash-boar/dashboar)
2. Clone (or Download) this Repository: `dashboar_starter`

The Directory Structure should be as follows:

```
dashboar/
dashboar_starter/
```
3. Run the `dashboar_starter_bin` Binary. This will run a Websocket Server on your machine that the [Starter Board](https://dashboar.net/starter) can connect to.
4. When the `dashboar_starter_bin` app is running. Press the `Connection` button on the [Starter Board](https://dashboar.net/starter), for the Board to connect to your Websocket Server.

### Example Terminal Commands and output for the above:
```
$ git clone git@github.com:dash-boar/dashboar.git
$ git clone git@github.com:dash-boar/dashboar_starter.git
$ cd dashboar_starter
$ cargo run --bin dashboar_starter_bin
<-- Output from Cargo building the app -->
     Running `target/debug/dashboar_starter_bin`
Listening on: 127.0.0.1:8080
Incoming TCP connection from: 127.0.0.1:51946
WebSocket connection established: 127.0.0.1:51946
Sent layout message: 127.0.0.1:51946
Sent data snapshot message: 127.0.0.1:51946
Received a message from 127.0.0.1:51946: {"action":"hello"}
```

By Default, `dashboar_starter_bin` will listen on "127.0.0.1:8080", and the [Starter Board](https://dashboar.net/starter) will connect to "ws://127.0.0.1:8080" (the same address). If you need to change the address, you can do the following:

- Change the Listening Address on `dashboar_starter_bin` by:
  - Providing a different address as the first Command Line Argument:
    - i.e.  ``` $ cargo run --bin dashboar_starter_bin -- 127.0.0.1:8081```
  - Or modifying the code of `dashboar_starter_bin` to listen on a different address.
- Change the Connection URL on the [Starter Board](https://dashboar.net/starter) in the IO Settings, within the Board Settings (see the top right of the page).
