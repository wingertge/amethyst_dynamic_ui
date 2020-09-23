use crate::{
    image_button_to_ui_button,
    layout::{Layout, LayoutId},
    retrigger::{UiButtonTintAction, UiButtonTintActionType, UiButtonTintRetrigger},
    solver::LimnSolver,
    LayoutElement, ModalData, NoCustomElements, ToLayoutElement,
    HashMap
};
use amethyst::{
    assets::{
        AssetStorage, Format, Handle, Loader, PrefabData, PrefabLoaderSystem,
        PrefabLoaderSystemDesc, Progress
    },
    core::ecs::{shred::ResourceId, Component, DenseVecStorage, Entity, Read, WriteStorage},
    ecs::{ReadExpect, SystemData, World},
    error::{format_err, ResultExt},
    renderer::{resources::Tint, Transparent},
    ui::{
        Anchor, NoCustomUi, Stretch, ToNativeWidget, UiButtonData, UiImagePrefab, UiPrefab,
        UiTextData, UiTransformData
    },
    window::ScreenDimensions,
    Error
};
use derivative::Derivative;
use serde::{export::PhantomData, Deserialize, Serialize};
use std::{
    ops::{Deref, Sub},
    sync::{Arc, Mutex}
};
#[cfg(feature = "fmod")]
use crate::sound::{SoundEvent, UiFmodRetrigger};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ExtraButtonData {
    normal_tint: Option<Tinted>,
    hover_tint: Option<Tint>,
    click_tint: Option<Tint>,
    #[cfg(feature = "fmod")]
    click_sound: Option<SoundEvent>,
    #[cfg(feature = "fmod")]
    hover_sound: Option<SoundEvent>,
    #[cfg(feature = "fmod")]
    release_sound: Option<SoundEvent>
}

#[cfg(not(feature = "fmod"))]
type ButtonSystemData<'a> = WriteStorage<'a, UiButtonTintRetrigger>;

#[cfg(feature = "fmod")]
type ButtonSystemData<'a> = (
    WriteStorage<'a, UiButtonTintRetrigger>,
    WriteStorage<'a, UiFmodRetrigger>
);

impl<'a> PrefabData<'a> for ExtraButtonData {
    type SystemData = ButtonSystemData<'a>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        data: &mut Self::SystemData,
        _: &[Entity],
        _: &[Entity]
    ) -> Result<Self::Result, Error> {
        #[cfg(not(feature = "fmod"))]
        let tint_retriggers = data;
        #[cfg(feature = "fmod")]
        let (ref mut tint_retriggers, ref mut sound_retriggers) = data;

        let mut on_click_start = None;
        let mut on_click_stop = None;
        let mut on_hover_start = None;
        let mut on_hover_stop = None;

        if let Some(hover_tint) = self.hover_tint {
            on_hover_start.replace(UiButtonTintAction {
                target: entity,
                event_type: UiButtonTintActionType::SetTint(hover_tint)
            });

            on_hover_stop.replace(UiButtonTintAction {
                target: entity,
                event_type: UiButtonTintActionType::UnsetTint(hover_tint)
            });
        }

        if let Some(click_tint) = self.click_tint {
            on_click_start.replace(UiButtonTintAction {
                target: entity,
                event_type: UiButtonTintActionType::SetTint(click_tint)
            });

            on_click_stop.replace(UiButtonTintAction {
                target: entity,
                event_type: UiButtonTintActionType::UnsetTint(click_tint)
            });
        }

        if on_click_start.is_some()
            || on_click_stop.is_some()
            || on_hover_start.is_some()
            || on_hover_stop.is_some()
        {
            let retrigger = UiButtonTintRetrigger {
                on_click_start,
                on_click_stop,
                on_hover_start,
                on_hover_stop
            };
            tint_retriggers.insert(entity, retrigger)?;
        }

        #[cfg(feature = "fmod")]
        if self.hover_sound.is_some() || self.click_sound.is_some() || self.release_sound.is_some()
        {
            let retrigger = UiFmodRetrigger {
                on_click_start: self.click_sound.clone(),
                on_click_stop: self.release_sound.clone(),
                on_hover_start: self.hover_sound.clone(),
                on_hover_stop: None
            };

            sound_retriggers.insert(entity, retrigger)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct CalculatedDimensions {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32
}

impl Sub for &CalculatedDimensions {
    type Output = CalculatedDimensions;

    fn sub(self, rhs: Self) -> Self::Output {
        CalculatedDimensions {
            left: self.left - rhs.left,
            top: self.top - rhs.top,
            width: self.width,
            height: self.height
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct LayoutIdentifier(pub LayoutId);

impl Deref for LayoutIdentifier {
    type Target = LayoutId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Component for LayoutIdentifier {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> PrefabData<'a> for LayoutIdentifier {
    type SystemData = WriteStorage<'a, LayoutIdentifier>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        layout_ids: &mut Self::SystemData,
        _: &[Entity],
        _: &[Entity]
    ) -> Result<Self::Result, Error> {
        layout_ids.insert(entity, *self)?;
        Ok(())
    }
}

pub type CustomData<C = <NoCustomElements as ToLayoutElement>::PrefabData> = (
    Option<DynamicLayout>,
    Option<LayoutIdentifier>,
    Option<ExtraButtonData>,
    Option<Tinted>,
    C
);

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug)]
#[serde(transparent)]
pub struct Tinted(pub Tint);

impl<'a> PrefabData<'a> for Tinted {
    type SystemData = (WriteStorage<'a, Tint>, WriteStorage<'a, Transparent>);
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (tints, transparent): &mut Self::SystemData,
        _: &[Entity],
        _: &[Entity]
    ) -> Result<Self::Result, Error> {
        tints.insert(entity, self.0)?;
        transparent.insert(entity, Transparent)?;
        Ok(())
    }
}

fn transform(
    id: String,
    dims: Option<&CalculatedDimensions>,
    parent: Option<&CalculatedDimensions>
) -> UiTransformData<()> {
    let mut transform = UiTransformData::default();
    if let Some(dims) = dims {
        let dims = if let Some(parent) = parent {
            dims - parent
        } else {
            dims.clone()
        };

        transform.id = id;
        transform.x = dims.left;
        transform.y = -dims.top;
        transform.width = dims.width;
        transform.height = dims.height;
        transform.anchor = Anchor::TopLeft;
        transform.pivot = Anchor::TopLeft;
    }
    transform
}

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""), Default(bound = ""))]
pub struct DynamicUiFormat<C> {
    width: f32,
    height: f32,
    dpi: f32,
    _phantom_data: PhantomData<C>
}

