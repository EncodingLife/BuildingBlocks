use super::instruction::Instruction;

pub struct InstructionSet {
    instructions: Vec<Instruction>,
}

impl InstructionSet {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions: instructions.clone(),
        }
    }

    pub fn next(&mut self) -> Instruction {
        match self.instructions.pop() {
            Some(i) => i,
            None => Instruction::default(),
        }
    }
}
