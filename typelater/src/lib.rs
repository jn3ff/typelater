extern crate typelater_derive;
pub use typelater_derive::Typelater; // attribute macro, derive macro
pub trait Typelater<T>: From<T> {}
