pub use aoc_util::array2d::{Pos, Array2D};

#[macro_export]
macro_rules! try_block {
	{ $($token:tt)* } => {{
		|| -> Option<()> {
			$($token)*
			Some(())
		}()
	}}
}
