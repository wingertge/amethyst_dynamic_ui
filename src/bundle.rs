use crate::{
    prefab::DynamicUiLoaderSystemDesc,
    resize_system::ResizeSystemDesc,
    retrigger::UiButtonTintRetriggerSystemDesc,
    tint::UiButtonTintSystemDesc,
    NoCustomElements, ToLayoutElement
};
#[cfg(feature = "fmod")]
use crate::sound::{UiFmodRetriggerSystemDesc, UiFmodSystemDesc};
use amethyst::{
    core::{
        shred::{DispatcherBuilder, World},
        SystemBundle
    },
    prelude::SystemDesc,
    Error
};
use derive_new::new;
use std::marker::PhantomData;

#[derive(new, Default)]
pub struct DynamicUiBundle<C = NoCustomElements> {
    #[new(default)]
    _phantom_data: PhantomData<C>
}

impl<'a, 'b, C: ToLayoutElement> SystemBundle<'a, 'b> for DynamicUiBundle<C> {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>
    ) -> Result<(), Error> {
        builder.add(
            DynamicUiLoaderSystemDesc::<C::PrefabData>::default().build(world),
            "dynamic_ui_loader",
            &[]
        );
        builder.add(
            ResizeSystemDesc::default().build(world),
            "dynamic_ui_resize_system",
            &[]
        );
        builder.add(
            UiButtonTintSystemDesc::default().build(world),
            "ui_button_tint_system",
            &["ui_mouse_system"]
        );
        builder.add(
            UiButtonTintRetriggerSystemDesc::default().build(world),
            "ui_button_tint_retrigger_system",
            &["ui_button_tint_system"]
        );
        #[cfg(feature = "fmod")]
        {
            builder.add(
                UiFmodSystemDesc::default().build(world),
                "ui_fmod_system",
                &[]
            );
            builder.add(
                UiFmodRetriggerSystemDesc::default().build(world),
                "ui_fmod_retrigger_system",
                &["ui_fmod_system"]
            );
        }

        Ok(())
    }
}
