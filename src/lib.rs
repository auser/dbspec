mod db_introspect;
pub use db_introspect::get_tables;

#[cfg(feature = "fake")]
mod faking;

#[cfg(feature = "fake")]
pub use faking::Faking;

#[cfg(feature = "fake")]
pub use fakeit::*;
