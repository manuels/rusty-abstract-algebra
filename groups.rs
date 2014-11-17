#![crate_type = "lib"]
#![crate_name = "groups"]

#![feature(globs)]

extern crate sets;
extern crate operators;

use sets::*;
use operators::*;

pub trait Magma<S: Set>: Fn<(S,S),S> {
	/*
	 * requires an operator
	 * (M x M) -> M
	 */
}

pub trait Semigroup<S: Set>: Magma<S> + IsAssociative<S,S,S> {
}

pub trait Monoid<S: Set>: Semigroup<S> + HasIdentity<S> {
}

pub trait Quasigroup<S:Set>: Magma<S> +
	IsLeftInvertible<S,S,S> + IsRightInvertible<S,S,S>
{
}

pub trait Loop<S:Set>: Quasigroup<S> + HasIdentity<S> {
}

pub trait Group<S:Set>: Monoid<S> + HasInverse<S> +
	IsLeftInvertible<S,S,S> + IsRightInvertible<S,S,S>
{
	// TODO: remove since HasInverse
	fn invert(x: S) -> S;
}

pub trait AbelianGroup<S:Set>: Group<S> + IsInvertible<S,S> {
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
