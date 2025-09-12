use std::{
    cell::Cell,
    sync::{Arc, Mutex},
};

pub struct StaticSafeMutableField<T>(Arc<Mutex<Cell<Option<T>>>>);

impl<T> StaticSafeMutableField<T> {
    pub fn apply_mut<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> anyhow::Result<R> {
        let mg = self
            .0
            .lock()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        let mut data = mg.take().unwrap();
        let result = f(&mut data);
        mg.replace(Some(data));
        Ok(result)
    }

    pub fn apply<R, F: FnOnce(&T) -> R>(&self, f: F) -> anyhow::Result<R> {
        let mg = self
            .0
            .lock()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        let data = mg.take().unwrap();
        let result = f(&data);
        mg.replace(Some(data));
        Ok(result)
    }
}

impl<T: Default> Default for StaticSafeMutableField<T> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Cell::new(Some(T::default())))))
    }
}
