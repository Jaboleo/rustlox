mod chunk;
mod vm;

use chunk::{Chunk, OpCode};
use vm::VM;

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new();
    chunk.constants.values.push(1.2);
    chunk.write_chunk(OpCode::Constant(chunk.constants.len() - 1), 123);
    // chunk.write_constant(chunk.constants.len() - 1);

    chunk.constants.values.push(3.4);
    chunk.write_chunk(OpCode::Constant(chunk.constants.len() - 1), 123);

    chunk.write_chunk(OpCode::Add, 123);

    chunk.constants.values.push(5.6);
    chunk.write_chunk(OpCode::Constant(chunk.constants.len() - 1), 123);

    chunk.write_chunk(OpCode::Divide, 123);
    chunk.write_chunk(OpCode::Negate, 123);
    chunk.write_chunk(OpCode::Return, 123);
    chunk.disassemble_chunk("test chunk");
    vm.interpret(chunk);
}