unsafe impl<C> Send for DynamicUiFormat<C> {}
unsafe impl<C> Sync for DynamicUiFormat<C> {}

impl<C> Format<UiPrefab<CustomData<C::PrefabData>, u32>> for DynamicUiFormat<C>
where
    C: ToLayoutElement + for<'de> Deserialize<'de>
{
    fn name(&self) -> &'static str {
        "Dynamic Ui"
    }

    fn import_simple(
        &self,
        bytes: Vec<u8>
    ) -> Result<UiPrefab<CustomData<C::PrefabData>, u32>, amethyst::Error> {
        use ron::de::Deserializer;
        let mut d = Deserializer::from_bytes(&bytes)
            .with_context(|_| format_err!("Failed deserializing Ron file"))?;
        let mut root: LayoutElement<C> = LayoutElement::deserialize(&mut d)
            .with_context(|_| format_err!("Failed parsing Ron file"))?;
        d.end()
            .with_context(|_| format_err!("Failed parsing Ron file"))?;

        let layout = root.create_solver((self.width, self.height), self.dpi);
        let layout_cloned = layout.0.lock().unwrap();
        // The solution should be discarded the first time, since we don't set screen dimensions yet

        let mut solution_index = 0;
        let mut prefab = UiPrefab::new();
        walk_ui_tree(
            root,
            0,
            &mut solution_index,
            &mut prefab,
            Some(layout.clone()),
            &layout_cloned.solutions,
            self.dpi,
            (self.width, self.height),
            None,
            Default::default()
        );

        Ok(prefab)
    }
}

