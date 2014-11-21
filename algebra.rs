#![crate_type = "lib"]
#![crate_name = "algebra"]

#![feature(globs)]

extern crate sets;
extern crate operators;
extern crate groups;
extern crate rings;
extern crate module;

use sets::*;
use operators::*;
use groups::*;
use rings::*;
use module::*;

pub trait BilinearProduct<V, S, VAdd, SMul, SVMul>:
	Fn<(V,V),V> + DistributesOver<V,V,VAdd> + CompatibleWith<S,V, SVMul, SMul>
{
}

pub trait Algebra<S, V, SAdd, SMul, VAdd, SVMul,
		K: Field<S, SAdd, SMul>,
		A: VectorSpace<S, V, SAdd, SMul, VAdd, K, SVMul>,
		VMul: BilinearProduct<V,S,VAdd,SMul,SVMul>>:
{}
