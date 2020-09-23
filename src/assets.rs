use amethyst::assets::Handle;
use amethyst::ui::FontAsset;
use derive_deref::Deref;
use amethyst::renderer::Texture;
use parking_lot::RwLock;
use crate::HashMap;

#[derive(Deref, Default)]
pub struct Fonts(RwLock<HashMap<String, Handle<FontAsset>>>);

#[derive(Deref, Default)]
pub struct Textures(RwLock<HashMap<String, Handle<Texture>>>);