extern crate uuid;
use uuid::Uuid;

use crate::net::{Payload, Route};

#[derive(Clone, Debug)]
pub struct Message {
    conversation_id: Uuid,
    to: Route,
    from: Route,
    payload: Option<Box<Payload>>,
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

    pub fn with_payload<T: Payload>(self, pl: T) -> MessageBuilder {
        let mut ret = self.clone();
        ret._m.payload = Some(Box::new(pl));
        ret
    }

    pub fn finish(self) -> Message {
        self._m
    }
}
