#[macro_use]
extern crate bitflags;

// #[cfg(feature = "shared-api")]
// pub mod shared_lib;
// #[cfg(feature = "shared-api")]
// pub use shared_lib as bgfx;
#[cfg(not(feature = "shared-api"))]
mod static_lib;
#[cfg(not(feature = "shared-api"))]
pub use static_lib::*;
pub mod bgfx {
    pub use super::static_lib::*;
}
// pub use static_lib as bgfx;

