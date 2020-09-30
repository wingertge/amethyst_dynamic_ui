use crate::{
    constraint,
    constraint::Padding,
    layout::{
        centered::CenteredLayout,
        linear::{ItemAlignment, LinearLayout, LinearLayoutData, Spacing},
        ExactFrame, Layout, VarType
    },
    prefab::{CalculatedDimensions, DynamicLayout, DynamicLayoutData, ExtraButtonData, Tinted},
    solver::LimnSolver,
    HashMap, UiCachedImage, UiCachedText
};
use amethyst::{
    assets::{AssetPrefab, PrefabData},
    audio::Source,
    ui::{Anchor, FontAsset, UiButtonData, UiImageLoadPrefab, UiImagePrefab}
};
use cassowary::strength::*;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub trait ToLayoutElement
where
    Self: Sized + 'static
{
    type PrefabData: for<'a> PrefabData<'a> + Default + Send + Sync + 'static;

    fn into_native_widget(self, data: Self::PrefabData) -> (LayoutElement<Self>, Self::PrefabData);
    fn visit_layout(
        &self,
        layout: &mut Layout,
        children: &mut Vec<Layout>,
        dpi: f32,
        layout_index: &mut usize
    );
}

fn true_() -> bool {
    true
}
fn default_modal_anchor() -> Anchor {
    Anchor::Middle
}

