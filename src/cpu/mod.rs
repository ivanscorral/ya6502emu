pub mod registers;

pub struct Cpu {
    pub registers: registers::Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: registers::Registers::new(),
        }
    }
}
