//! Re-exports everything from [hecs].
//!
//! Genji uses the excellent [hecs]
//! ECS crate, with no real modifications. However, an
//! [`EntityStore`] struct is provided
//! to give convenient ID's to entities.

pub use hecs::*;

use crate::store::Store;

/// A way to store and access
/// [`Entity`]s
/// via human-friendly names.
pub type EntityStore = Store<Entity>;
