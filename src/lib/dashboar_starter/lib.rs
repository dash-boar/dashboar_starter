use crate::data::NewGuiState;
use rand::Rng;

pub mod data;
pub mod layout;
pub mod msg;

pub fn randomly_flip_off_servers(curr_state: &mut NewGuiState) {
    // randomly flip off the status of one of them
    // to simulate a server going down

    let mut rng = rand::thread_rng();

    let server_status_down = curr_state
        .connections
        .iter_mut()
        .find(|s| s.flag && !s.status);
    match server_status_down {
        None => {
            // randomly flip one off

            // if all flags are off return
            // count the number of flags that are ON
            let num_on_flags = curr_state.connections.iter().filter(|s| s.flag).count();
            if num_on_flags == 0 {
                return; // none are ON
            }

            let random_server_idx = rng.gen_range(0..num_on_flags);
            let random_server = curr_state
                .connections
                .iter_mut()
                .filter(|s| s.flag)
                .nth(random_server_idx)
                .unwrap();

            // only do this only so often, to make it happen more slowly
            let random_number = rng.gen_range(0..10);
            if random_number != 0 {
                return;
            }

            random_server.status = false;
        }
        Some(server_status_down) => {
            // randomly bring it back up, or flip off the flag too
            let random_number = rng.gen_range(0..5);
            if random_number == 0 {
                // it took down even the flag
                server_status_down.flag = false;
            } else {
                // it came back up!
                server_status_down.status = true;
            }
        }
    }
}
