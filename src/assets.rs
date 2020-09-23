use crate::HashMap;
use amethyst::{assets::Handle, renderer::Texture, ui::FontAsset};
use parking_lot::RwLock;
use std::ops::Deref;

#[derive(Default)]
pub struct Fonts(RwLock<HashMap<String, Handle<FontAsset>>>);

impl Deref for Fonts {
    type Target = RwLock<HashMap<String, Handle<FontAsset>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
pub struct Textures(RwLock<HashMap<String, Handle<Texture>>>);

impl Deref for Textures {
    type Target = RwLock<HashMap<String, Handle<Texture>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
