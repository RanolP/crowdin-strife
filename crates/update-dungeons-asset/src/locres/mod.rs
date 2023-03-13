//! This module is partial port of https://github.com/akintos/UnrealLocres/tree/master/LocresLib

pub use buf_ext::SeekReadError;
pub use file::*;
pub use namespace::*;

pub(super) mod buf_ext;
mod file;
mod namespace;
mod version;
