use std::{ops::Deref, sync::RwLock};

#[derive(Debug)]
pub struct UnsafeRwLock<T>(RwLock<T>);
impl<T> UnsafeRwLock<T> {
    pub fn new(value: T) -> Self {
        Self(RwLock::new(value))
    }
    pub fn unsafe_read(&self) -> T {
        // self.0
        //     .try_read()
        //     .expect("You should not have called `unsafe_read` while locked.")
        //     .deref()

        todo!()
    }
    pub fn unsafe_write(&self, new: T) {
        *self
            .0
            .try_write()
            .expect("You should not have called `unsafe_write` while locked.") = new;
    }
}
impl<T: std::hash::Hash> std::hash::Hash for UnsafeRwLock<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.unsafe_read().hash(state)
    }
}
impl<T: Eq> Eq for UnsafeRwLock<T> {}
impl<T: PartialEq> PartialEq for UnsafeRwLock<T> {
    fn eq(&self, other: &Self) -> bool {
        self.unsafe_read() == other.unsafe_read()
    }
}
impl<T: Clone> Clone for UnsafeRwLock<T> {
    fn clone(&self) -> Self {
        Self::new(self.unsafe_read().clone())
    }
}
