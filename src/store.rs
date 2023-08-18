use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

/// A way to store and access generic items
/// via human-friendly names.
#[derive(Clone, Debug)]
pub struct Store<T: Clone>(HashMap<String, T>);

impl<T: Clone> Store<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Store an item in a builder pattern.
    ///
    /// ```
    /// # use genji::store::Store;
    /// # let store = Store::new();
    /// # let item = ();
    ///
    /// let store = store.with("item", item);
    /// ```
    pub fn with<I: ToString>(mut self, id: I, item: T) -> Self {
        self.0.insert(id.to_string(), item);
        self
    }

    /// Store an item.
    ///
    /// ```
    /// # use genji::store::Store;
    /// # let item = ();
    ///
    /// store.add("item", item);
    /// ```
    pub fn add<I: ToString>(&mut self, id: I, item: T) {
        self.0.insert(id.to_string(), item);
    }

    /// Returns an item if it exists.
    ///
    /// ```
    /// # use genji::store::Store;
    /// # let item = ();
    /// store.add("item", item);
    ///
    /// assert!(store.get("item").is_some());
    /// ```
    pub fn get<I: ToString>(&self, id: I) -> Option<T> {
        self.0.get(&id.to_string()).cloned()
    }

    /// Remove an item, returning it if it exists.
    ///
    /// ```
    /// # use genji::store::Store;
    /// # let item = ();
    /// store.add("item", item);
    ///
    /// assert!(store.remove("item").is_some());
    /// assert!(store.remove("item").is_none());
    /// ```
    pub fn remove<I: ToString>(&mut self, id: I) -> Option<T> {
        self.0.remove(&id.to_string())
    }
}

impl<T: Clone> Deref for Store<T> {
    type Target = HashMap<String, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone> DerefMut for Store<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Clone> Default for Store<T> {
    fn default() -> Self {
        Self::new()
    }
}
