//! Re-exports everything from [hecs].
//!
//! Genji uses the excellent [hecs]
//! ECS crate, with no real modifications. However, an
//! [`EntityStore`] struct is provided
//! to give ID's to entities.

use std::collections::HashMap;

pub use hecs::*;

/// A way to store and access
/// [`Entity`]s
/// via human-friendly names.
#[derive(Clone, Debug, Default)]
pub struct EntityStore(HashMap<String, Entity>);

impl EntityStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Store an entity.
    /// 
    /// ```
    /// # use genji::ecs::{Entity, EntityStore};
    /// # let mut store = EntityStore::new();
    /// # let player = Entity::DANGLING;
    /// 
    /// store.add("player", player);
    /// ```
    pub fn add<S: ToString>(&mut self, id: S, entity: Entity) {
        self.0.insert(id.to_string(), entity);
    }

    /// Gets an entity, returning it if it exists.
    /// 
    /// ```
    /// # use genji::ecs::{Entity, EntityStore};
    /// # let mut store = EntityStore::new();
    /// # let player = Entity::DANGLING;
    /// store.add("player", player);
    /// 
    /// assert!(store.get("player").is_some());
    /// ```
    pub fn get<S: ToString>(&self, id: S) -> Option<Entity> {
        self.0.get(&id.to_string()).cloned()
    }

    /// Remove an entity, returning it if it exists.
    /// 
    /// ```
    /// # use genji::ecs::{Entity, EntityStore};
    /// # let mut store = EntityStore::new();
    /// # let player = Entity::DANGLING;
    /// store.add("player", player);
    /// 
    /// assert!(store.remove("player").is_some());
    /// assert!(store.remove("player").is_none());
    /// ```
    pub fn remove<S: ToString>(&mut self, id: S) -> Option<Entity> {
        self.0.remove(&id.to_string())
    }
}
