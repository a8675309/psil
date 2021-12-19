use crate::{evals_and_eq, fails_eval};
use crate::val::Val::{Number, List, Boolean, StringV};
use crate::tests::{eval, parse};

#[test]
fn list() {
	evals_and_eq!("(list)", List(vec![]));
	evals_and_eq!("(list 1)", List(vec![Number(1.0)]));
	evals_and_eq!("(list 1 false \"pee\")", List(vec![Number(1.0), Boolean(false), StringV("pee".to_string())]));
}

#[test]
fn list_len() {
	evals_and_eq!("(list-len (list))", Number(0.0));
	evals_and_eq!("(list-len (list 5))", Number(1.0));
	evals_and_eq!("(list-len (list 9 0))", Number(2.0));
	fails_eval!("(list-len 5)");
	fails_eval!("(list-len (list 9 0) (list 9 0))");
}

#[test]
fn list_range() {
	evals_and_eq!("(list-range 1 2)", List(vec![Number(1.0), Number(2.0)]));
	evals_and_eq!("(list-range 2 5)", List(vec![Number(2.0), Number(3.0), Number(4.0), Number(5.0)]));
	evals_and_eq!("(list-range 2 8 2)", List(vec![Number(2.0), Number(4.0), Number(6.0), Number(8.0)]));
	fails_eval!("(list-range)");
	fails_eval!("(list-range 1)");
	fails_eval!("(list-range 1 1 1 1)");
}
