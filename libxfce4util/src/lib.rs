#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

mod auto;
mod consolekit;
mod functions;
mod systemd;

pub use auto::functions::*;
pub use auto::*;
pub use consolekit::*;
pub use functions::*;
pub use libxfce4util_sys as ffi;
pub use systemd::*;
