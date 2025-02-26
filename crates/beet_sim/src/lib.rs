#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]
#![feature(let_chains)]
pub mod behavior;
pub mod plugins;
#[cfg(feature = "render")]
pub mod render;
pub mod sim;
pub mod stat_modifiers;
pub mod stats;





pub mod prelude {
	pub use crate::behavior::*;
	pub use crate::plugins::*;
	#[cfg(feature = "render")]
	pub use crate::render::*;
	pub use crate::sim::*;
	pub use crate::stat_modifiers::*;
	pub use crate::stats::*;
}
