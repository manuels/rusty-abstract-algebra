#![crate_type = "lib"]
#![crate_name = "operators"]

#![feature(globs)]

extern crate sets;

use sets::*;

pub trait IsAssociative<A,B,C>: Fn<(A,B),C> {
	/*
	 * ((a.b).c) == (a.(b.c))
	 */
}

pub trait IsCommutative<A,C>: Fn<(A,A),C> {
	/*
	 * call(a,b) == call(b,a)
	 */
}

pub trait IsLeftInvertible<A,B,C>: Fn<(A,B),C> {
	/*
	 * left divisible
	 */
	fn left_invert(a: A, b: B) -> C;
}

pub trait IsRightInvertible<A,B,C>: Fn<(A,B),C> {
	/*
	 * right divisible
	 */
	fn right_invert(a: A, B: B) -> C;
}

pub trait IsInvertible<A,C>: Fn<(A,A),C> + IsCommutative<A,C> {
	/*
	 * left and right divisible must be the same
	 */
	fn invert(a: A, b: A) -> C;
}

pub trait LeftDistributesOver<A,B, Op: Fn<(B,B),B>>: Fn<(A,B),B> {
	/*
	 * Self(a, Op(b, c)) = Op(Self(a, b), Self(a, c))
	 */
}

pub trait RightDistributesOver<A,B, Op: Fn<(A,A),A>>: Fn<(A,B),B> {
	/*
	 * Self(Op1(b, c), a) = Op2(Self(b, a), Self(c, a))

	 */
}

pub trait DistributesOver<A,B, Op: Fn<(A,B),B>>: Fn<(A,B),B> +
	LeftDistributesOver<A,B, Op> + RightDistributesOver<A,B, Op> {
	/*
	 * Self(a, Op(b, c)) = Op(Self(a, b), Self(a, c))
	 * Self(Op(b, c), a) = Op(Self(b, a), Self(c, a))
	 */
}

pub trait CompatibleWith<A,X, Op1: Fn<(A,X),X>, Op2:Fn<(A,A),A>>: Fn<(X,X),X> {
	/*
	 * Self(Op1(a,x), Op1(b, y)) = Op1(Op2(a,b), Self(x,y))
	 */
}

pub trait LeftAnnihilatesIdentityOf<S, Op: Fn<(S,S),S> + HasIdentity<S>> {
	// a*0 = 0
}

pub trait RightAnnihilatesIdentityOf<S, Op: Fn<(S,S),S> + HasIdentity<S>> {
	// 0*a = 0
}

pub trait AnnihilatesIdentityOf<S, Op: Fn<(S,S),S> + HasIdentity<S>> {
	// a*0 = 0
	// 0*a = 0
}

pub trait HasZeroProduct<S> {
	// a*b = 0 => a = 0 or b = 0
}
pub trait IsIdempotent<S>: Fn<(S,S),S> {
	// Self(a,a) == a
}

pub trait IsAdditive<A,B,C>: Fn<(A,B),C> {
	// Self(a,b) = Self(a) + Self(b)
}
