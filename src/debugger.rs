use crate::chip;

pub struct CpuState {
    pub asm: String,
}

impl CpuState {
    pub fn new() -> CpuState {
        CpuState {
            asm: "".to_string(),
        }
    }

    pub fn show_cpu_state(
        &mut self,
        chip: &chip::Chip,
        digit1: u16,
        digit2: u16,
        digit3: u16,
        digit4: u16,
    ) {
        match (digit1, digit2, digit3, digit4) {
            (0, _, _, 0xE) => self.asm = "RET".to_owned(),
            (0, _, _, _) => self.asm = "CLR".to_owned(),
            (1, _, _, _) => self.asm = format!("JMP {:X}{:X}{:X}", digit2, digit3, digit4),
            (2, _, _, _) => self.asm = format!("CALL {:X}{:X}{:X}", digit2, digit3, digit4),
            (3, _, _, _) => self.asm = format!("SE {:X}, {:X}{:X}", digit2, digit3, digit4),
            (_, _, _, _) => {}
        }

        println!("0{:X}: {}", chip.pc, self.asm);
    }
}
