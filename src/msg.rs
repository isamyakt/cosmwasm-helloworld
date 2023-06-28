use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GreetResp {
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum QueryMsg {
    Greet {},
}