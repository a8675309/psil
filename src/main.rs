use std::{env, fs};
use std::io::{self, Write};

use environment::Environment;
use eval::{eval, eval_expr};
use parser::{parse, parse_expr_entry};

pub mod ast;
pub mod environment;
pub mod eval;
pub mod native;
pub mod parser;
pub mod scanner;
pub mod val;

#[cfg(test)]
mod tests;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() >= 2 { // first arg is always the name of the executable
		load_and_interpret(&args[1])
	} else {
		repl()
	}
}

fn load_and_interpret(file_name: &String) {
	let script = fs::read_to_string(file_name);

	match script {
		Ok(s) => {
			let parse_tree = parse(s).unwrap();
			match eval(parse_tree) {
				Ok(_) => {}
				Err(e) => eprintln!("{}", e)
			}
		}
		Err(e) => eprintln!("{:?}", e)
	}
}

fn repl() {
	let mut env = Environment::new();

	loop {
		print!(">>> ");
		io::stdout().flush().unwrap();

		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();
		line = line.trim().to_string();

		if line.is_empty() {
			continue
		}

		let parse_tree = parse_expr_entry(line);

		match parse_tree {
			Ok(tree) =>
				match eval_expr(tree, &mut env) {
					Ok(val) => println!("{}", val),
					Err(e) => eprintln!("{}", e)
				}
			Err(e) => eprintln!("{:?}", e)
		}

		io::stdout().flush().unwrap();
	}
}
