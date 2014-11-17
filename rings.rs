#![crate_type = "lib"]
#![crate_name = "rings"]

#![feature(globs)]

extern crate sets;
extern crate operators;
extern crate groups;

use sets::*;
use operators::*;
use groups::*;

pub trait Semiring<S:Set,
		Add: Monoid<S> + HasIdentity<S> + IsCommutative<S, S>,
		Mul: Monoid<S> + HasIdentity<S> +
			DistributesOver<S,S,S,Add> +
			AnnihilatesIdentityOf<S,Add>>:
	Tuple2<Add, Mul>
{
}

pub trait Ring<S:Set,
		Add: AbelianGroup<S>,
		Mul: Monoid<S> + DistributesOver<S,S,S,Add>>:
	Tuple2<Add,Mul>
{
}

pub trait Rng<S:Set,
		Add: AbelianGroup<S>,
		Mul: Semigroup<S> + DistributesOver<S,S,S,Add>>:
	Tuple2<Add,Mul>
{
}

pub trait RightNearRing<S:Set,
		Add: Group<S>,
		Mul: Semigroup<S> + RightDistributesOver<S,S,S,Add>>:
	Tuple2<Add,Mul>
{
}

pub trait LeftNearRing<S:Set,
		Add: Group<S>,
		Mul: Semigroup<S> + LeftDistributesOver<S,S,S,Add>>:
	Tuple2<Add,Mul>
{
}

pub trait CommutativeRing<S:Set,
		Add, Mul: IsCommutative<S,S>>:
	Ring<S, Add, Mul>
{
}

pub trait Domain<S, Add, Mul:HasZeroProduct<S>>: Ring<S, Add, Mul>
{
}

pub trait IntegralDomain<S, Add, Mul>:
	Domain<S, Add, Mul> + CommutativeRing<S, Add, Mul>
{
}

pub trait Field<S, Add, Mul: IsInvertible<S,S>>:
	IntegralDomain<S, Add, Mul>
{
}
