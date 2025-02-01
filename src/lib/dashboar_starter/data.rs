use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Status {
    pub id: String,
    pub name: String,
    pub server_location: String,
    pub flag: bool,
    pub status: bool,
}

#[derive(Serialize, Clone)]
pub struct NewGuiState {
    pub connections: Vec<Status>,
}

pub fn starting_gui_state() -> NewGuiState {
    NewGuiState {
        connections: vec![
            Status {
                id: "eu-west-1".to_string(),
                name: "IRE".to_string(),
                server_location: "Ireland".to_string(),
                flag: true,
                status: true,
            },
            Status {
                id: "ap-southeast-1".to_string(),
                name: "SG".to_string(),
                server_location: "Singapore".to_string(),
                flag: true,
                status: true,
            },
        ],
    }
}