fn walk_ui_tree<C: ToLayoutElement>(
    widget: LayoutElement<C>,
    entity_index: usize,
    solution_index: &mut usize,
    prefab: &mut UiPrefab<CustomData<C::PrefabData>>,
    layout: Option<DynamicLayout>,
    solution: &HashMap<usize, CalculatedDimensions>,
    dpi: f32,
    (width, height): (f32, f32),
    parent: Option<&CalculatedDimensions>,
    custom_data: C::PrefabData
) {
    match widget {
        LayoutElement::Image { id, image, .. } => {
            let transform = transform(id, solution.get(solution_index), parent);
            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data((
                    Some(transform),
                    Some(image),
                    None,
                    None,
                    (
                        layout,
                        Some(LayoutIdentifier(*solution_index)),
                        None,
                        None,
                        custom_data
                    )
                ));
        }
        LayoutElement::Label { id, mut text, .. } => {
            let transform = transform(id, solution.get(solution_index), parent);
            text.font_size *= dpi;
            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data((
                    Some(transform),
                    None,
                    Some(text),
                    None,
                    (
                        layout,
                        Some(LayoutIdentifier(*solution_index)),
                        None,
                        None,
                        custom_data
                    )
                ));
        }
        LayoutElement::Linear {
            id,
            background,
            children,
            tint,
            ..
        }
        | LayoutElement::Grid {
            id,
            background,
            children,
            tint,
            ..
        } => {
            let container_solution = solution.get(solution_index);
            let transform = transform(id, container_solution, parent);

            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data((
                    Some(transform),
                    background,
                    None,
                    None,
                    (
                        layout,
                        Some(LayoutIdentifier(*solution_index)),
                        None,
                        tint,
                        custom_data
                    )
                ));

            *solution_index += 1; // padding placeholder

            for child_widget in children {
                let child_index = prefab.add(Some(entity_index), None);
                *solution_index += 1;
                walk_ui_tree(
                    child_widget,
                    child_index,
                    solution_index,
                    prefab,
                    None,
                    solution,
                    dpi,
                    (width, height),
                    container_solution,
                    Default::default()
                );
            }
        }
        LayoutElement::Canvas {
            id,
            background,
            tint,
            ..
        } => {
            let transform = transform(id, solution.get(solution_index), parent);

            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data((
                    Some(transform),
                    background,
                    None,
                    None,
                    (
                        layout,
                        Some(LayoutIdentifier(*solution_index)),
                        None,
                        tint,
                        custom_data
                    )
                ));
        }
        LayoutElement::Button {
            id,
            mut button,
            extra_button,
            ..
        } => {
            let transform = transform(id, solution.get(solution_index), parent);
            button.font_size *= dpi;

            let id = transform.id.clone();
            let text = UiTextData {
                color: button.normal_text_color,
                editable: None,
                font: button.font.clone(),
                password: false,
                align: None,
                line_mode: None,
                text: button.text.clone(),
                font_size: button.font_size
            };
            let tint = extra_button
                .as_ref()
                .and_then(|button| button.normal_tint.as_ref())
                .copied()
                .unwrap_or_else(Default::default);

            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data((
                    Some(transform),
                    button.normal_image.take().map(UiImagePrefab),
                    None,
                    Some(button),
                    (
                        layout,
                        Some(LayoutIdentifier(*solution_index)),
                        extra_button,
                        Some(tint),
                        custom_data
                    )
                ));

            // Text isn't in the layout solver, don't pass a layout ID
            prefab.add(
                Some(entity_index),
                Some((
                    Some(button_text_transform(id)),
                    None,
                    Some(text),
                    None,
                    (None, None, None, None, Default::default())
                ))
            );
        }
        LayoutElement::ImageButton {
            id,
            mut image_button,
            extra_button,
            ..
        } => {
            let transform = transform(id, solution.get(solution_index), parent);
            let id = transform.id.clone();

            let tint = extra_button
                .as_ref()
                .and_then(|button| button.normal_tint.as_ref())
                .copied()
                .unwrap_or_else(Default::default);

            let mut button = image_button_to_ui_button(&mut image_button);

            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data((
                    Some(transform),
                    button.normal_image.take().map(UiImagePrefab),
                    None,
                    None,
                    (
                        layout,
                        Some(LayoutIdentifier(*solution_index)),
                        extra_button,
                        Some(tint),
                        custom_data
                    )
                ));

            prefab.add(
                Some(entity_index),
                Some((
                    Some(button_icon_transform(
                        id,
                        image_button.margin.take().unwrap_or(8.0)
                    )),
                    image_button.icon.take().map(UiImagePrefab),
                    None,
                    None,
                    (None, None, None, None, Default::default())
                ))
            );
        }
        LayoutElement::Padded { inner, .. } => {
            *solution_index += 1;
            walk_ui_tree(
                *inner,
                entity_index,
                solution_index,
                prefab,
                layout,
                solution,
                dpi,
                (width, height),
                parent,
                Default::default()
            );
        }
        LayoutElement::Modal {
            id,
            options,
            header,
            content,
            ..
        } => {
            let options = options.unwrap_or_else(ModalData::default);
            let background_solution = solution.get(solution_index);

            let mut background_transform =
                transform(format!("{}_background", id), background_solution, parent);
            background_transform.z = 100.0;

            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data((
                    Some(background_transform),
                    Some(options.background),
                    None,
                    None,
                    (layout, None, None, None, custom_data)
                ));

            *solution_index += 1;

            let container_solution = solution.get(solution_index);
            let container_transform = transform(id, container_solution, background_solution);
            let container = prefab.add(
                Some(entity_index),
                Some((
                    Some(container_transform),
                    None,
                    None,
                    None,
                    (
                        None,
                        Some(LayoutIdentifier(*solution_index)),
                        None,
                        None,
                        Default::default()
                    )
                ))
            );

            *solution_index += 1;
            let header_index = prefab.add(Some(container), None);
            walk_ui_tree(
                *header,
                header_index,
                solution_index,
                prefab,
                None,
                solution,
                dpi,
                (width, height),
                container_solution,
                Default::default()
            );

            *solution_index += 1;
            let content_index = prefab.add(Some(container), None);
            walk_ui_tree(
                *content,
                content_index,
                solution_index,
                prefab,
                None,
                solution,
                dpi,
                (width, height),
                container_solution,
                Default::default()
            );
        }
        LayoutElement::Custom(custom) => {
            let (widget, custom_data) = custom.into_native_widget(custom_data);
            walk_ui_tree(
                widget,
                entity_index,
                solution_index,
                prefab,
                layout,
                solution,
                dpi,
                (width, height),
                parent,
                custom_data
            );
        }
    }
}

