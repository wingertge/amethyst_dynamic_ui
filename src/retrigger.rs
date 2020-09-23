use amethyst::{
    core::ecs::{Component, DenseVecStorage, Entity},
    renderer::resources::Tint,
    ui::{EventReceiver, EventRetrigger, EventRetriggerSystemDesc, UiEvent, UiEventType}
};

pub type UiButtonTintRetriggerSystemDesc = EventRetriggerSystemDesc<UiButtonTintRetrigger>;

#[derive(Clone, Debug)]
pub struct UiButtonTintAction {
    pub target: Entity,
    pub event_type: UiButtonTintActionType
}

#[derive(Clone, Debug)]
pub enum UiButtonTintActionType {
    SetTint(Tint),
    UnsetTint(Tint)
}

pub struct UiButtonTintRetrigger {
    pub on_click_start: Option<UiButtonTintAction>,
    pub on_click_stop: Option<UiButtonTintAction>,
    pub on_hover_start: Option<UiButtonTintAction>,
    pub on_hover_stop: Option<UiButtonTintAction>
}

impl Component for UiButtonTintRetrigger {
    type Storage = DenseVecStorage<Self>;
}

impl EventRetrigger for UiButtonTintRetrigger {
    type In = UiEvent;
    type Out = UiButtonTintAction;

    fn apply<R>(&self, event: &Self::In, out: &mut R)
    where
        R: EventReceiver<Self::Out>
    {
        let to_send = match event.event_type {
            UiEventType::ClickStart => &self.on_click_start,
            UiEventType::ClickStop => &self.on_click_stop,
            UiEventType::HoverStart => &self.on_hover_start,
            UiEventType::HoverStop => &self.on_hover_stop,
            _ => &None
        };

        if let Some(to_send) = to_send {
            out.receive_one(to_send);
        }
    }
}
