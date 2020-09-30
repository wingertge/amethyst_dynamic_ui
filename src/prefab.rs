#[cfg(feature = "fmod")]
use crate::sound::{SoundEvent, UiFmodRetrigger};
use crate::{
    assets::{Fonts, Textures},
    image_button_to_ui_button,
    layout::{Layout, LayoutId},
    retrigger::{UiButtonTintAction, UiButtonTintActionType, UiButtonTintRetrigger},
    solver::LimnSolver,
    styling::{FontFamily, HtmlText, Styles},
    HashMap, LayoutElement, ModalData, NoCustomElements, Properties, ToLayoutElement
};
use amethyst::{
    assets::{
        Asset, AssetPrefab, AssetStorage, Format, Handle, Loader, Prefab, PrefabData,
        PrefabLoaderSystem, PrefabLoaderSystemDesc, Progress, ProgressCounter
    },
    core::ecs::{shred::ResourceId, Component, DenseVecStorage, Entity, Read, WriteStorage},
    ecs::{ReadExpect, SystemData, World},
    error::{format_err, ResultExt},
    renderer::{resources::Tint, TexturePrefab, Transparent},
    ui::{
        get_default_font, Anchor, FontAsset, LineMode, Stretch, UiButtonData, UiImageLoadPrefab,
        UiImagePrefab, UiMultipartText, UiTextData, UiTransformData
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

/*impl<'a> PrefabData<'a> for LayoutElement {
    type SystemData = (
        (Read<'a, Textures>, <UiImagePrefab as PrefabData<'a>>::SystemData),
        (Read<'a, Fonts>, <UiTextData as PrefabData<'a>>::SystemData)
    );
    type Result = ();

    fn add_to_entity(&self, entity: Entity, system_data: &mut Self::SystemData, entities: &[Entity], children: &[Entity]) -> Result<Self::Result, Error> {
        unimplemented!()
    }

    fn load_sub_assets(&mut self, progress: &mut ProgressCounter, system_data: &mut Self::SystemData) -> Result<bool, Error> {
        let (image_data, text_data) = system_data;
        match self {
            LayoutElement::Image { image, .. } => {
                image.load_sub_assets(progress, image_data)
            }
            LayoutElement::Button { button, .. } => {
                let UiButtonData {
                    font, normal_image, hover_image, press_image, ..
                } = button;
                let mut text = UiCachedText(UiTextData {
                    color: [0.0, 0.0, 0.0, 0.0],
                    editable: None,
                    font: font.take(),
                    password: false,
                    align: None,
                    line_mode: None,
                    text: String::new(),
                    font_size: 0.0
                });
                let mut needs_loading = text.load_sub_assets(progress, text_data)?;
                *font = text.0.font;
                if let Some(image) = normal_image.take() {
                    let mut cached = UiCachedImage(UiImagePrefab(image));
                    needs_loading = cached.load_sub_assets(progress, image_data)? || needs_loading;
                    *normal_image = Some((cached.0).0);
                }
                if let Some(image) = hover_image.take() {
                    let mut cached = UiCachedImage(UiImagePrefab(image));
                    needs_loading = cached.load_sub_assets(progress, image_data)? || needs_loading;
                    *hover_image = Some((cached.0).0);
                }
                if let Some(image) = press_image.take() {
                    let mut cached = UiCachedImage(UiImagePrefab(image));
                    needs_loading = cached.load_sub_assets(progress, image_data)? || needs_loading;
                    *press_image = Some((cached.0).0);
                }
                Ok(needs_loading)
            }
            LayoutElement::ImageButton { image_button, .. } => {
                let ImageButtonData {
                    icon, normal_image, hover_image, press_image, ..
                } = image_button;
                let mut needs_loading = icon.load_sub_assets(progress, image_data)?;
                if let Some(normal_image) = normal_image {
                    needs_loading = normal_image.load_sub_assets(progress, image_data)? || needs_loading;
                }
                if let Some(hover_image) = hover_image {
                    needs_loading = hover_image.load_sub_assets(progress, image_data)? || needs_loading;
                }
                if let Some(press_image) = press_image {
                    needs_loading = press_image.load_sub_assets(progress, image_data)? || needs_loading;
                }
                Ok(needs_loading)
            }
            LayoutElement::Label { text, .. } => {
                text.load_sub_assets(progress, text_data)
            }
            LayoutElement::Linear { background, children, .. } |
            LayoutElement::Grid { background, children, .. } => {
                let mut res = background.load_sub_assets(progress, image_data)?;
                for child in children {
                    res = child.load_sub_assets(progress, system_data)? || res;
                }
                Ok(res)
            }
            LayoutElement::Padded { inner, .. } => {
                inner.load_sub_assets(progress, image_data)
            }
            LayoutElement::Modal { header, content, options, .. } => {
                let mut res = header.load_sub_assets(progress, system_data)?;
                res = content.load_sub_assets(progress, system_data)? || res;
                if let Some(options) = options {
                    res = options.background.load_sub_assets(progress, image_data)? || res;
                }
                Ok(res)
            }
            LayoutElement::Canvas { background, .. } => {
                if let Some(background) = background {
                    background.load_sub_assets(progress, image_data)
                } else {
                    Ok(false)
                }
            }
            LayoutElement::Custom(_) => Ok(false)
        }
    }
}*/

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ExtraButtonData {
    pub normal_tint: Option<Tinted>,
    pub hover_tint: Option<Tint>,
    pub click_tint: Option<Tint>,
    #[cfg_attr(feature = "document", doc(cfg(feature = "fmod")))]
    #[cfg(feature = "fmod")]
    pub click_sound: Option<SoundEvent>,
    #[cfg_attr(feature = "document", doc(cfg(feature = "fmod")))]
    #[cfg(feature = "fmod")]
    pub hover_sound: Option<SoundEvent>,
    #[cfg_attr(feature = "document", doc(cfg(feature = "fmod")))]
    #[cfg(feature = "fmod")]
    pub release_sound: Option<SoundEvent>
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UiCachedImage(pub UiImagePrefab);

impl Deref for UiCachedImage {
    type Target = UiImagePrefab;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> PrefabData<'a> for UiCachedImage {
    type SystemData = (
        Read<'a, Textures>,
        <UiImagePrefab as PrefabData<'a>>::SystemData
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (_, inner_data): &mut Self::SystemData,
        entities: &[Entity],
        children: &[Entity]
    ) -> Result<Self::Result, Error> {
        self.0.add_to_entity(entity, inner_data, entities, children)
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        (cache, inner_data): &mut Self::SystemData
    ) -> Result<bool, Error> {
        match &mut (&mut self.0).0 {
            UiImageLoadPrefab::Texture(tex)
            | UiImageLoadPrefab::PartialTexture { tex, .. }
            | UiImageLoadPrefab::NineSlice { tex, .. } => {
                let res;
                let name = match tex {
                    TexturePrefab::File(name, _) => {
                        let handle = {
                            let cache = cache.read();
                            cache.get(name).cloned()
                        };
                        if let Some(handle) = handle {
                            *tex = TexturePrefab::Handle(handle);
                            None
                        } else {
                            Some(name.clone())
                        }
                    }
                    _ => None
                };
                if let Some(name) = name {
                    let mut cache = cache.write();
                    res = tex.load_sub_assets(progress, &mut inner_data.0)?;
                    let new_handle = match tex {
                        TexturePrefab::Handle(handle) => handle.clone(),
                        _ => unreachable!()
                    };
                    cache.insert(name, new_handle);
                } else {
                    res = tex.load_sub_assets(progress, &mut inner_data.0)?;
                }
                Ok(res)
            }
            UiImageLoadPrefab::SolidColor(..) => Ok(false)
        }
    }
}

impl<'a> PrefabData<'a> for UiTextPrefab {
    type SystemData = (
        (Read<'a, Fonts>, <UiTextData as PrefabData<'a>>::SystemData),
        WriteStorage<'a, UiMultipartText>,
        WriteStorage<'a, HtmlText>,
        Read<'a, Styles>
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        entities: &[Entity],
        children: &[Entity]
    ) -> Result<Self::Result, Error> {
        match self {
            UiTextPrefab::Regular(cached_text) => {
                cached_text.add_to_entity(entity, &mut system_data.0, entities, children)
            }
            UiTextPrefab::Html(html_text) => {
                html_text.add_to_entity(entity, system_data, entities, children)
            }
        }
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        system_data: &mut Self::SystemData
    ) -> Result<bool, Error> {
        match self {
            UiTextPrefab::Regular(cached_text) => {
                cached_text.load_sub_assets(progress, &mut system_data.0)
            }
            UiTextPrefab::Html(html_text) => html_text.load_sub_assets(progress, system_data)
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct UiCachedText(pub UiTextData);

impl<'a> PrefabData<'a> for UiCachedText {
    type SystemData = (Read<'a, Fonts>, <UiTextData as PrefabData<'a>>::SystemData);
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (_, inner_data): &mut Self::SystemData,
        entities: &[Entity],
        children: &[Entity]
    ) -> Result<Self::Result, Error> {
        self.0.add_to_entity(entity, inner_data, entities, children)
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        (ref cache, ref mut inner): &mut Self::SystemData
    ) -> Result<bool, Error> {
        if let Some(font) = &mut self.0.font {
            get_or_load_font(font, progress, cache, &mut inner.2)
        } else {
            self.0.load_sub_assets(progress, inner)
        }
    }
}

pub struct FontFamilyPrefab {
    pub regular: Option<AssetPrefab<FontAsset>>,
    pub bold: Option<AssetPrefab<FontAsset>>,
    pub italic: Option<AssetPrefab<FontAsset>>,
    pub bold_italic: Option<AssetPrefab<FontAsset>>
}

type FontSystemData<'a> = <AssetPrefab<FontAsset> as PrefabData<'a>>::SystemData;

fn get_or_load_font(
    font: &mut AssetPrefab<FontAsset>,
    progress: &mut ProgressCounter,
    cache: &Fonts,
    inner: &mut FontSystemData<'_>
) -> Result<bool, Error> {
    let res;
    let name = match font {
        AssetPrefab::File(name, _) => {
            let handle = {
                let cache = cache.read();
                cache.get(name).cloned()
            };
            if let Some(handle) = handle {
                *font = AssetPrefab::Handle(handle);
                None
            } else {
                Some(name.clone())
            }
        }
        _ => None
    };
    if let Some(name) = name {
        let mut cache = cache.write();
        res = font.load_sub_assets(progress, inner)?;
        let new_handle = match font {
            AssetPrefab::Handle(handle) => handle.clone(),
            _ => unreachable!()
        };
        cache.insert(name, new_handle);
    } else {
        res = font.load_sub_assets(progress, inner)?;
    }
    Ok(res)
}

fn handle<A: Asset>(prefab: &AssetPrefab<A>) -> &Handle<A> {
    match prefab {
        AssetPrefab::Handle(handle) => handle,
        _ => panic!("Asset prefab should be a handle")
    }
}

impl<'a> PrefabData<'a> for FontFamilyPrefab {
    type SystemData = (Read<'a, Fonts>, <UiTextData as PrefabData<'a>>::SystemData);
    type Result = FontFamily;

    fn add_to_entity(
        &self,
        _: Entity,
        (_, inner): &mut Self::SystemData,
        _: &[Entity],
        _: &[Entity]
    ) -> Result<Self::Result, Error> {
        let regular = if let Some(AssetPrefab::Handle(handle)) = &self.regular {
            handle.clone()
        } else {
            get_default_font(&(inner.2).0, &(inner.2).2)
        };
        let bold = self.bold.as_ref().map(handle).cloned();
        let italic = self.italic.as_ref().map(handle).cloned();
        let bold_italic = self.bold_italic.as_ref().map(handle).cloned();
        Ok(FontFamily {
            regular,
            bold,
            italic,
            bold_italic
        })
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        (ref cache, ref mut inner): &mut Self::SystemData
    ) -> Result<bool, Error> {
        let inner = &mut inner.2;
        let mut needs_loading = false;
        if let Some(font) = &mut self.regular {
            needs_loading = get_or_load_font(font, progress, cache, inner)?;
        }
        if let Some(font) = &mut self.bold {
            needs_loading = get_or_load_font(font, progress, cache, inner)?;
        }
        if let Some(font) = &mut self.italic {
            needs_loading = get_or_load_font(font, progress, cache, inner)?;
        }
        if let Some(font) = &mut self.bold_italic {
            needs_loading = get_or_load_font(font, progress, cache, inner)?;
        }
        Ok(needs_loading)
    }
}

pub struct HtmlTextData {
    pub text: UiTextData,
    pub fonts: FontFamilyPrefab
}

impl<'a> PrefabData<'a> for HtmlTextData {
    type SystemData = (
        (Read<'a, Fonts>, <UiTextData as PrefabData<'a>>::SystemData),
        WriteStorage<'a, UiMultipartText>,
        WriteStorage<'a, HtmlText>,
        Read<'a, Styles>
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        (ref mut text_data, text_storage, html_storage, ref styles): &mut Self::SystemData,
        entities: &[Entity],
        children: &[Entity]
    ) -> Result<Self::Result, Error> {
        let font = self
            .fonts
            .add_to_entity(entity, text_data, entities, children)?;
        let html = HtmlText::new(font, self.text.color, self.text.font_size);
        let text = html.parse(&self.text.text, &styles, &((text_data.1).2).2);
        let text = UiMultipartText::new(
            text,
            self.text.line_mode.unwrap_or(LineMode::Single),
            self.text.align.unwrap_or(Anchor::Middle)
        );
        text_storage.insert(entity, text)?;
        html_storage.insert(entity, html)?;
        Ok(())
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        (ref mut family_data, ..): &mut Self::SystemData
    ) -> Result<bool, Error> {
        self.fonts.load_sub_assets(progress, family_data)
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

struct DataBuilder<C: Default> {
    transform: UiTransformData<()>,
    button: Option<UiButtonData>,
    layout: Option<DynamicLayout>,
    ident: Option<LayoutIdentifier>,
    extra_button: Option<ExtraButtonData>,
    tint: Option<Tinted>,
    image: Option<UiCachedImage>,
    text: Option<UiTextPrefab>,
    custom_data: Option<C>
}

impl<C: Default> DataBuilder<C> {
    pub fn new(transform: UiTransformData<()>) -> Self {
        Self {
            transform,
            button: None,
            layout: None,
            ident: None,
            extra_button: None,
            tint: None,
            image: None,
            text: None,
            custom_data: None
        }
    }

    pub fn with_button(mut self, button: UiButtonData) -> Self {
        self.button = Some(button);
        self
    }

    pub fn with_layout(mut self, layout: Option<DynamicLayout>) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_ident(mut self, ident: LayoutIdentifier) -> Self {
        self.ident = Some(ident);
        self
    }

    pub fn with_extra_button(mut self, extra_button: Option<ExtraButtonData>) -> Self {
        self.extra_button = extra_button;
        self
    }

    pub fn with_tint(mut self, tint: Option<Tinted>) -> Self {
        self.tint = tint;
        self
    }

    pub fn with_image(mut self, image: Option<UiCachedImage>) -> Self {
        self.image = image;
        self
    }

    pub fn with_text(mut self, text: UiCachedText) -> Self {
        self.text = Some(UiTextPrefab::Regular(text));
        self
    }

    pub fn with_html_text(mut self, html_text: HtmlTextData) -> Self {
        self.text = Some(UiTextPrefab::Html(html_text));
        self
    }

    pub fn with_custom_data(mut self, custom_data: C) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    pub fn build(self) -> UiPrefabData<C> {
        let custom_data = self.custom_data.unwrap_or_else(Default::default);
        (
            Some(self.transform),
            self.button,
            self.layout,
            self.ident,
            self.extra_button,
            self.tint,
            self.image,
            self.text,
            custom_data
        )
    }
}

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
    parent: Option<&CalculatedDimensions>,
    properties: Properties
) -> UiTransformData<()> {
    let Properties {
        opaque,
        transparent_target,
        mouse_reactive,
        hidden,
        selectable
    } = properties;
    let mut transform: UiTransformData<()> = UiTransformData::default();

    transform.id = id;
    transform.anchor = Anchor::TopLeft;
    transform.pivot = Anchor::TopLeft;

    transform.opaque = opaque;
    transform.transparent_target = transparent_target;
    transform.mouse_reactive = mouse_reactive;
    transform.hidden = hidden;
    transform.selectable = selectable;

    if let Some(dims) = dims {
        let dims = if let Some(parent) = parent {
            dims - parent
        } else {
            dims.clone()
        };

        transform.x = dims.left;
        transform.y = -dims.top;
        transform.width = dims.width;
        transform.height = dims.height;
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

type UiPrefab<C> = Prefab<UiPrefabData<C>>;

impl<C> Format<UiPrefab<C::PrefabData>> for DynamicUiFormat<C>
where
    C: ToLayoutElement + for<'de> Deserialize<'de>
{
    fn name(&self) -> &'static str {
        "Dynamic Ui"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<UiPrefab<C::PrefabData>, amethyst::Error> {
        use ron::de::Deserializer;
        let mut d = Deserializer::from_bytes(&bytes)
            .with_context(|_| format_err!("Failed deserializing Ron file"))?;
        let root: LayoutElement<C> = LayoutElement::deserialize(&mut d)
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
    prefab: &mut UiPrefab<C::PrefabData>,
    layout: Option<DynamicLayout>,
    solution: &HashMap<usize, CalculatedDimensions>,
    dpi: f32,
    (width, height): (f32, f32),
    parent: Option<&CalculatedDimensions>,
    custom_data: C::PrefabData
) {
    match widget {
        LayoutElement::Image {
            id,
            image,
            properties,
            ..
        } => {
            let transform = transform(id, solution.get(solution_index), parent, properties);
            let data = DataBuilder::new(transform)
                .with_layout(layout)
                .with_ident(LayoutIdentifier(*solution_index))
                .with_image(Some(image))
                .with_custom_data(custom_data)
                .build();
            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data(data);
        }
        LayoutElement::Label {
            id,
            mut text,
            extra_text,
            properties,
            ..
        } => {
            let transform = transform(id, solution.get(solution_index), parent, properties);
            text.0.font_size *= dpi;
            let data = DataBuilder::new(transform)
                .with_layout(layout)
                .with_ident(LayoutIdentifier(*solution_index))
                .with_custom_data(custom_data);
            let is_html = extra_text
                .as_ref()
                .map(|extra| extra.is_html)
                .unwrap_or(false);
            let data = if is_html {
                let extra_text = extra_text.unwrap_or_else(Default::default);
                let fonts = FontFamilyPrefab {
                    regular: text.0.font.take(),
                    bold: extra_text.font_bold,
                    italic: extra_text.font_italic,
                    bold_italic: extra_text.font_bold_italic
                };
                data.with_html_text(HtmlTextData {
                    fonts,
                    text: text.0
                })
            } else {
                data.with_text(text)
            };
            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data(data.build());
        }
        LayoutElement::Linear {
            id,
            background,
            children,
            tint,
            properties,
            ..
        }
        | LayoutElement::Grid {
            id,
            background,
            children,
            tint,
            properties,
            ..
        } => {
            let container_solution = solution.get(solution_index);
            let transform = transform(id, container_solution, parent, properties);

            let data = DataBuilder::new(transform)
                .with_layout(layout)
                .with_ident(LayoutIdentifier(*solution_index))
                .with_tint(tint)
                .with_image(background)
                .with_custom_data(custom_data)
                .build();
            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data(data);

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
            properties,
            ..
        } => {
            let transform = transform(id, solution.get(solution_index), parent, properties);

            let data = DataBuilder::new(transform)
                .with_layout(layout)
                .with_ident(LayoutIdentifier(*solution_index))
                .with_image(background)
                .with_tint(tint)
                .with_custom_data(custom_data)
                .build();
            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data(data);
        }
        LayoutElement::Button {
            id,
            mut button,
            extra_button,
            mut properties,
            ..
        } => {
            properties.mouse_reactive = true;
            let transform = transform(id, solution.get(solution_index), parent, properties);
            button.font_size *= dpi;

            let id = transform.id.clone();
            let text = UiTextData {
                color: button.normal_text_color,
                editable: None,
                font: button.font.take(),
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
            let image = button
                .normal_image
                .take()
                .map(UiImagePrefab)
                .map(UiCachedImage);

            let data = DataBuilder::new(transform)
                .with_button(button)
                .with_extra_button(extra_button)
                .with_layout(layout)
                .with_ident(LayoutIdentifier(*solution_index))
                .with_tint(Some(tint))
                .with_image(image)
                .with_custom_data(custom_data)
                .build();
            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data(data);

            // Text isn't in the layout solver, don't pass a layout ID
            let data = DataBuilder::new(button_text_transform(id))
                .with_text(UiCachedText(text))
                .build();
            prefab.add(Some(entity_index), Some(data));
        }
        LayoutElement::ImageButton {
            id,
            mut image_button,
            extra_button,
            properties,
            ..
        } => {
            let transform = transform(id, solution.get(solution_index), parent, properties);
            let id = transform.id.clone();

            let tint = extra_button
                .as_ref()
                .and_then(|button| button.normal_tint.as_ref())
                .copied()
                .unwrap_or_else(Default::default);

            let mut button = image_button_to_ui_button(&mut image_button);

            let data = DataBuilder::new(transform)
                .with_layout(layout)
                .with_ident(LayoutIdentifier(*solution_index))
                .with_extra_button(extra_button)
                .with_tint(Some(tint))
                .with_image(
                    button
                        .normal_image
                        .take()
                        .map(UiImagePrefab)
                        .map(UiCachedImage)
                )
                .with_custom_data(custom_data)
                .build();
            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data(data);

            let data = DataBuilder::new(button_icon_transform(
                id,
                image_button.margin.take().unwrap_or(8.0)
            ))
            .with_image(image_button.icon.take())
            .build();
            prefab.add(Some(entity_index), Some(data));
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

            let mut background_transform = transform(
                format!("{}_background", id),
                background_solution,
                parent,
                Default::default()
            );
            background_transform.z = 100.0;

            let data = DataBuilder::new(background_transform)
                .with_layout(layout)
                .with_ident(LayoutIdentifier(*solution_index))
                .with_image(Some(options.background))
                .with_custom_data(custom_data)
                .build();
            prefab
                .entity(entity_index)
                .expect("Unreachable: `Prefab` entity should always be set when walking ui tree")
                .set_data(data);

            *solution_index += 1;

            let container_solution = solution.get(solution_index);
            let container_transform = transform(
                id,
                container_solution,
                background_solution,
                Default::default()
            );
            let data = DataBuilder::new(container_transform)
                .with_ident(LayoutIdentifier(*solution_index))
                .build();
            let container = prefab.add(Some(entity_index), Some(data));

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
    storage: Read<'a, AssetStorage<UiPrefab<C::PrefabData>>>,
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
    ) -> Handle<UiPrefab<C::PrefabData>> {
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
    PrefabLoaderSystemDesc<UiPrefabData<C>>;

pub type DynamicUiLoaderSystem<C = <NoCustomElements as ToLayoutElement>::PrefabData> =
    PrefabLoaderSystem<UiPrefabData<C>>;

type UiPrefabData<C = <NoCustomElements as ToLayoutElement>::PrefabData> = (
    Option<UiTransformData<()>>,
    Option<UiButtonData<u32>>,
    Option<DynamicLayout>,
    Option<LayoutIdentifier>,
    Option<ExtraButtonData>,
    Option<Tinted>,
    Option<UiCachedImage>,
    Option<UiTextPrefab>,
    C
);

pub enum UiTextPrefab {
    Regular(UiCachedText),
    Html(HtmlTextData)
}
