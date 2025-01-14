use std::borrow::Borrow;
use crate::{check_arity_is, fail_on_bad_type};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Boolean, StringV as StringVal, Symbol};

pub fn add_symbol_procs(env: &mut Environment) {
	env.add_proc("str2symb".to_string(), str_to_symb);
	env.add_proc("symb2str".to_string(), symb_to_str);
	env.add_proc("is-symb?".to_string(), is_symb);
	env.add_proc("is-void?".to_string(), is_void);
}

fn str_to_symb(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str2symb", 1, rands);

	match rands[0].borrow() {
		StringVal(s) => Ok(Symbol(s.to_string())),
		_ => fail_on_bad_type!("str2symb", "string", rands)
	}
}

fn symb_to_str(rands: ValList) -> Result<Val, String> {
	check_arity_is!("symb2str", 1, rands);

	match rands[0].borrow() {
		Symbol(s) => Ok(StringVal(s.to_string())),
		_ => fail_on_bad_type!("symb2str", "symbol", rands)
	}
}

fn is_symb(rands: ValList) -> Result<Val, String> {
	check_arity_is!("is-symb?", 1, rands);

	match rands[0] {
		Symbol(_) => Ok(Boolean(true)),
		_ => Ok(Boolean(false))
	}
}

fn is_void(rands: ValList) -> Result<Val, String> {
	check_arity_is!("is-void?", 1, rands);

	match rands[0].borrow() {
		Symbol(s) => Ok(Boolean(s.eq("void"))),
		_ => Ok(Boolean(false))
	}
}
