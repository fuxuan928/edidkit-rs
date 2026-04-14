pub mod base;
pub mod cta861;
pub mod displayid;

mod edid;
mod error;
mod utils;

pub use edid::{Edid, ExtensionBlock, Product};
pub use error::EdidError;
