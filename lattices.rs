#![crate_type = "lib"]
#![crate_name = "lattices"]

#![feature(globs)]

extern crate sets;
extern crate operators;
extern crate groups;
extern crate rings;

use sets::*;
use operators::*;
use groups::*;
use rings::*;

pub trait JoinSemilattice<S:Set + PartiallyOrdered>: Fn<(S,S),bool>
	+ IsCommutative<S,bool> + IsAssociative<S,S,S> + IsIdempotent<S>
{
	// has a least upper bound
}


pub trait MeetSemilattice<S:Set + PartiallyOrdered>: Fn<(S,S),bool>
	+ IsCommutative<S,bool> + IsAssociative<S,S,S> + IsIdempotent<S>
{
	// has a greatest lower bound
}

pub trait Lattice<S>: JoinSemilattice<S> + MeetSemilattice<S> {
}
