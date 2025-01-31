use core::hash::Hash;
use core::borrow::Borrow;
use crate::*;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
/// Unicode case-insensitive wrapper without optimisation
pub struct UniCaseNoOpt<S:?Sized>(S);

impl<S> UniCaseNoOpt<S> {
    pub fn new(s: S) -> Self { UniCaseNoOpt(s) }
}
impl<S:?Sized> UniCaseNoOpt<S> {
    pub fn from_ref<'s>(s: &'s S) -> &'s UniCaseNoOpt<S> {
        let x: &'s S               = s;
        let y: &'s UniCaseNoOpt<S> = unsafe { core::mem::transmute(x) };
        y
    }
}
impl<S> From<S> for UniCaseNoOpt<S> {
    fn from(s: S) -> UniCaseNoOpt<S> { UniCaseNoOpt(s) }
}

impl PartialEq for UniCaseNoOpt<str>
{
    fn eq(&self, other: &Self) -> bool {
        Unicode(&self.0) ==
        Unicode(&other.0)
    }
}
impl Eq for UniCaseNoOpt<str> { }

impl<S:?Sized> Hash for UniCaseNoOpt<S>
where S: AsRef<str>, for <'u> UniCase<&'u S>: Hash
{
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        Unicode(&self.0).hash(hasher)
    }
}

impl<S:AsRef<str>> Borrow<UniCaseNoOpt<str>> for UniCase<S> {
    fn borrow(&self) -> &UniCaseNoOpt<str> {
        UniCaseNoOpt::from_ref(self.as_ref())
    }
}

#[test]
fn hashset() {
    use std::collections::HashSet;

    let mut hm: HashSet<UniCase<String>> = Default::default();
    hm.insert(UniCase::new("ascii".to_string()));
    assert!(hm.contains(UniCaseNoOpt::from_ref("Ascii")));
}
