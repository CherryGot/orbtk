use std::sync::{Arc, Mutex};

use dces::entity::Entity;

use crate::event::*;

/// The `EventAdapter` provides a thread save way to push event to the widget tree of a window.
///
/// # Example
///
/// ```rust
/// impl State for MayState {
///     fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
///         let event_adapter = ctx.event_adapter();
///         let entity = ctx.entity();
///         
///         let _ = std::thread::spawn(move|| {
///             loop {
///                 // pushes all 10 milliseconds an activate event on the entity of the state
///                 event_adapter.push_event(entity, ActivateEvent(entity));
///                 std::thread::sleep(std::time::Duration::from_millis(10));
///             }
///         });
///     }    
/// }
/// ```
#[derive(Clone, Default, Debug)]
pub struct EventAdapter {
    event_queue: Arc<Mutex<EventQueue>>,
}

impl EventAdapter {
    /// Creates a new event adapter.
    pub fn new() -> Self {
        EventAdapter::default()
    }

    /// Push an event to the tree starting by the given entity. The event bubbles through the tree until it is handled.
    pub fn push_event<E: Event + Send>(&self, entity: Entity, event: E) {
        self.event_queue
            .lock()
            .expect("EventAdapter::push_event: Cannot lock event queue.")
            .register_event(event, entity);
    }

    /// Pushes an event that is direct send to the given entity (widget). I occurs only by the given entity and will not bubble through the tree.
    pub fn push_event_direct<E: Event + Send>(&self, entity: Entity, event: E) {
        self.event_queue
            .lock()
            .expect("EventAdapter::push_event_direct: Cannot lock event queue")
            .register_event_with_strategy(event, EventStrategy::Direct, entity);
    }

    /// Returns the number of events in the queue.
    pub fn len(&self) -> usize {
        self.event_queue
            .lock()
            .expect("EventAdapter::len: Cannot lock event queue.")
            .len()
    }

    /// Returns `true` if the event queue contains no events.
    pub fn is_empty(&self) -> bool {
        self.event_queue
            .lock()
            .expect("EventAdapter::is_empty: Cannot lock event queue.")
            .is_empty()
    }

    /// Returns an dequeue iterator, that dequeue events from the event queue.
    pub(crate) fn dequeue(&self) -> EventDequeueIterator {
        EventDequeueIterator {
            event_adapter: self.clone(),
        }
    }
}

/// Dequeue iterator through a list of events.
pub struct EventDequeueIterator {
    event_adapter: EventAdapter,
}

impl Iterator for EventDequeueIterator {
    type Item = EventBox;

    fn next(&mut self) -> Option<Self::Item> {
        self.event_adapter
            .event_queue
            .lock()
            .expect("EventDequeueIterator::next: Cannot lock event queue.")
            .dequeue()
    }
}
