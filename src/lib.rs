mod downloader;
mod loader;
mod model;
pub mod util;

pub use downloader::{download3, download4};
pub use loader::load_data;
pub use model::NumbersData;
