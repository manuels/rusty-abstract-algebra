#![crate_type = "lib"]
#![crate_name = "quaternions"]

#![feature(globs)]

extern crate sets;
extern crate operators;
extern crate groups;
extern crate rings;
extern crate module;
extern crate algebra;

use sets::*;
use operators::*;
use groups::*;
use rings::*;
use module::*;
use algebra::*;

use std::num::{Num, NumCast};

pub struct Quaternion<V:Num> {
	r: V,
	i: V,
	j: V,
	k: V,
}

impl<S:Num> Set for Quaternion<S> {}

impl<Scalar:Num> Quaternion<Scalar> {
	pub fn new(r:Scalar, i:Scalar, j:Scalar, k:Scalar) -> Quaternion<Scalar> {
		Quaternion {
			r: r,
			i: i,
			j: j,
			k: k,
		}
	}
}

pub struct SAdd<S>;
impl<S:Num+Set> IsAssociative<S,S,S> for SAdd<S> {}
impl<S:Num+Set> IsCommutative<S,S> for SAdd<S> {}
impl<S:Num+Set> Monoid<S> for SAdd<S> {}
impl<S:Num+Set> Semigroup<S> for SAdd<S> {}
impl<S:Num+Set> Group<S> for SAdd<S> {}
impl<S:Num+Set> AbelianGroup<S> for SAdd<S> {}
impl<S:Num+Set> RightDistributesOver<S,S,SAdd<S>> for SMul<S> {}
impl<S:Num+Set> LeftDistributesOver<S,S,SAdd<S>> for SMul<S> {}
impl<S:Num+Set> DistributesOver<S,S,SAdd<S>> for SMul<S> {}

impl<S:Num+Set> Fn<(S,S),S> for SAdd<S> {
	extern "rust-call" fn call(&self, (a,b): (S,S)) -> S {
		a*b
	}
}
impl<S:Num+Set> HasIdentity<S> for SAdd<S> {
	fn identity() -> S {
		std::num::zero()
	}
}
impl<S:Num+Set> HasInverse<S> for SAdd<S> {
	fn inverse(a:S) -> S {
		-a
	}
}
impl<S:Num+Set> IsLeftInvertible<S,S,S> for SAdd<S> {
	fn left_invert(a:S, b:S) -> S {
		a-b
	}
}
impl<S:Num+Set> IsRightInvertible<S,S,S> for SAdd<S> {
	fn right_invert(a:S, b:S) -> S {
		a-b
	}
}
impl<S:Num+Set> IsInvertible<S,S> for SAdd<S> {
	fn invert(a:S, b:S) -> S {
		a-b
	}
}

pub struct SMul<S>;
impl<S:Num+Set> Fn<(S,S),S> for SMul<S> {
	extern "rust-call" fn call(&self, args: (S,S)) -> S {
		let (a,b) = args;
		a*b
	}
}

impl<S:Num+Set> Monoid<S> for SMul<S> {}
impl<S:Num+Set> Semigroup<S> for SMul<S> {}
impl<S:Num+Set> IsAssociative<S,S,S> for SMul<S> {}
impl<S:Num+Set> IsCommutative<S,S> for SMul<S> {}
impl<S:Num+Set> HasZeroProduct<S> for SMul<S> {}

impl<S:Num+Set> HasIdentity<S> for SMul<S> {
	fn identity() -> S {
		std::num::one()
	}
}

pub struct SField<S>;
impl<S:Num+Set> Ring<S, SAdd<S>, SMul<S>> for SField<S> {}
impl<S:Num+Set> CommutativeRing<S, SAdd<S>, SMul<S>> for SField<S> {}
impl<S:Num+Set> Domain<S, SAdd<S>, SMul<S>> for SField<S> {}
impl<S:Num+Set> IntegralDomain<S, SAdd<S>, SMul<S>> for SField<S> {}
impl<S:Num+Set> Field<S, SAdd<S>, SMul<S>> for SField<S> {}

impl<S:Num+Set> IsLeftInvertible<S,S,S> for SMul<S> {
	fn left_invert(a:S, b:S) -> S {
		a/b
	}
}
impl<S:Num+Set> IsRightInvertible<S,S,S> for SMul<S> {
	fn right_invert(a:S, b:S) -> S {
		a/b
	}
}
impl<S:Num+Set> IsInvertible<S,S> for SMul<S> {
	fn invert(a:S, b:S) -> S {
		a/b
	}
}

pub struct VAdd<V>;
impl<V:Num> Monoid<Quaternion<V>> for VAdd<V> {}
impl<V:Num> Semigroup<Quaternion<V>> for VAdd<V> {}
impl<V:Num> Group<Quaternion<V>> for VAdd<V> {}
impl<V:Num> AbelianGroup<Quaternion<V>> for VAdd<V> {}

impl<V:Num> IsAssociative<Quaternion<V>,Quaternion<V>,Quaternion<V>> for VAdd<V> {}
impl<V:Num> IsCommutative<Quaternion<V>,Quaternion<V>> for VAdd<V> {}

impl<V:Num> Fn<(Quaternion<V>,Quaternion<V>),Quaternion<V>> for VAdd<V> {
	extern "rust-call" fn call(&self, args: (Quaternion<V>,Quaternion<V>)) -> Quaternion<V> {
		let (a,b) = args;
		Quaternion::new(
			a.r + b.r,
			a.i + b.i,
			a.j + b.j,
			a.k + b.k,
		)
	}
}

