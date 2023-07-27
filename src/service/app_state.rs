use crate::world::World;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct AppState {
    map: dashmap::DashMap<String, World>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            map: dashmap::DashMap::new(),
        }
    }

    pub fn fetch(&self, key: &str) -> Option<impl Deref<Target = World> + '_> {
        let x = self.map.get(key)?;
        Some(x)
    }

    pub fn fetch_mut(&self, key: &str) -> Option<impl DerefMut<Target = World> + '_> {
        self.map.get_mut(key)
    }

    pub fn update(&self, key: &str, world: World) {
        self.map.insert(key.to_owned(), world);
    }
}
