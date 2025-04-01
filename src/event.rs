use stellar_axelar_std::{Bytes, IntoEvent, String};

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct ExecutedEvent {
    pub source_chain: String,
    pub message_id: String,
    pub source_address: String,
    #[data]
    pub payload: Bytes,
}