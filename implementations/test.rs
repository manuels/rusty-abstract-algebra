#![feature(globs)]
#![feature(overloaded_calls)]

extern crate quaternions;

use quaternions::*;

fn main() {
	let a: QuaternionAlgebra<f32>;
	let qi:Quaternion<f32> = Quaternion::new(0.0, 1.0, 0.0, 0.0);
	let qj:Quaternion<f32> = Quaternion::new(0.0, 0.0, 1.0, 0.0);
	let qk:Quaternion<f32> = Quaternion::new(0.0, 0.0, 0.0, 1.0);

	println!("{}", VMul(qi,qi));
	println!("{}", VMul(qj,qj));
	println!("{}", VMul(qk,qk));
}
