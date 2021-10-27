use std::fmt::Debug;

pub trait Object: Sized + Clone + Debug {
    type Id: Sized + Clone + Debug;

    fn id(&self) -> &Self::Id;
}
