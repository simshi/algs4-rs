mod heap;
mod heap_opt;
mod insertion;
mod max_pq;
mod merge;
mod merge_opt;
mod quick;
mod quick3way;
mod quick_opt;
mod selection;
mod shell;

pub use self::heap::*;
pub use self::heap_opt::*;
pub use self::insertion::*;
pub use self::max_pq::*;
pub use self::merge::*;
pub use self::merge_opt::*;
pub use self::quick::*;
pub use self::quick3way::*;
pub use self::quick_opt::*;
pub use self::selection::*;
pub use self::shell::*;

pub fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
	arr.windows(2).all(|w| w[0] <= w[1])
}
