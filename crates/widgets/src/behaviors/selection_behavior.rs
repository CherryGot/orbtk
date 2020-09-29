use crate::{api::prelude::*, proc_macros::*};

enum Action {
    ToggleSelection,
    UpdateVisualState,
}

/// The `SelectionBehaviorState` handles the `SelectionBehavior` widget.
#[derive(Default, AsAny)]
pub struct SelectionBehaviorState {
    target: Entity,
}

impl SelectionBehaviorState {
    fn update_visual_state(&self, ctx: &mut Context) {
        toggle_flag("selected", &mut ctx.get_widget(self.target));
        ctx.get_widget(self.target).update(false);
    }
}

impl State for SelectionBehaviorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.target = (*ctx.widget().get::<u32>("target")).into();
        self.update_visual_state(ctx);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        for action in ctx.messages::<Action>() {
            match action {
                Action::ToggleSelection => {
                    let selected = *SelectionBehavior::selected_ref(&ctx.widget());
                    SelectionBehavior::selected_set(&mut ctx.widget(), !selected);
                }
                Action::UpdateVisualState => self.update_visual_state(ctx),
            }
        }
    }
}

widget!(
    /// The `SelectionBehavior` widget is used to handle internal the pressed behavior of a widget.
    ///
    /// **style:** `check-box`
    SelectionBehavior<SelectionBehaviorState>: MouseHandler {
        /// Sets or shares the target of the behavior.
        target: u32,

        /// Sets or shares the selected property.
        selected: bool,

        /// Sets the parent id.
        parent: u32
    }
);

impl Template for SelectionBehavior {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("SelectionBehavior")
            .selected(true)
            .on_click(move |sender, _| {
                sender.send(Action::ToggleSelection, id);
                false
            })
            .on_changed("selected", move |sender, _| {
                sender.send(Action::UpdateVisualState, id);
            })
    }
}
