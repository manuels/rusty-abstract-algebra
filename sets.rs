#![crate_type = "lib"]
#![crate_name = "sets"]

pub trait Set: PartialEq {}

pub trait HasInverse<Self> {
	fn inverse(x: Self) -> Self;
}

pub trait HasIdentity<Self, Op: Fn<(Self,Self),Self>> {
	/*
	 * requires a Identity element
	 */
	fn identity() -> Self;
}
