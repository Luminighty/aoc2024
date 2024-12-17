pub type Op = u8;
pub type Num = i64;

const DEBUG: bool = false;

pub type Program = Vec<Op>;

#[derive(Debug)]
pub struct Device {
    a: Num,
    b: Num,
    c: Num,
    ip: usize,
    pub output: Vec<Op>,
}

impl Device {
    pub fn new(a: Num, b: Num, c: Num) -> Self {
        Self {
            a,
            b,
            c,
            output: Vec::new(),
            ip: 0,
        }
    }

    pub fn step(&mut self, program: &Program) -> Result<(), DeviceErr> {
        let op = program.get(self.ip).ok_or(DeviceErr::Halt)?;
        let op = Opcode::try_from(*op)?;
        match op {
            Opcode::ADV => self.adv(program),
            Opcode::BXL => self.bxl(program),
            Opcode::BST => self.bst(program),
            Opcode::JNZ => self.jnz(program),
            Opcode::BXC => self.bxc(program),
            Opcode::OUT => self.out(program),
            Opcode::BDV => self.bdv(program),
            Opcode::CDV => self.cdv(program),
        }
    }

    fn literal(&self, program: &Program) -> Result<Num, DeviceErr> {
        match program.get(self.ip + 1) {
            Some(x) if *x < 8 => Ok(*x as Num),
            Some(op) => Err(DeviceErr::UndefinedLiteralOperand(*op)),
            None => Err(DeviceErr::Halt),
        }
    }

    fn combo(&self, program: &Program) -> Result<Num, DeviceErr> {
        match program.get(self.ip + 1) {
            Some(x) if *x < 4 => Ok(*x as Num),
            Some(4) => Ok(self.a),
            Some(5) => Ok(self.b),
            Some(6) => Ok(self.c),
            Some(op) => Err(DeviceErr::UndefinedComboOperand(*op)),
            None => Err(DeviceErr::Halt),
        }
    }

    fn adv(&mut self, program: &Program) -> Result<(), DeviceErr> {
        let num = self.a;
        let op = self.combo(program)?;
        let denom = Num::pow(2, op as u32);
        self.a = num / denom;
        self.ip += 2;
        Ok(())
    }

    fn bxl(&mut self, program: &Program) -> Result<(), DeviceErr> {
        let rhs = self.literal(program)?;
        self.b = self.b ^ rhs;
        self.ip += 2;
        Ok(())
    }

    fn bst(&mut self, program: &Program) -> Result<(), DeviceErr> {
        let op = self.combo(program)?;
        self.b = op & 0b111;
        if DEBUG {
            println!("BST B {} <= {op} % 8", self.b);
        }
        self.ip += 2;
        Ok(())
    }

    fn jnz(&mut self, program: &Program) -> Result<(), DeviceErr> {
        if self.a == 0 {
            self.ip += 2;
            return Ok(());
        };
        let target = self.literal(program)?;
        self.ip = target as usize;
        Ok(())
    }

    fn bxc(&mut self, program: &Program) -> Result<(), DeviceErr> {
        self.literal(program)?;
        self.b = self.b ^ self.c;
        self.ip += 2;
        Ok(())
    }

    fn out(&mut self, program: &Program) -> Result<(), DeviceErr> {
        let op = self.combo(program)? % 8;
        self.output.push(op as Op);
        self.ip += 2;
        Ok(())
    }

    fn bdv(&mut self, program: &Program) -> Result<(), DeviceErr> {
        let num = self.a;
        let op = self.combo(program)?;
        let denom = Num::pow(2, op as u32);
        self.b = num / denom;
        self.ip += 2;
        Ok(())
    }

    fn cdv(&mut self, program: &Program) -> Result<(), DeviceErr> {
        let num = self.a;
        let op = self.combo(program)?;
        let denom = Num::pow(2, op as u32);
        self.c = num / denom;
        self.ip += 2;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DeviceErr {
    Halt,
    UndefinedComboOperand(Op),
    UndefinedLiteralOperand(Op),
    UndefinedOpcode(Op),
}

#[derive(Debug)]
pub enum Opcode {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}

impl TryFrom<Op> for Opcode {
    type Error = DeviceErr;

    fn try_from(value: Op) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::ADV),
            1 => Ok(Opcode::BXL),
            2 => Ok(Opcode::BST),
            3 => Ok(Opcode::JNZ),
            4 => Ok(Opcode::BXC),
            5 => Ok(Opcode::OUT),
            6 => Ok(Opcode::BDV),
            7 => Ok(Opcode::CDV),
            x => Err(DeviceErr::UndefinedOpcode(x)),
        }
    }
}

#[cfg(test)]
mod tests {

    fn run(device: &mut Device, program: Program) -> bool {
        loop {
            match device.step(&program) {
                Err(DeviceErr::Halt) => {
                    return true;
                }
                Err(err) => {
                    println!("{err:?}");
                    return false;
                }
                _ => {}
            }
        }
    }

    use super::*;
    #[test]
    pub fn test_examples() {
        let mut device = Device::new(0, 0, 9);
        assert!(run(&mut device, vec![2, 6]));
        assert_eq!(1, device.b);

        let mut device = Device::new(10, 0, 0);
        assert!(run(&mut device, vec![5, 0, 5, 1, 5, 4]));
        assert_eq!(vec![0, 1, 2], device.output);

        let mut device = Device::new(2024, 0, 0);
        assert!(run(&mut device, vec![0, 1, 5, 4, 3, 0]));
        assert_eq!(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], device.output);
        assert_eq!(0, device.a);
    }
}
