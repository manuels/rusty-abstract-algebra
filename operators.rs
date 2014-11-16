#![crate_type = "lib"]
#![crate_name = "operators"]

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

pub trait LeftDistributesOver<A,B,C, Op: Fn<(A,B),C>>: Fn<(A,B),C> {
	/*
	 * Self(a, Op(b, c)) = Op(Self(a, b), Self(a, c))
	 */
}

pub trait RightDistributesOver<A,B,C, Op: Fn<(A,B),C>>: Fn<(A,B),C> {
	/*
	 * Self(Op(b, c), a) = Op(Self(b, a), Self(c, a))
	 */
}

pub trait DistributesOver<A,B,C, Op: Fn<(A,B),C>>: Fn<(A,B),C> {
	/*
	 * Self(a, Op(b, c)) = Op(Self(a, b), Self(a, c))
	 * Self(Op(b, c), a) = Op(Self(b, a), Self(c, a))
     */
}
