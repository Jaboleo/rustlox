use chunk::{Chunk, OpCode, Value};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: [Value; 256],
    stack_top: usize
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            ip: 0,
            stack: [0.0 ;256],
            stack_top: 0
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        println!("{:?}", self.stack);
        while self.ip < self.chunk.code.len() {
            let instruction = &self.chunk.code[self.ip];
            self.ip += 1;
            match instruction {
                OpCode::Constant(value_index) => {
                    let constant = self.chunk.constants.values[*value_index];
                    self.push(constant);
                },
                OpCode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b)
                }
                OpCode::Subtract => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b)
                },
                OpCode::Multiply => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b)
                },
                OpCode::Divide => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a / b)
                },
                OpCode::Negate => {
                    let negated = -self.pop();
                    self.push(negated);
                },
                OpCode::Return => {
                    println!("{}", self.pop());
                    return InterpretResult::Ok
                },
                _ => println!("Unknown opcode"),
            }
        }
        InterpretResult::RuntimeError
    }

    fn reset_stack(&mut self){
        self.stack_top = 0;
    }

    fn push(&mut self, value: Value){
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value{
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

}
