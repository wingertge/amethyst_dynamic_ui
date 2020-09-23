use crate::{
    retrigger::{UiButtonTintAction, UiButtonTintActionType},
    HashMap
};
use amethyst::{
    core::ecs::{Entity, ReaderId, System, Write, WriteStorage},
    derive::SystemDesc,
    ecs::SystemData,
    renderer::resources::Tint,
    shrev::EventChannel
};
use std::fmt::Debug;

#[derive(Debug)]
struct ActionChangeStack<T: Debug + Clone + PartialEq> {
    initial_value: T,
    stack: Vec<T>
}

impl<T> ActionChangeStack<T>
where
    T: Debug + Clone + PartialEq
{
    pub fn new(initial_value: T) -> Self {
        ActionChangeStack {
            initial_value,
            stack: Vec::new()
        }
    }

    pub fn add(&mut self, change: T) {
        self.stack.push(change);
    }

    pub fn remove(&mut self, change: &T) -> Option<T> {
        if let Some(idx) = self.stack.iter().position(|it| it == change) {
            Some(self.stack.remove(idx))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn current(&self) -> T {
        if self.stack.is_empty() {
            self.initial_value.clone()
        } else {
            self.stack
                .iter()
                .last()
                .map(T::clone)
                .expect("Unreachable: Just checked that stack is not empty")
        }
    }
}

#[derive(Debug, SystemDesc)]
#[system_desc(name(UiButtonTintSystemDesc))]
pub struct UiButtonTintSystem {
    #[system_desc(event_channel_reader)]
    event_reader: ReaderId<UiButtonTintAction>,
    #[system_desc(skip)]
    set_tints: HashMap<Entity, ActionChangeStack<Tint>>
}

impl UiButtonTintSystem {
    pub fn new(event_reader: ReaderId<UiButtonTintAction>) -> Self {
        Self {
            event_reader,
            set_tints: Default::default()
        }
    }
}

impl<'s> System<'s> for UiButtonTintSystem {
    type SystemData = (
        WriteStorage<'s, Tint>,
        Write<'s, EventChannel<UiButtonTintAction>>
    );

    fn run(&mut self, (ref mut tints, ref mut events): Self::SystemData) {
        for event in events.read(&mut self.event_reader) {
            match event.event_type {
                UiButtonTintActionType::SetTint(set_tint) => {
                    if let Some(tint) = tints.get_mut(event.target) {
                        self.set_tints
                            .entry(event.target)
                            .or_insert_with(|| ActionChangeStack::new(*tint))
                            .add(set_tint);

                        *tint = set_tint;
                    }
                }
                UiButtonTintActionType::UnsetTint(ref unset_tint) => {
                    if let Some(tint) = tints.get_mut(event.target) {
                        if !self.set_tints.contains_key(&event.target) {
                            continue;
                        }

                        self.set_tints
                            .get_mut(&event.target)
                            .and_then(|it| it.remove(unset_tint));

                        *tint = self.set_tints[&event.target].current();

                        if self.set_tints[&event.target].is_empty() {
                            self.set_tints.remove(&event.target);
                        }
                    }
                }
            }
        }
    }
}