impl<V:Num> HasIdentity<Quaternion<V>> for VAdd<V> {
	fn identity() -> Quaternion<V> {
		Quaternion::new(
			std::num::zero(),
			std::num::zero(),
			std::num::zero(),
			std::num::zero(),
		)
	}
}

impl<V:Num> HasInverse<Quaternion<V>> for VAdd<V> {
	fn inverse(q: Quaternion<V>) -> Quaternion<V> {
		Quaternion::new(
			-q.r,
			-q.i,
			-q.j,
			-q.k,
		)
	}
}

impl<V:Num> IsLeftInvertible<Quaternion<V>,Quaternion<V>,Quaternion<V>> for VAdd<V> {
	fn left_invert(q:Quaternion<V>, p:Quaternion<V>) -> Quaternion<V> {
		Quaternion::new(
			p.r-q.r,
			p.i-q.i,
			p.j-q.j,
			p.k-q.k,
		)
	}
}

impl<V:Num> IsRightInvertible<Quaternion<V>,Quaternion<V>,Quaternion<V>> for VAdd<V> {
	fn right_invert(q:Quaternion<V>, p:Quaternion<V>) -> Quaternion<V> {
		Quaternion::new(
			p.r-q.r,
			p.i-q.i,
			p.j-q.j,
			p.k-q.k,
		)
	}
}

impl<V:Num> IsInvertible<Quaternion<V>,Quaternion<V>> for VAdd<V> {
	fn invert(q:Quaternion<V>, p:Quaternion<V>) -> Quaternion<V> {
		Quaternion::new(
			p.r-q.r,
			p.i-q.i,
			p.j-q.j,
			p.k-q.k,
		)
	}
}

pub struct SVMul<S:Num+Set+NumCast,V:Num+NumCast>;
impl<S:Num+Set+NumCast,V:Num+NumCast> IsAdditive<S,Quaternion<V>,Quaternion<V>> for SVMul<S,V> {}
impl<S:Num+Set+NumCast,V:Num+NumCast> IsAssociative<S,Quaternion<V>,Quaternion<V>> for SVMul<S,V> {}
impl<S:Num+Set+NumCast,V:Num+NumCast> ScalarMultiplier<S,Quaternion<V>> for SVMul<S,V> {}
impl<S:Num+Set+NumCast,V:Num+NumCast> LeftDistributesOver<S,Quaternion<V>, VAdd<V>> for SVMul<S,V> {}
impl<S:Num+Set+NumCast,V:Num+NumCast> RightDistributesOver<S, Quaternion<V>, SAdd<S>> for SVMul<S,V> {}

impl<S:Num+Set+NumCast,V:Num+NumCast> Fn<(S,Quaternion<V>),Quaternion<V>> for SVMul<S,V> {
	extern "rust-call" fn call(&self, (s,p): (S,Quaternion<V>)) -> Quaternion<V> {
		let a:V = std::num::cast(s).unwrap();
		Quaternion::new(
			a*p.r,
			a*p.i,
			a*p.j,
			a*p.k,
		)
	}
}

impl<S:Num+Set+NumCast,V:Num+NumCast> HasIdentity<S> for SVMul<S,V> {
	fn identity() -> S {
		return std::num::one();
	}
}

pub struct QuaternionSpace<V>;
impl<S:Num+Set+NumCast,V:Num+NumCast>
VectorSpace<S,Quaternion<V>, SAdd<S>, SMul<S>, VAdd<V>, SField<S>, SVMul<S,V>> for QuaternionSpace<V> {}

pub struct VMul<V>;
impl<S:Num+Set+NumCast,V:Num+NumCast> BilinearProduct<Quaternion<V>, S, VAdd<V>, SMul<S>, SVMul<S,V>> for VMul<V> {}
impl<V:Num+NumCast> RightDistributesOver<Quaternion<V>,Quaternion<V>,VAdd<V>> for VMul<V> {}
impl<V:Num+NumCast> LeftDistributesOver<Quaternion<V>,Quaternion<V>,VAdd<V>> for VMul<V> {}
impl<V:Num+NumCast> DistributesOver<Quaternion<V>,Quaternion<V>,VAdd<V>> for VMul<V> {}
impl<S:Num+NumCast+Set,V:Num+NumCast> CompatibleWith<S,Quaternion<V>, SVMul<S,V>, SMul<S>> for VMul<V> {}

impl<V:Num+NumCast> Fn<(Quaternion<V>,Quaternion<V>),Quaternion<V>> for VMul<V> {
	extern "rust-call" fn call(&self, (p,q): (Quaternion<V>,Quaternion<V>)) -> Quaternion<V> {
		Quaternion::new(
			p.r*q.r - p.i*q.i - p.j*q.j - p.k*q.k,
			p.r*q.i + p.i*q.r + p.j*q.k - p.k*q.j,
			p.r*q.j + p.j*q.i - p.i*q.k + p.k*q.i,
			p.r*q.k + p.k*q.i + p.i*q.j - p.j*q.i,
		)
	}
}

pub struct QuaternionAlgebra<V>;
impl<S:Num+NumCast+Set,V:Num+NumCast> Algebra<S, Quaternion<V>, SAdd<S>, SMul<S>, VAdd<V>, SVMul<S,V>,
		SField<S>,
		QuaternionSpace<V>,
		VMul<V>> for QuaternionAlgebra<V>
{}

impl<V:Num+std::fmt::Show> std::fmt::Show for Quaternion<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{},{},{},{}]", self.r, self.i, self.j, self.k)
    }
}
