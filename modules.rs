#![crate_type = "lib"]
#![crate_name = "module"]

#![feature(globs)]

extern crate sets;
extern crate operators;
extern crate groups;
extern crate rings;

use sets::*;
use operators::*;
use groups::*;
use rings::*;

pub trait ScalarMultiplier<R:Set,M:Set>:
	Fn<(R,M),M> + IsAdditive<R,M,M> + HasIdentity<R> + IsAssociative<R,M,M>
{
}

pub trait LeftModule<R,M, RAdd, RMul,
		MAdd:    AbelianGroup<M>,
		RAddMul: CommutativeRing<R, RAdd, RMul>,
		RMMul:   ScalarMultiplier<R,M> + LeftDistributesOver<R,M,M,MAdd>>:
	Tuple3<
		RMMul,
		MAdd,
		RAddMul>
{
}

pub trait RightModule<R,M, RAdd, RMul,
		MAdd:    AbelianGroup<M>,
		RAddMul: CommutativeRing<R, RAdd, RMul>,
		RMMul:   ScalarMultiplier<R,M> + RightDistributesOver<R,M,M,MAdd>>:
	Tuple3<
		RMMul,
		MAdd,
		RAddMul>
{
}

pub trait Module<R,M, RAdd, RMul, MAdd, RAddMul, RMMul>:
	LeftModule<R,M, RAdd, RMul, MAdd, RAddMul, RMMul> + 
	RightModule<R,M, RAdd, RMul, MAdd, RAddMul, RMMul>
{
}

pub trait OmegaGroup<S, G: Group<S>,
		Omega: DistributesOver<Omega,S,S,G>>
{
}

pub trait VectorSpace<R,M, RAdd, RMul,
		MAdd:    AbelianGroup<M>,
		RAddMul: Field<R, RAdd, RMul>,
		RMMul:   ScalarMultiplier<R,M> + RightDistributesOver<R,M,M,MAdd>
			+ LeftDistributesOver<R,M,M,RAdd>>:
	Tuple3<
		RMMul,
		MAdd,
		RAddMul>
{
}
