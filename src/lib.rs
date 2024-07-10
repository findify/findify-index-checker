use bytes::Bytes;
use pulsar::{DeserializeMessage, Payload};
use crate::findify::sync::IndexPublished;
use prost::Message;

pub mod findify {
    include!(concat!(env!("OUT_DIR"), "/findify.rs"));

    pub mod sync {
        include!(concat!(env!("OUT_DIR"), "/io.findify.sync.rs"));
    }
}

impl DeserializeMessage for IndexPublished {
    type Output = Result<IndexPublished, prost::DecodeError>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        let bytes = Bytes::copy_from_slice(&payload.data);
        IndexPublished::decode(bytes)
    }
}
