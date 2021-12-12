use crate::{evals_and_eq, evals_and_eq_with_env, fails_eval};
use crate::val::Val::{Boolean, Number, Symbol};
use crate::val::void;

use crate::environment::Environment;
use crate::tests::{eval, eval_with_env, parse};

#[test]
fn if_test() {
	evals_and_eq!("(if true true false)", Boolean(true));
	evals_and_eq!("(if true 5 (/ 10 0))", Number(5.0));
	fails_eval!("(if false 5 (/ 10 0))");
}

#[test]
fn cond() {
	// TODO: test failure conditions
	evals_and_eq!("(cond false 1.0 false 2.0 true 3.0)", Number(3.0));
	evals_and_eq!("(cond false 1.0 false 2.0 true 3.0 true (/ 4.0 0))", Number(3.0));
}

#[test]
fn define() {
	// TODO: test failure conditions
	let mut env = Environment::new();
	evals_and_eq_with_env!("(define x 5)", void(), env);
	evals_and_eq_with_env!("x", Number(5.0), env);

	let mut env2= Environment::new();
	evals_and_eq_with_env!("(define add {|a b| (+ a b)})", void(), env2);
	evals_and_eq_with_env!("(add 3 2)", Number(5.0), env2);
}

#[test]
fn do_test() {
	// I'm not really sure what else to test here.
	evals_and_eq!("(do (+ 5 5) (- 5 9))", void());
}

#[test]
fn fail() {
	fails_eval!("(fail)");
	fails_eval!("(fail \"Fail message\")");
	fails_eval!("(fail 5)"); // failing (fail)
}

#[test]
fn exit() {
	// TODO: not sure how to test this lol, could fork
	fails_eval!("(exit \"bad type test\")");
}

#[test]
fn type_test() {
	evals_and_eq!("(type 5)", Symbol("number".to_string()));
	evals_and_eq!("(type false)", Symbol("boolean".to_string()));
	evals_and_eq!("(type \"false\")", Symbol("string".to_string()));
	evals_and_eq!("(type (type \"false\"))", Symbol("symbol".to_string()));
	evals_and_eq!("(type type)", Symbol("procedure".to_string()));

	fails_eval!("(type)");
	fails_eval!("(type 5 5)");
}