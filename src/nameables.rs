use crate::Renamed;

use std::fmt::Debug;

mod impls;

pub trait AnyNameableBounds<'a>: Debug + 'a {}
impl<'a, T: Debug + 'a> AnyNameableBounds<'a> for T {}

pub struct AnyNameable<'a>(pub(crate) Box<dyn AnyNameableBounds<'a>>);

impl<'a> AnyNameable<'a> {
    pub fn new<T: AnyNameableBounds<'a>>(t: T) -> Self {
        AnyNameable(Box::new(t))
    }
}

impl<'a> std::ops::Deref for AnyNameable<'a> {
    type Target = dyn Debug + 'a;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a> std::fmt::Display for AnyNameable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> std::fmt::Debug for AnyNameable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.0)
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

pub trait Rename: Sized + Debug {
    fn nameables(&self) -> Vec<AnyNameable<'_>>;

    fn renamed(self) -> Renamed<Self> {
        self.into()
    }
}
