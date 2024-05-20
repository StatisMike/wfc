//! Wave Function Collapse algorithm implementation for arbitrary grids.
//!
//! Functionality in this crate allows generating two-dimensional grid of some arbitrary data using patterns from sample
//! data. If you are looking for implementation over images, there is `wfc-image` crate which is tailored to be used
//! for it.
//!
//! If your aim is to generate grids of some custom type, you can implement WFC algorithm using runner in this crate.
//! General workflow:
//!
//! 1. Generate [`OverlappingPatterns`](crate::overlapping::OverlappingPatterns) using sample [`Grid`](grid_2d::Grid).
//! 2. Use [`GlobalStats`] from overlapping patterns to create either:
//!   - runner owning the data: [`RunOwn`]
//!   - runner borrowing the data: [`RunBorrow`]
//!    Setup of additional objects may be needed depending on your choice.
//! 3. Run the algorithm, either completely to its completion (`collapse()` / `collapse_retrying()`) or on step by step basis
//!   (`step()`).
//! 4. Convert generated [`Wave`] into output of your choice.

pub mod orientation;
pub mod overlapping;
pub mod retry;
mod tiled_slice;
mod wfc;
pub mod wrap;

pub use crate::wfc::*;
pub use coord_2d::{Coord, Size};
pub use orientation::Orientation;
pub use wrap::Wrap;
