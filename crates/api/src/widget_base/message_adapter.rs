use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use dces::entity::Entity;

#[derive(Debug)]
pub struct MessageBox {
    message: Box<dyn Any + Send>,
    message_type: TypeId,
    target: Entity,
}

impl MessageBox {
    /// Creates a new `MessageBox`.
    pub fn new<M: Any + Send>(message: M, target: Entity) -> Self {
        MessageBox {
            message: Box::new(message),
            target,
            message_type: TypeId::of::<M>(),
        }
    }

    /// Check if the given type is the type of the message.
    pub fn is_type<M: Any>(&self) -> bool {
        self.message_type == TypeId::of::<M>()
    }

    /// Returns the type of the event.
    pub fn message_type(&self) -> TypeId {
        self.message_type
    }

    /// Downcasts the box to an concrete message.
    pub fn downcast<M: Any>(self) -> Result<M, String> {
        if self.message_type == TypeId::of::<M>() {
            return Ok(*self.message.downcast::<M>().unwrap());
        }

        Err("Wrong message type".to_string())
    }

    /// Downcasts the box as reference of an concrete message.
    pub fn downcast_ref<M: Any>(&self) -> Result<&M, String> {
        if self.message_type == TypeId::of::<M>() {
            return Ok(&*self.message.downcast_ref::<M>().unwrap());
        }

        Err("Wrong message type".to_string())
    }
}

#[derive(Clone, Default, Debug)]
pub struct MessageAdapter {
    messages: Arc<Mutex<BTreeMap<Entity, Vec<MessageBox>>>>,
}

impl MessageAdapter {
    pub fn new() -> Self {
        MessageAdapter::default()
    }

    pub fn push_message<M: Any + Send>(&self, target: Entity, message: M) {
        if !self
            .messages
            .lock()
            .expect("MessageAdapter::push_message: Cannot lock message queue.")
            .contains_key(&target)
        {
            self.messages
                .lock()
                .expect("MessageAdapter::push_message: Cannot lock message queue.")
                .insert(target, vec![]);
        }

        self.messages
            .lock()
            .expect("MessageAdapter::push_message: Cannot lock message queue.")
            .get_mut(&target)
            .unwrap()
            .push(MessageBox::new(message, target));
    }

    /// Returns the number of messages in the queue.
    pub fn len(&self) -> usize {
        self.messages
            .lock()
            .expect("EventAdapter::len: Cannot lock message queue.")
            .len()
    }

    /// Returns `true` if the event message contains no events.
    pub fn is_empty(&self) -> bool {
        self.messages
            .lock()
            .expect("EventAdapter::is_empty: Cannot lock message queue.")
            .is_empty()
    }

    pub fn message_reader<M: Any + Send>(&self, target: Entity) -> MessageReader<M> {
        if let Some(messages) = self
            .messages
            .lock()
            .expect("EventAdapter::message_reader: Cannot lock message queue.")
            .remove(&target)
        {
            return MessageReader::new(messages, target);
        }

        MessageReader::new(vec![], target)
    }

    pub fn message_sender(&self) -> MessageSender {
        MessageSender::new(self.clone())
    }
}

#[derive(Debug)]
pub struct MessageReader<M>
where
    M: Any + Send,
{
    messages: Vec<MessageBox>,
    target: Entity,
    _phatom: PhantomData<M>,
}

impl<M> MessageReader<M>
where
    M: Any + Send,
{
    pub(crate) fn new(messages: Vec<MessageBox>, target: Entity) -> Self {
        MessageReader {
            messages,
            target,
            _phatom: PhantomData::default(),
        }
    }
}

impl<M> Iterator for MessageReader<M>
where
    M: Any + Send,
{
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self
            .messages
            .iter()
            .position(|m| m.target == self.target && m.type_id() == TypeId::of::<M>())
        {
            return Some(self.messages.remove(index).downcast::<M>().unwrap());
        }

        None
    }
}

pub struct MessageSender {
    adapter: MessageAdapter,
}

impl MessageSender {
    pub(crate) fn new(adapter: MessageAdapter) -> Self {
        MessageSender { adapter }
    }

    pub fn send<M: Any + Send>(&self, message: M, target: Entity) {
        self.adapter.push_message(target, message);
    }
}