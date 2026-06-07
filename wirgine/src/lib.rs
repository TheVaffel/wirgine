#![feature(extern_types)]

mod c_functions;
mod c_types;
mod utils;
mod wrappers;

pub use wrappers::*;

extern crate wirgine_macros;

pub use utils::image::Image;
pub use utils::IsReprC;
pub use wirgine_macros::IsReprC as IsReprCMacro;
