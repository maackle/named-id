use crate::Renamed;

use std::fmt::Debug;

mod impls;

pub trait AnyNameableBounds: Debug + 'static {}
impl<T: Debug + 'static> AnyNameableBounds for T {}

pub struct AnyNameable(pub(crate) Box<dyn AnyNameableBounds>);

impl AnyNameable {
    pub fn new<T: AnyNameableBounds>(t: T) -> Self {
        AnyNameable(Box::new(t))
    }
}

impl std::ops::Deref for AnyNameable {
    type Target = dyn Debug;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl std::fmt::Display for AnyNameable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Debug for AnyNameable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.0)
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

pub trait Rename: Sized + Debug {
    fn nameables(&self) -> Vec<AnyNameable>;

    fn renamed(self) -> Renamed<Self> {
        self.into()
    }
}
