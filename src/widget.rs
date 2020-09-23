use crate::{
    constraint,
    constraint::Padding,
    layout::{
        centered::CenteredLayout,
        linear::{ItemAlignment, LinearLayout, LinearLayoutData, Spacing},
        Frame, Layout, VarType
    },
    prefab::{CalculatedDimensions, DynamicLayout, DynamicLayoutData, ExtraButtonData, Tinted},
    solver::LimnSolver,
    HashMap
};
use amethyst::{
    assets::{AssetPrefab, PrefabData},
    audio::Source,
    ui::{Anchor, UiButtonData, UiImageLoadPrefab, UiImagePrefab, UiTextData}
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
        &mut self,
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

fn default_modal_background() -> UiImagePrefab {
    UiImagePrefab(UiImageLoadPrefab::SolidColor(0.0, 0.0, 0.0, 0.627))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SizeConstraints {
    min_width: Option<f32>,
    min_height: Option<f32>,
    max_width: Option<f32>,
    max_height: Option<f32>,
    preferred_width: Option<f32>,
    preferred_height: Option<f32>
}

#[derive(Serialize, Deserialize, Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct ModalData {
    #[serde(default = "default_modal_anchor")]
    #[derivative(Default(value = "Anchor::Middle"))]
    pub align: Anchor,
    #[serde(default = "default_modal_background")]
    #[derivative(Default(value = "default_modal_background()"))]
    pub background: UiImagePrefab
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NoCustomElements;

impl ToLayoutElement for NoCustomElements {
    type PrefabData = ();

    fn into_native_widget(self, _: Self::PrefabData) -> (LayoutElement<Self>, Self::PrefabData) {
        unreachable!()
    }

    fn visit_layout(&mut self, _: &mut Layout, _: &mut Vec<Layout>, _: f32, _: &mut usize) {
        unreachable!()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ImageButtonData {
    pub icon: Option<UiImageLoadPrefab>,
    pub margin: Option<f32>,
    // this `normal_image` is "transplanted" into UiImagePrefab at the top level
    // of ui widget. This happens inside `walk_ui_tree` function. It means that
    // it will always be `None` during `add_to_entity`.
    /// Default image
    pub normal_image: Option<UiImageLoadPrefab>,
    /// Image used when the mouse hovers over this element
    pub hover_image: Option<UiImageLoadPrefab>,
    /// Image used when button is pressed
    pub press_image: Option<UiImageLoadPrefab>,
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
        normal_image: button.normal_image.take(),
        hover_image: button.hover_image.take(),
        hover_text_color: None,
        press_image: button.press_image.take(),
        press_text_color: None,
        hover_sound: button.hover_sound.take(),
        press_sound: button.press_sound.take(),
        release_sound: button.release_sound.take()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum LayoutElement<C: ToLayoutElement = NoCustomElements> {
    Image {
        id: String,
        image: UiImagePrefab,
        constraints: Option<SizeConstraints>,
        #[serde(default = "true_")]
        keep_aspect_ratio: bool
    },
    Button {
        id: String,
        button: UiButtonData<u32>,
        extra_button: Option<ExtraButtonData>,
        constraints: Option<SizeConstraints>
    },
    ImageButton {
        id: String,
        image_button: ImageButtonData,
        extra_button: Option<ExtraButtonData>,
        constraints: Option<SizeConstraints>
    },
    Label {
        id: String,
        text: UiTextData,
        constraints: Option<SizeConstraints>
    },
    Linear {
        id: String,
        background: Option<UiImagePrefab>,
        tint: Option<Tinted>,
        constraints: Option<SizeConstraints>,
        options: Option<LinearLayoutData>,
        children: Vec<LayoutElement<C>>,
        padding: Option<Padding>
    },
    Grid {
        id: String,
        columns: usize,
        background: Option<UiImagePrefab>,
        tint: Option<Tinted>,
        constraints: Option<SizeConstraints>,
        children: Vec<LayoutElement<C>>,
        padding: Option<Padding>
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
        background: Option<UiImagePrefab>,
        tint: Option<Tinted>
    },
    Custom(Box<C>)
}

impl<C: ToLayoutElement> LayoutElement<C> {
    /// Solve layout and transform into widget tree with absolute
    pub fn create_solver(
        &mut self,
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
    element: &mut LayoutElement<C>,
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
            layout.set_container(Frame { padding: padding * dpi });

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
            layout.set_container(Frame {
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
