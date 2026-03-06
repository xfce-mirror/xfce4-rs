#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unused_imports)]

mod auto;
mod shortcut;

pub use auto::traits::*;
pub use auto::*;
pub use ffi;
pub use shortcut::*;
