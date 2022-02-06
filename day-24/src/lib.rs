type Register = char;

#[derive(Debug)]
enum Instruction {
    Inp(Register),
    Add(Register, Register),
    AddLiteral(Register, isize),
    Mul(Register, Register),
    MulLiteral(Register, isize),
    Div(Register, Register),
    DivLiteral(Register, isize),
    Mod(Register, Register),
    ModLiteral(Register, isize),
    Eql(Register, Register),
    EqlLiteral(Register, isize),
}

impl Instruction {
    pub fn new(instruction: &str) -> Self {
        let mut parts = instruction.split(" ");
        let fun = parts.next().unwrap();

        match fun {
            "inp" => Instruction::Inp(parts.next().unwrap().chars().nth(0).unwrap()),
            _ => {
                let a = parts.next().unwrap().chars().nth(0).unwrap();
                let b_str = parts.next().unwrap();
                match b_str.parse::<isize>() {
                    Ok(b) => match fun {
                        "add" => Instruction::AddLiteral(a, b),
                        "mul" => Instruction::MulLiteral(a, b),
                        "div" => Instruction::DivLiteral(a, b),
                        "mod" => Instruction::ModLiteral(a, b),
                        "eql" => Instruction::EqlLiteral(a, b),
                        _ => panic!("no such instruction on this machine"),
                    },
                    _ => {
                        let b = b_str.chars().next().unwrap();
                        match fun {
                            "add" => Instruction::Add(a, b),
                            "mul" => Instruction::Mul(a, b),
                            "div" => Instruction::Div(a, b),
                            "mod" => Instruction::Mod(a, b),
                            "eql" => Instruction::Eql(a, b),
                            _ => panic!("no such instruction on this machine"),
                        }
                    }
                }
            }
        }
    }
}

pub struct ALU {
    pub w: isize,
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl ALU {
    pub fn new() -> Self {
        ALU {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn reset(&mut self) {
        self.w = 0;
        self.x = 0;
        self.y = 0;
        self.z = 0;
    }

    pub fn execute(&mut self, program: &str, inputs: &[isize]) -> Result<isize, &'static str> {
        let mut inputs_iter = inputs.iter();

        program
            .trim()
            .split("\n")
            .map(|line| Instruction::new(line.trim()))
            .try_for_each(|instruction| self.execute_instruction(instruction, &mut inputs_iter))?;

        Ok(self.z)
    }

    fn read(&self, register: Register) -> isize {
        match register {
            'w' => self.w,
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => panic!("no such register in this machine"),
        }
    }

    fn write(&mut self, register: Register, value: isize) {
        match register {
            'w' => {
                self.w = value;
            }
            'x' => {
                self.x = value;
            }
            'y' => {
                self.y = value;
            }
            'z' => {
                self.z = value;
            }
            _ => panic!("no such register in this machine"),
        }
    }

    fn execute_instruction<'a, I>(
        &mut self,
        instruction: Instruction,
        inputs_iter: &mut I,
    ) -> Result<(), &'static str>
    where
        I: Iterator<Item = &'a isize>,
    {
        match instruction {
            Instruction::Inp(register) => {
                if let Some(value) = inputs_iter.next() {
                    self.write(register, *value)
                } else {
                    return Err("Out of inputs");
                }
            }
            Instruction::Add(register_a, register_b) => {
                return self.execute_instruction(
                    Instruction::AddLiteral(register_a, self.read(register_b)),
                    inputs_iter,
                )
            }
            Instruction::Mul(register_a, register_b) => {
                return self.execute_instruction(
                    Instruction::MulLiteral(register_a, self.read(register_b)),
                    inputs_iter,
                )
            }
            Instruction::Div(register_a, register_b) => {
                return self.execute_instruction(
                    Instruction::DivLiteral(register_a, self.read(register_b)),
                    inputs_iter,
                )
            }
            Instruction::Mod(register_a, register_b) => {
                return self.execute_instruction(
                    Instruction::ModLiteral(register_a, self.read(register_b)),
                    inputs_iter,
                )
            }
            Instruction::Eql(register_a, register_b) => {
                return self.execute_instruction(
                    Instruction::EqlLiteral(register_a, self.read(register_b)),
                    inputs_iter,
                )
            }
            Instruction::AddLiteral(register, value) => {
                self.write(register, self.read(register) + value)
            }
            Instruction::MulLiteral(register, value) => {
                self.write(register, self.read(register) * value)
            }
            Instruction::DivLiteral(register, value) => {
                self.write(register, self.read(register) / value)
            }
            Instruction::ModLiteral(register, value) => {
                self.write(register, self.read(register) % value)
            }
            Instruction::EqlLiteral(register, value) => {
                self.write(register, if self.read(register) == value { 1 } else { 0 })
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negation_program() {
        let mut alu = ALU::new();

        let program = "
            inp x
            mul x -1
        ";
        let inputs = [5];
        alu.execute(program, &inputs).unwrap();

        assert_eq!(alu.x, -5);
    }

    #[test]
    fn test_three_times_larger_program() {
        let mut alu = ALU::new();

        let program = "
            inp z
            inp x
            mul z 3
            eql z x
        ";

        let false_inputs = [1, 2];
        alu.execute(program, &false_inputs).unwrap();
        assert_eq!(alu.z, 0);

        let true_inputs = [4, 12];
        alu.execute(program, &true_inputs).unwrap();
        assert_eq!(alu.z, 1);
    }

    #[test]
    fn test_to_binary_program() {
        let mut alu = ALU::new();

        let program = "
            inp w
            add z w
            mod z 2
            div w 2
            add y w
            mod y 2
            div w 2
            add x w
            mod x 2
            div w 2
            mod w 2
        ";

        let inputs = [5];
        alu.execute(program, &inputs).unwrap();

        assert_eq!(alu.z, 1);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.w, 0);

        let inputs = [15];

        alu.reset();
        alu.execute(program, &inputs).unwrap();

        assert_eq!(alu.z, 1);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.w, 1);
    }
}
