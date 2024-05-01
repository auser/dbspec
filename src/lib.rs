mod db_introspect;
mod faking;

pub use db_introspect::get_tables;

#[cfg(feature = "fake")]
pub use faking::Faking;

#[cfg(feature = "fake")]
pub use fakeit::*;
