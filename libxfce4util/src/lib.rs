#![cfg_attr(docsrs, feature(doc_cfg))]

mod auto;
mod functions;

pub use auto::functions::*;
pub use auto::*;
pub use ffi;
pub use functions::*;
