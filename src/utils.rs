use std::{collections::HashMap};

use vek::vec::Vec2;

#[macro_export]
macro_rules! try_block {
	{ $($token:tt)* } => {{
		|| -> Option<()> {
			$($token)*
			Some(())
		}()
	}}
}

pub fn map_area<T, V>(origin: Vec2<T>, map: &HashMap<Vec2<T>, V>) -> (Vec2<T>, Vec2<T>) 
	where T: Ord + Copy
{
	map.keys().fold((origin, origin), |(min, max), v| (Vec2::min(min, *v), Vec2::max(max, *v)))
}