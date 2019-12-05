mod heap;
mod heap_opt;
mod insertion;
mod max_pq;
mod merge;
mod quick;
mod quick3way;
mod selection;
mod shell;

pub use self::heap::*;
pub use self::heap_opt::*;
pub use self::insertion::*;
pub use self::max_pq::*;
pub use self::merge::*;
pub use self::quick::*;
pub use self::quick3way::*;
pub use self::selection::*;
pub use self::shell::*;

pub fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
	arr.windows(2).all(|w| w[0] <= w[1])
}
