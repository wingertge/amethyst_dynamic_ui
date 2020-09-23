use crate::{
    prefab::{CalculatedDimensions, DynamicLayout, LayoutIdentifier},
    solve_layout, HashMap
};
use amethyst::{
    core::{
        ecs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteStorage},
        ParentHierarchy
    },
    derive::SystemDesc,
    ecs::SystemData,
    ui::UiTransform,
    window::ScreenDimensions
};

#[derive(SystemDesc, Default)]
#[system_desc(name(ResizeSystemDesc))]
pub struct ResizeSystem {
    #[system_desc(skip)]
    screen_size: (f32, f32)
}

impl<'s> System<'s> for ResizeSystem {
    type SystemData = (
        WriteStorage<'s, DynamicLayout>,
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, LayoutIdentifier>,
        ReadExpect<'s, ScreenDimensions>,
        ReadExpect<'s, ParentHierarchy>,
        Entities<'s>
    );

    fn run(
        &mut self,
        (
            ref mut dynamic_layouts,
            ref mut transforms,
            ref layout_ids,
            ref screen_dimensions,
            ref hierarchy,
            ref entities
        ): Self::SystemData
    ) {
        let dimensions = (screen_dimensions.width(), screen_dimensions.height());
        if dimensions != self.screen_size {
            println!("Updating UI for new screen size");
            for (layout, entity) in (dynamic_layouts, entities).join() {
                let mut layout = layout.0.lock().unwrap();
                solve_layout(&mut layout, dimensions);

                layout.solver.debug_layouts();

                update_tree(
                    &mut layout.solutions,
                    entity,
                    hierarchy.children(entity),
                    transforms,
                    layout_ids,
                    hierarchy,
                    dimensions,
                    self.screen_size
                );
            }
            self.screen_size = dimensions;
        }
    }
}

fn update_tree(
    solution: &mut HashMap<usize, CalculatedDimensions>,
    item: Entity,
    children: &[Entity],
    transforms: &mut WriteStorage<'_, UiTransform>,
    layout_ids: &ReadStorage<'_, LayoutIdentifier>,
    hierarchy: &ParentHierarchy,
    (width, height): (f32, f32),
    (last_width, last_height): (f32, f32)
) {
    if let Some((transform, id)) = transforms.get_mut(item).zip(layout_ids.get(item)) {
        log::debug!("Updating transform for {}", transform.id);
        if let Some(dims) = solution.get(id) {
            transform.local_x = dims.left;
            transform.local_y = -dims.top;
            transform.width = dims.width;
            transform.height = dims.height;
        }
        log::debug!("{:?}", transform);
    }

    for child in children {
        update_tree(
            solution,
            *child,
            hierarchy.children(*child),
            transforms,
            layout_ids,
            hierarchy,
            (width, height),
            (last_width, last_height)
        );
    }
}
