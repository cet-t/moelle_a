#![no_std]
pub(crate) mod splitmix;

#[cfg(feature = "biski")]
mod biski;
#[cfg(feature = "biski")]
pub use crate::biski::*;

#[cfg(feature = "mt")]
mod mt;
#[cfg(feature = "mt")]
pub use crate::mt::*;

#[cfg(feature = "sfmt")]
mod sfmt;
#[cfg(feature = "sfmt")]
pub use crate::sfmt::*;

#[cfg(feature = "xorwow")]
mod xorwow;
#[cfg(feature = "xorwow")]
pub use crate::xorwow::*;

#[cfg(feature = "sfc")]
mod sfc;
#[cfg(feature = "sfc")]
pub use crate::sfc::*;

#[cfg(feature = "squares")]
mod squares;
#[cfg(feature = "squares")]
pub use crate::squares::*;
