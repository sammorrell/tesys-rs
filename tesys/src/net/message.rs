extern crate uuid;
use uuid::Uuid;

use crate::net::{Payload, Route};

#[derive(Clone, Debug)]
pub struct Message {
    conversation_id: Uuid,
    to: Route,
    from: Route,
    pub payload: Option<Box<dyn Payload>>,
}

impl Message {
    pub fn new() -> MessageBuilder {
        MessageBuilder::new()
    }

    pub fn blank() -> Message {
        Message {
            conversation_id: Uuid::new_v4(),
            to: Route::blank(),
            from: Route::blank(),
            payload: None,
        }
    }

    pub fn get_conversation_id(&self) -> Uuid {
        self.conversation_id.clone()
    }

    pub fn get_payload_obj(&self) -> Option<Box<dyn Payload>> {
        self.payload.clone()
    }

    pub fn get_payload<T: Clone + Payload + Sync + Send + 'static>(&self) -> Result<T, ()> {
        T::unpack(self.get_payload_obj().unwrap())
    }
}

#[derive(Clone)]
pub struct MessageBuilder {
    _m: Message,
}

impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder {
            _m: Message::blank(),
        }
    }

    pub fn in_conversation(self, conv_id: Uuid) -> MessageBuilder {
        let mut ret = self.clone();
        ret._m.conversation_id = conv_id;
        ret
    }

    pub fn to(self, to: Route) -> MessageBuilder {
        let mut ret = self.clone();
        ret._m.to = to;
        ret
    }

    pub fn from(self, from: Route) -> MessageBuilder {
        let mut ret = self.clone();
        ret._m.from = from;
        ret
    }

    pub fn with_payload<T: Payload + Clone>(self, pl: T) -> MessageBuilder {
        let mut ret = self.clone();
        ret._m.payload = Some(T::pack(pl));
        ret
    }

    pub fn finish(self) -> Message {
        self._m
    }
}
