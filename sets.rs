#![crate_type = "lib"]
#![crate_name = "sets"]

pub trait Set {}

pub trait HasInverse<S> {
	fn inverse(x: S) -> S;
}

pub trait HasIdentity<S> {
	/*
	 * requires a Identity element
	 */
	fn identity() -> S;
}

pub trait PartiallyOrdered {
	fn less_or_equal(&self, other: Self) -> bool;
}
