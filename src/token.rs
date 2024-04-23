use dyn_clone::DynClone;

use crate::error::ConjugateError;

pub mod verb;

pub struct JapaneseSentence {
    tokens: Vec<Box<dyn Token>>,
}

pub trait Token : DynClone {
    fn is_trait<T>(&self) -> bool where T: Particle {};
    fn is_trait<T>(&self) -> bool where T: ?Particle {};
}

pub trait Particle : DynClone {
    fn conjugate_true<>(self, lhs: &dyn Token, rhs: &dyn Token) -> Result<(Box<dyn Token>, [bool; 2]), ConjugateError>;
}

impl Token for dyn Particle {
    fn is_trait<T>() -> bool 
    where T: Particle
    {
        
    }
}

// Hacky shit

pub trait Container<const B: bool> {}
pub trait Boolean {
    fn eval() -> bool;
}
impl Boolean for dyn Container<true> {
    fn eval() -> bool {
        true
    }
}
impl Boolean for dyn Container<false> {
    fn eval() -> bool {
        false
    }
}
