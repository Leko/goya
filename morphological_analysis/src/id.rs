use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, Hash, Default, PartialEq, Eq, Ord, PartialOrd)]
pub struct Id<T> {
    id: usize,
    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self) -> usize {
        self.id
    }
}
