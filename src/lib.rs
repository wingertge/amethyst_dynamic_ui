#![cfg_attr(feature = "document", feature(doc_cfg))]

use amethyst::core::math::{Point2, Vector2};
use std::fmt;

pub mod assets;
mod bundle;
#[allow(clippy::precedence)]
pub mod constraint;
#[allow(clippy::precedence)]
pub mod layout;
pub mod prefab;
pub mod resize_system;
pub mod retrigger;
pub mod solver;
#[cfg_attr(feature = "document", doc(cfg(feature = "fmod")))]
#[cfg(feature = "fmod")]
pub mod sound;
pub mod tint;
mod widget;

pub use bundle::*;
use fnv::FnvBuildHasher;
pub use prefab::*;
pub use widget::*;

pub(crate) type HashMap<K, V> = std::collections::HashMap<K, V, FnvBuildHasher>;

pub type Length = f32;
pub type Size = Vector2<f32>;
pub type Point = Point2<f32>;

#[derive(Clone, Debug)]
pub struct Rect {
    pub origin: Point2<f32>,
    pub size: Vector2<f32>
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Rect {
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
}
