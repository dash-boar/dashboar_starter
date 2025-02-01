use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum GuiTx {
    Hello,
    ConnectionFlag(ConnectionFlagUpdate),
    ChangeName(ChangeName),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConnectionFlagUpdate {
    pub id: String,
    pub connect_flag: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChangeName {
    pub id: String,
    pub new_name: String,
}
