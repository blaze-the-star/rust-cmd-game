
use std::{collections::HashMap, process::exit, io::{Write, stdout}};
use console::Term;

fn cmd_action(_params:&String) -> (){
	println!("Acitoned!")
}

fn cmd_action2(_params:&String) -> (){
	println!("You did it!")
}

fn cmd_exit(_params:&String) -> (){
	exit(0);
}

fn main() {
	type FnCmd = fn(&String) -> ();

	let cmd_library:HashMap<Vec<&str>, FnCmd> = HashMap::from([
		(vec!["exit", "close", "quit"],	cmd_exit as FnCmd),
		(vec!["action", "do"],	cmd_action),
		(vec!["action2", "do2"],	cmd_action2),
	]);

	// Construct map of commands from the command library
	let mut cmd:HashMap<String, FnCmd> = HashMap::new();
	for (k, v) in cmd_library {
		for c in k {
			cmd.insert(c.to_string(), v);
		}
	}

	let term = Term::stdout();
	loop {

		print!("What do you do? ");
		stdout().flush().unwrap();
		match term.read_line() {
			Ok(line) => {
				stdout().flush().unwrap();
				if cmd.contains_key(&line) {
					cmd[&line](&line);
				} else {
					println!("That is impossible: '{line}'");
				}
			},
			Err(e) => {
				println!("Error handling input: {e}")
			},
		}

		
		
	}
}
