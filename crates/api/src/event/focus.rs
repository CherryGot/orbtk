use dces::prelude::Entity;

use crate::{
    prelude::*,
    proc_macros::{Event, IntoHandler},
    widget_base::MessageSender,
};

/// Used to request keyboard focus on the window.
#[derive(Event, Clone)]
pub enum FocusEvent {
    RequestFocus(Entity),
    RemoveFocus(Entity),
}

pub type FocusHandlerFn = dyn Fn(MessageSender, FocusEvent) -> bool + 'static;

#[derive(IntoHandler)]
pub struct FocusEventHandler {
    pub handler: Rc<FocusHandlerFn>,
}

impl EventHandler for FocusEventHandler {
    fn handle_event(&self, states: MessageSender, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<FocusEvent>() {
            return (self.handler)(states, event.clone());
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<FocusEvent>()
    }
}
