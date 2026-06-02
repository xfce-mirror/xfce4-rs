#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unused_imports)]
#![allow(unsafe_op_in_unsafe_fn)]

mod auto;
#[cfg(feature = "v4_21_4")]
mod shortcut;

pub use auto::traits::*;
pub use auto::*;
pub use ffi;
#[cfg(feature = "v4_21_4")]
pub use shortcut::*;
