use std::marker::PhantomData;

#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Default,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
pub struct ID<T> {
    id: usize,
    _phantom: PhantomData<T>,
}

impl<T> ID<T> {
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