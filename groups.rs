#![crate_type = "lib"]
#![crate_name = "groups"]

#![feature(globs)]

extern crate sets;
extern crate operators;

use sets::*;
use operators::*;

pub trait Magma<Self, Op: Fn<(Self,Self),Self>>: Set {
	/*
	 * requires an operator
	 * (M x M) -> M
	 */
}

pub trait Semigroup<Self, Op: IsAssociative<Self,Self,Self>>: Magma<Self, Op> {
}

pub trait Monoid<Self, Op>: Semigroup<Self, Op> + HasIdentity<Self, Op> {
}

pub trait Quasigroup<Self, Op: IsLeftInvertible<Self,Self,Self> + IsRightInvertible<Self,Self,Self>>:
	Magma<Self, Op>
{
}

pub trait Loop<Self, Op>: Quasigroup<Self, Op> + HasIdentity<Self, Op> {
}

pub trait Group<Self, Op: IsLeftInvertible<Self, Self, Self> + IsRightInvertible<Self, Self, Self>>:
	Monoid<Self, Op> + HasInverse<Self>
{
	// TODO: remove since HasInverse
	fn inverse(x: Self) -> Self;
}

pub trait AbelianGroup<Self, Op: IsInvertible<Self,Self>>: Group<Self, Op> {
}

struct MyMagma;
impl MyMagma {
	fn new() -> MyMagma {
		return MyMagma;
	}
}

struct MyOp;
impl Fn<(MyMagma,MyMagma),MyMagma> for MyOp {
	extern "rust-call" fn call(&self, args: (MyMagma,MyMagma)) -> MyMagma {
		return MyMagma::new();
	}
}

fn main() {

}
