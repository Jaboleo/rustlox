mod chunk;
mod scanner;
mod compiler;
mod vm;

use std::env;
use std::fs;
use std::io;
use std::process;
use vm::{InterpretResult, VM};

fn repl(vm: &mut VM) {
    loop {
        println!("> ");
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).expect("Read line failed.");
        vm.interpret(line);
    }
}

fn run_file(vm: &mut VM, path: &String) {
    let source: String = fs::read_to_string(path).expect("Should have been able to read the file");
    let result: InterpretResult = vm.interpret(source);

    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        _ => {
            println!("Unhandled result")
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vm: VM = VM::new();
    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &args[1]),
        _ => {
            println!("Usage: clox [path]\n");
            process::exit(64);
        }
    }
}
