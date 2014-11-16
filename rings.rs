#![crate_type = "lib"]
#![crate_name = "rings"]

#![feature(globs)]

extern crate sets;
extern crate operators;
extern crate groups;

use sets::*;
use operators::*;
use groups::*;

pub trait Semiring<Self,
		Add: IsCommutative<Self, Self>,
		Mul: DistributesOver<Self,Self,Self,Add>>:
	Monoid<Self,Add> + HasIdentity<Self, Add> +
	Monoid<Self,Mul> + HasIdentity<Self, Mul>
{
}

pub trait Ring<Self, Add, Mul: DistributesOver<Self,Self,Self,Add>>:
	AbelianGroup<Self, Add> + Monoid<Self, Mul>
{
}
