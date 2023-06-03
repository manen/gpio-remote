use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
/// Message represents the packets sent from the sender to the receiver
pub enum Message {
	OpenIn { id: u16 },
	OpenOut { id: u16 },
	RequestIn { id: u16 },
	SetOut { id: u16, val: bool },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
/// Response represents the packets the receiver sends to the sender
pub enum Response {
	InValue { id: u16, val: bool },
}