fn default_modal_background() -> UiCachedImage {
    UiCachedImage(UiImagePrefab(UiImageLoadPrefab::SolidColor(
        0.0, 0.0, 0.0, 0.627
    )))
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SizeConstraints {
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub preferred_width: Option<f32>,
    pub preferred_height: Option<f32>
}

#[derive(Serialize, Deserialize, Clone, Debug, Derivative)]
#[serde(default)]
#[derivative(Default)]
pub struct Properties {
    #[derivative(Default(value = "true"))]
    pub opaque: bool,
    #[derivative(Default(value = "false"))]
    pub transparent_target: bool,
    pub mouse_reactive: bool,
    pub hidden: bool,
    pub selectable: Option<u32>
}

#[derive(Serialize, Deserialize, Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct ModalData {
    #[serde(default = "default_modal_anchor")]
    #[derivative(Default(value = "Anchor::Middle"))]
    pub align: Anchor,
    #[serde(default = "default_modal_background")]
    #[derivative(Default(value = "default_modal_background()"))]
    pub background: UiCachedImage
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NoCustomElements;

impl ToLayoutElement for NoCustomElements {
    type PrefabData = ();

    fn into_native_widget(self, _: Self::PrefabData) -> (LayoutElement<Self>, Self::PrefabData) {
        unreachable!()
    }

    fn visit_layout(&self, _: &mut Layout, _: &mut Vec<Layout>, _: f32, _: &mut usize) {
        unreachable!()
    }
}

#[derive(Serialize, Deserialize, Clone, Derivative, Default)]
#[derivative(Debug)]
pub struct UiExtraTextData {
    #[serde(default)]
    pub is_html: bool,
    #[derivative(Debug = "ignore")]
    pub font_bold: Option<AssetPrefab<FontAsset>>,
    #[derivative(Debug = "ignore")]
    pub font_italic: Option<AssetPrefab<FontAsset>>,
    #[derivative(Debug = "ignore")]
    pub font_bold_italic: Option<AssetPrefab<FontAsset>>
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ImageButtonData {
    pub icon: Option<UiCachedImage>,
    pub margin: Option<f32>,
    // this `normal_image` is "transplanted" into UiImagePrefab at the top level
    // of ui widget. This happens inside `walk_ui_tree` function. It means that
    // it will always be `None` during `add_to_entity`.
    /// Default image
    pub normal_image: Option<UiCachedImage>,
    /// Image used when the mouse hovers over this element
    pub hover_image: Option<UiCachedImage>,
    /// Image used when button is pressed
    pub press_image: Option<UiCachedImage>,
    /// Sound made when this button is hovered over
    pub hover_sound: Option<AssetPrefab<Source>>,
    /// Sound made when this button is pressed.
    pub press_sound: Option<AssetPrefab<Source>>,
    /// Sound made when this button is released.
    pub release_sound: Option<AssetPrefab<Source>>
}

pub(crate) fn image_button_to_ui_button(button: &mut ImageButtonData) -> UiButtonData {
    UiButtonData {
        id: None,
        text: String::new(),
        font_size: 0.0,
        font: None,
        normal_text_color: [0.0, 0.0, 0.0, 0.0],
        normal_image: button.normal_image.take().map(|a| (a.0).0),
        hover_image: button.hover_image.take().map(|a| (a.0).0),
        hover_text_color: None,
        press_image: button.press_image.take().map(|a| (a.0).0),
        press_text_color: None,
        hover_sound: button.hover_sound.take(),
        press_sound: button.press_sound.take(),
        release_sound: button.release_sound.take()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct I18nData {
    pub context: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum LayoutElement<C: ToLayoutElement = NoCustomElements> {
    Image {
        id: String,
        image: UiCachedImage,
        constraints: Option<SizeConstraints>,
        #[serde(default = "true_")]
        keep_aspect_ratio: bool,
        #[serde(default)]
        properties: Properties
    },
    Button {
        id: String,
        button: UiButtonData<u32>,
        #[serde(default)]
        i18n: I18nData,
        extra_button: Option<ExtraButtonData>,
        constraints: Option<SizeConstraints>,
        #[serde(default)]
        properties: Properties
    },
    ImageButton {
        id: String,
        image_button: ImageButtonData,
        extra_button: Option<ExtraButtonData>,
        constraints: Option<SizeConstraints>,
        #[serde(default)]
        properties: Properties
    },
    Label {
        id: String,
        text: UiCachedText,
        #[serde(default)]
        i18n: I18nData,
        extra_text: Option<UiExtraTextData>,
        constraints: Option<SizeConstraints>,
        #[serde(default)]
        properties: Properties
    },
    Linear {
        id: String,
        background: Option<UiCachedImage>,
        tint: Option<Tinted>,
        constraints: Option<SizeConstraints>,
        options: Option<LinearLayoutData>,
        children: Vec<LayoutElement<C>>,
        padding: Option<Padding>,
        #[serde(default)]
        properties: Properties
    },
    Grid {
        id: String,
        columns: usize,
        background: Option<UiCachedImage>,
        tint: Option<Tinted>,
        constraints: Option<SizeConstraints>,
        children: Vec<LayoutElement<C>>,
        padding: Option<Padding>,
        #[serde(default)]
        properties: Properties
    },
    Padded {
        padding: Padding,
        inner: Box<LayoutElement<C>>
    },
    Modal {
        id: String,
        options: Option<ModalData>,
        constraints: Option<SizeConstraints>,
        header: Box<LayoutElement<C>>,
        content: Box<LayoutElement<C>>
    },
    /// An empty container that can me filled with dynamic content
    Canvas {
        id: String,
        constraints: Option<SizeConstraints>,
        background: Option<UiCachedImage>,
        tint: Option<Tinted>,
        #[serde(default)]
        properties: Properties
    },
    Custom(Box<C>)
}

impl<C: ToLayoutElement> LayoutElement<C> {
    /// Solve layout and transform into widget tree with absolute
    pub fn create_solver(
        &self,
        (screen_width, screen_height): (f32, f32),
        dpi: f32
    ) -> DynamicLayout {
        let mut solver = LimnSolver::new();
        let mut index = 0;
        let mut root_layout = walk_layout_tree(self, &mut solver, dpi, &mut index);

        {
            let left = root_layout.edit_left();
            left.set(0.0);
        }
        {
            let top = root_layout.edit_top();
            top.set(0.0);
        }

        solver.update_layout(&mut root_layout);

        let solution_map = HashMap::default();

        let mut data = DynamicLayoutData {
            solver,
            root: root_layout,
            solutions: solution_map
        };

        solve_layout(&mut data, (screen_width, screen_height));
        data.solver.debug_layouts();

        DynamicLayout(Arc::new(Mutex::new(data)))
    }

    fn id(&self) -> &str {
        match self {
            LayoutElement::Image { id, .. }
            | LayoutElement::Button { id, .. }
            | LayoutElement::Label { id, .. }
            | LayoutElement::Linear { id, .. }
            | LayoutElement::Grid { id, .. }
            | LayoutElement::ImageButton { id, .. }
            | LayoutElement::Canvas { id, .. }
            | LayoutElement::Modal { id, .. } => id,
            LayoutElement::Padded { inner, .. } => inner.id(),
            LayoutElement::Custom(_) => ""
        }
    }

    fn constraints(&self) -> &Option<SizeConstraints> {
        match self {
            LayoutElement::Image { constraints, .. }
            | LayoutElement::Button { constraints, .. }
            | LayoutElement::Label { constraints, .. }
            | LayoutElement::ImageButton { constraints, .. }
            | LayoutElement::Linear { constraints, .. }
            | LayoutElement::Grid { constraints, .. }
            | LayoutElement::Canvas { constraints, .. }
            | LayoutElement::Modal { constraints, .. } => constraints,
            _ => &None
        }
    }
}

fn walk_layout_tree<C: ToLayoutElement>(
    element: &LayoutElement<C>,
    solver: &mut LimnSolver,
    dpi: f32,
    index: &mut usize
) -> Layout {
    let id = match element {
        LayoutElement::Padded { .. } => format!("{}_padding", element.id()),
        LayoutElement::Modal { .. } => format!("{}_background", element.id()),
        _ => element.id().to_string()
    };
    let mut layout = Layout::new(*index, Some(id.clone()));
    let mut children_to_update = Vec::new();

    match element {
        LayoutElement::Custom(custom) => {
            custom.visit_layout(&mut layout, &mut children_to_update, dpi, index);
        }
        LayoutElement::Linear {
            constraints,
            options,
            children,
            padding,
            ..
        } => {
            let padding = padding.as_ref().copied().unwrap_or_else(Default::default);
            layout.set_container(ExactFrame {
                padding: padding * dpi
            });

            if let Some(constraints) = constraints.as_ref() {
                apply_constraints(&mut layout, constraints, dpi);
            }

            *index += 1;
            let mut linear_layout = Layout::new(*index, Some(format!("{}_linear", id)));
            let mut options = options.clone().unwrap_or_else(Default::default);
            options.padding *= dpi;
            let container = LinearLayout::new(&mut linear_layout, options);
            linear_layout.set_container(container);

            for child in children {
                *index += 1;
                let mut child_layout = walk_layout_tree(child, solver, dpi, index);
                linear_layout.add_child(&mut child_layout);
                children_to_update.push(child_layout);
            }

            solver.update_layout(&mut linear_layout);
            layout.add_child(&mut linear_layout);
            children_to_update.push(linear_layout);
        }
        LayoutElement::Grid { .. } => {
            todo!("Implement grid");
        }
        LayoutElement::Padded { padding, inner } => {
            layout.set_container(ExactFrame {
                padding: *padding * dpi
            });
            *index += 1;
            let mut inner = walk_layout_tree(inner, solver, dpi, index);
            layout.add_child(&mut inner);
            children_to_update.push(inner);
        }
        LayoutElement::Modal {
            id,
            constraints,
            header,
            content,
            ..
        } => {
            layout.set_container(CenteredLayout);

            *index += 1;
            let mut container_layout = Layout::new(*index, Some(id.to_string()));
            let container = LinearLayout::new(
                &mut container_layout,
                LinearLayoutData {
                    spacing: Spacing::Around,
                    alignment: ItemAlignment::Fill,
                    ..LinearLayoutData::default()
                }
            );
            container_layout.set_container(container);

            if let Some(constraints) = constraints {
                apply_constraints(&mut container_layout, constraints, dpi);
            }

            *index += 1;
            let mut header = walk_layout_tree(header, solver, dpi, index);
            *index += 1;
            let mut content = walk_layout_tree(content, solver, dpi, index);

            container_layout.add_child(&mut header);
            container_layout.add_child(&mut content);

            layout.add_child(&mut container_layout);

            children_to_update.push(container_layout);
            children_to_update.push(header);
            children_to_update.push(content);
        }
        LayoutElement::Label { constraints, .. } => {
            let constraints = constraints.clone().unwrap_or_default();
            // This doesn't work the way things are set up now, skip it for now
            // todo: Implement this
            /*            let font = get_font(text.0.font.as_ref(), fonts);
            let scale = Scale::uniform(text.0.font_size);
            let v_metrics = font.0.v_metrics(scale);

            if constraints.min_height.is_none() {
                match text.0.line_mode.unwrap_or(LineMode::Single) {
                    LineMode::Single => {
                        constraints.min_height = Some(v_metrics.ascent + v_metrics.descent);
                    }
                    LineMode::Wrap => {
                        if constraints.preferred_width.is_none() && constraints.min_width.is_none() {}
                        else {
                            let line_breaks = xi_unicode::LineBreakIterator::new(&text.0.text);

                            let width = constraints.preferred_width.or(constraints.min_width).unwrap();
                            let mut words = Vec::new();
                            let mut last_index = 0;
                            for (index, hard) in line_breaks {
                                words.push((&text.0.text[last_index..index], hard));
                                last_index = index;
                            }
                            let mut lines = 1;
                            let mut current_length = 0.0;
                            for (word, hard) in words {
                                if hard {
                                    lines += 1;
                                    current_length = 0.0;
                                } else {
                                    let word_len = calculate_word_length(word, font, scale);
                                    if current_length + word_len > width {
                                        lines += 1;
                                        current_length = word_len;
                                    } else {
                                        current_length += word_len;
                                    }
                                }
                            }
                            let height = lines * v_metrics.descent + v_metrics.ascent + (lines - 1) * v_metrics.line_gap;
                            constraints.preferred_height = height;
                            log::debug!("Calculated Lines: {}, Calculated height: {}", lines, height);
                        }
                    }
                }
            }*/

            apply_constraints(&mut layout, &constraints, dpi);
        }
        _ => {
            let constraints = element.constraints();
            if let Some(constraints) = constraints {
                apply_constraints(&mut layout, constraints, dpi);
            }
        }
    }

    solver.update_layout(&mut layout);
    for child in children_to_update.iter_mut() {
        solver.update_layout(child);
    }
    layout
}

/*fn calculate_word_length(word: &str, font: &FontAsset, scale: Scale) -> f32 {
    word.chars()
        .map(|char| font.0.glyph(char).scaled(scale))
        .map(|glyph| glyph.h_metrics().advance_width)
        .sum()
}

fn get_font<'a>(font: Option<&AssetPrefab<FontAsset>>, fonts: &'a AssetStorage<FontAsset>) -> &'a FontAsset {
    let font_handle = match font {
        Some(AssetPrefab::Handle(handle)) => handle,
        _ => panic!("Font should be present and loaded at this point")
    };
    fonts.get(font_handle).expect("Font should have finished loading by now")
}*/

pub fn apply_constraints(layout: &mut Layout, constraints: &SizeConstraints, dpi: f32) {
    if let Some(min_width) = constraints.min_width {
        layout.add(constraint::min_width(min_width * dpi));
    }

    if let Some(min_height) = constraints.min_height {
        layout.add(constraint::min_height(min_height * dpi));
    }

    if let Some(max_width) = constraints.max_width {
        layout.add(constraint::max_width(max_width * dpi))
    }

    if let Some(max_height) = constraints.max_height {
        layout.add(constraint::max_height(max_height * dpi));
    }

    if let Some(preferred_width) = constraints.preferred_width {
        layout.add(constraint::width(preferred_width * dpi).strength(MEDIUM));
    }

    if let Some(preferred_height) = constraints.preferred_height {
        layout.add(constraint::height(preferred_height * dpi).strength(MEDIUM));
    }
}

pub fn solve_layout(data: &mut DynamicLayoutData, (screen_width, screen_height): (f32, f32)) {
    let DynamicLayoutData {
        root,
        solver,
        solutions: solution_map
    } = data;

    {
        let width = root.edit_width();
        width.set(screen_width);
    }
    {
        let height = root.edit_height();
        height.set(screen_height);
    }

    solver.update_layout(root);
    let solution = solver.fetch_changes();

    for (layout, var, val) in solution {
        let layout = solution_map
            .entry(layout)
            .or_insert_with(CalculatedDimensions::default);
        match var {
            VarType::Left => {
                layout.left = val as f32;
            }
            VarType::Top => {
                layout.top = val as f32;
            }
            VarType::Width => {
                layout.width = val as f32;
            }
            VarType::Height => {
                layout.height = val as f32;
            }
            _ => {}
        }
    }
}
