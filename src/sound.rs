use amethyst_fmod::{AudioSystem};
use amethyst::{
    core::ecs::{Component, DenseVecStorage, ReadExpect, System, SystemData, Write},
    derive::SystemDesc,
    shrev::{EventChannel, ReaderId},
    ui::{
        EventReceiver, EventRetrigger, EventRetriggerSystem, EventRetriggerSystemDesc, UiEvent,
        UiEventType::*
    }
};
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SoundEvent {
    Oneshot(String),
    Loop(String)
}

pub type UiFmodRetriggerSystemDesc = EventRetriggerSystemDesc<UiFmodRetrigger>;

pub type UiFmodRetriggerSystem = EventRetriggerSystem<UiFmodRetrigger>;

#[derive(Clone, Debug)]
pub struct UiFmodRetrigger {
    pub on_click_start: Option<SoundEvent>,
    pub on_click_stop: Option<SoundEvent>,
    pub on_hover_start: Option<SoundEvent>,
    pub on_hover_stop: Option<SoundEvent>
}

impl Component for UiFmodRetrigger {
    type Storage = DenseVecStorage<Self>;
}

impl EventRetrigger for UiFmodRetrigger {
    type In = UiEvent;
    type Out = SoundEvent;

    fn apply<R>(&self, event: &Self::In, out: &mut R)
    where
        R: EventReceiver<Self::Out>
    {
        let event_to_trigger = match &event.event_type {
            ClickStart => &self.on_click_start,
            ClickStop => &self.on_click_stop,
            HoverStart => &self.on_hover_start,
            HoverStop => &self.on_hover_stop,
            _ => return
        };

        if let Some(ev) = event_to_trigger {
            out.receive_one(ev);
        }
    }
}

#[derive(Debug, SystemDesc)]
#[system_desc(name(UiFmodSystemDesc))]
pub struct UiFmodSystem {
    #[system_desc(event_channel_reader)]
    event_reader: ReaderId<SoundEvent>
}

impl UiFmodSystem {
    pub fn new(event_reader: ReaderId<SoundEvent>) -> Self {
        Self { event_reader }
    }
}

impl<'s> System<'s> for UiFmodSystem {
    type SystemData = (
        Write<'s, EventChannel<SoundEvent>>,
        ReadExpect<'s, AudioSystem>
    );

    fn run(&mut self, (sound_events, audio): Self::SystemData) {
        for event in sound_events.read(&mut self.event_reader) {
            let name = match event {
                SoundEvent::Oneshot(name) => name,
                SoundEvent::Loop(name) => name
            };
            audio.play_simple(name).unwrap();
        }
    }
}
