#[derive(Debug)]
pub enum OpCode {
    Constant(usize),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

pub type Value = f32;

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: ValueArray,
    lines: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: ValueArray::new(),
            lines: Vec::new(),
        }
    }

    // Use `&mut self` for methods that modify the instance.
    pub fn write_chunk(&mut self, byte: OpCode, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    // `name` should be of type `&str`.
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==\n", name);
        for (offset, instruction) in self.code.iter().enumerate() {
            self.disassemble_instruction(instruction, offset);
        }
    }

    fn disassemble_instruction(&self, instruction: &OpCode, offset: usize) {
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ")
        } else {
            println!("{}", self.lines[offset])
        }

        match instruction {
            OpCode::Constant(index) => return self.constant_instruction("Constant", &index),
            OpCode::Add => return self.simple_instruction("Add"),
            OpCode::Subtract => return self.simple_instruction("Subtract"),
            OpCode::Multiply => return self.simple_instruction("Multiply"),
            OpCode::Divide => return self.simple_instruction("Divide"),
            OpCode::Negate => return self.simple_instruction("Negate"),
            OpCode::Return => return self.simple_instruction("Return"),
            _ => {
                println!("Unknown opcode {:#?}\n", instruction);
            }
        }
    }

    fn constant_instruction(&self, name: &str, index: &usize) {
        println!(
            "{} idx: {} val: {}",
            name, index, self.constants.values[*index]
        );
    }

    // `name` should be of type `&str`.
    fn simple_instruction(&self, name: &str) {
        println!("{}", name);
    }
}
