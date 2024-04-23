use super::*;

pub trait Verb : DynClone {}

impl Token for dyn Verb {}

#[derive(Clone)]
pub struct VerbStem {
    text: String
}   

impl Verb for VerbStem {}

// FIXME: Is there a solution where we don't have duplicate masu/polite
// It depends on whether we value less code or nicer parsing
// No duplicate: We have to parse sequetially, so we make a token and conjugate it
// duplicate: We can do a first pass over to split into tokens

#[derive(Clone)]
pub struct Masu {}
impl Token for Masu {}

// Maybe this will be a dyn eventually
#[derive(Clone)]
pub struct PoliteVerb<V: Verb> {
    stem: V
}

impl<V: Verb> PoliteVerb<V> {
    pub fn new(stem: V) -> PoliteVerb<V> {
        PoliteVerb { stem }
    }
}

impl Particle for Masu {
    fn conjugate(self, lhs: &dyn Token, rhs: &dyn Token) -> Result<(Box<dyn Token>, [bool; 2]), ConjugateError> {
        // We just use lhs

        (Box::new(PoliteVerb::new()))
    }
}