fn button_text_transform<G>(mut id: String) -> UiTransformData<G> {
    id.push_str("_btn_txt");
    UiTransformData::default()
        .with_id(id)
        .with_position(0., 0., 1.)
        .with_anchor(Anchor::Middle)
        .with_stretch(Stretch::XY {
            x_margin: 0.,
            y_margin: 0.,
            keep_aspect_ratio: false
        })
        .transparent()
}

fn button_icon_transform<G>(mut id: String, margin: f32) -> UiTransformData<G> {
    id.push_str("_btn_icon");
    UiTransformData::default()
        .with_id(id)
        .with_position(0.0, 0.0, 1.0)
        .with_anchor(Anchor::Middle)
        .with_stretch(Stretch::XY {
            x_margin: margin,
            y_margin: margin,
            keep_aspect_ratio: false
        })
        .transparent()
}

#[derive(Clone)]
pub struct DynamicLayout(pub Arc<Mutex<DynamicLayoutData>>);

pub struct DynamicLayoutData {
    pub solver: LimnSolver,
    pub root: Layout,
    pub solutions: HashMap<usize, CalculatedDimensions>
}

unsafe impl Send for DynamicLayout {}
unsafe impl Sync for DynamicLayout {}

impl Component for DynamicLayout {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> PrefabData<'a> for DynamicLayout {
    type SystemData = WriteStorage<'a, DynamicLayout>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        layouts: &mut Self::SystemData,
        _: &[Entity],
        _: &[Entity]
    ) -> Result<Self::Result, Error> {
        layouts.insert(entity, self.clone())?;
        Ok(())
    }
}

#[derive(SystemData)]
pub struct DynamicUiLoader<'a, C: ToLayoutElement = NoCustomElements> {
    loader: ReadExpect<'a, Loader>,
    storage: Read<'a, AssetStorage<UiPrefab<CustomData<C::PrefabData>, u32>>>,
    screen_dimensions: ReadExpect<'a, ScreenDimensions>
}

impl<'a, C> DynamicUiLoader<'a, C>
where
    C: ToLayoutElement + for<'de> Deserialize<'de>
{
    pub fn load<N: Into<String>, P: Progress>(
        &self,
        name: N,
        progress: P
    ) -> Handle<UiPrefab<CustomData<C::PrefabData>, u32>> {
        let format = DynamicUiFormat::<C> {
            width: self.screen_dimensions.width(),
            height: self.screen_dimensions.height(),
            dpi: self.screen_dimensions.hidpi_factor() as f32,
            _phantom_data: PhantomData::default()
        };
        self.loader.load(name, format, progress, &*self.storage)
    }
}

pub type DynamicUiLoaderSystemDesc<C = <NoCustomElements as ToLayoutElement>::PrefabData> =
    PrefabLoaderSystemDesc<UiPrefabData<CustomData<C>>>;

pub type DynamicUiLoaderSystem<C = <NoCustomElements as ToLayoutElement>::PrefabData> =
    PrefabLoaderSystem<UiPrefabData<CustomData<C>>>;

type UiPrefabData<D = <NoCustomUi as ToNativeWidget>::PrefabData, W = u32, G = ()> = (
    Option<UiTransformData<G>>,
    Option<UiImagePrefab>,
    Option<UiTextData>,
    Option<UiButtonData<W>>,
    D
);
