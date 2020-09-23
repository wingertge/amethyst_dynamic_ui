use crate::{
    constraints,
    constraint,
    constraint::ConstraintBuilder,
    layout::{Layout, LayoutContainer}
};
use cassowary::{strength::REQUIRED, WeightedRelation::EQ};

pub struct CenteredLayout;

impl LayoutContainer for CenteredLayout {
    fn add_child(&mut self, parent: &mut Layout, child: &mut Layout) {
        child.add(constraint::bound_by(&parent));
        child.add(constraints![
            child.vars.top + child.vars.height / 2.0
                | EQ(REQUIRED)
                | parent.vars.top + parent.vars.height / 2.0,
            child.vars.left + child.vars.width / 2.0
                | EQ(REQUIRED)
                | parent.vars.left + parent.vars.width / 2.0
        ]);
    }
}
