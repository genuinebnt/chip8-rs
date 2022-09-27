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
            (1, _, _, _) => self.asm = format!("JMP   {:X}{:X}{:X}", digit2, digit3, digit4),
            (2, _, _, _) => self.asm = format!("CALL  {:X}{:X}{:X}", digit2, digit3, digit4),
            (3, _, _, _) => self.asm = format!("SE    V{:X}, {:X}{:X}", digit2, digit3, digit4),
            (4, _, _, _) => self.asm = format!("SNE   V{:X}, {:X}{:X}", digit2, digit3, digit4),
            (5, _, _, _) => self.asm = format!("SE    V{:X}, V{:X}", digit2, digit3),
            (6, _, _, _) => self.asm = format!("LD    V{:X}, {:X}{:X}", digit2, digit3, digit4),
            (7, _, _, _) => self.asm = format!("ADD   V{:X}, {:X}{:X}", digit2, digit3, digit4),
            (8, _, _, 0) => self.asm = format!("LD    V{:X}, V{:X}", digit2, digit3),
            (8, _, _, 1) => self.asm = format!("OR    V{:X}, V{:X}", digit2, digit3),
            (8, _, _, 2) => self.asm = format!("AND   V{:X}, V{:X}", digit2, digit3),
            (8, _, _, 3) => self.asm = format!("XOR   V{:X}, V{:X}", digit2, digit3),
            (8, _, _, 4) => self.asm = format!("ADD   V{:X}, V{:X}", digit2, digit3),
            (8, _, _, 5) => self.asm = format!("SUB   V{:X}, V{:X}", digit2, digit3),
            (8, _, _, 6) => self.asm = format!("SHR   V{:X}, V{:X}", digit2, digit3),
            (8, _, _, 7) => self.asm = format!("SUBN  V{:X}, V{:X}", digit2, digit3),
            (8, _, _, 0xE) => self.asm = format!("SHL  V{:X}, V{:X}", digit2, digit3),
            (9, _, _, _) => self.asm = format!("SNE  V{:X}, V{:X}", digit2, digit3),
            (0xA, _, _, _) => self.asm = format!("LD    I, {:X}{:X}{:X}", digit2, digit3, digit4),
            (0xB, _, _, _) => self.asm = format!("JP    V0, {:X}{:X}{:X}", digit2, digit3, digit4),
            (0xC, _, _, _) => self.asm = format!("RND   V{:X}, {:X}{:X}", digit2, digit3, digit4),
            (0xD, _, _, _) => {
                self.asm = format!("DRW   V{:X}, V{:X}, {:X}", digit2, digit3, digit4)
            }
            (0xE, _, 9, 0xE) => self.asm = format!("SKP   V{:X}", digit2),
            (0xE, _, 0xA, 1) => self.asm = format!("SKNP  V{:X}", digit2),
            (0xF, _, 0, 7) => self.asm = format!("LD    V{:X}, DT", digit2),
            (0xF, _, 0, 0xA) => self.asm = format!("LD    V{:X}, K", digit2),
            (0xF, _, 1, 5) => self.asm = format!("LD    DT, V{:X}", digit2),
            (0xF, _, 1, 8) => self.asm = format!("LD    ST, V{:X}", digit2),
            (0xF, _, 1, 0xE) => self.asm = format!("ADD   I, V{:X}", digit2),
            (0xF, _, 2, 9) => self.asm = format!("LD    F, V{:X}", digit2),
            (0xF, _, 3, 3) => self.asm = format!("LD    B, V{:X}", digit2),
            (0xF, _, 5, 5) => self.asm = format!("LD    [I], V{:X}", digit2),
            (0xF, _, 6, 5) => self.asm = format!("LD    V{:X}, [I]", digit2),
            (_, _, _, _) => unreachable!("Ran unimplemented instruction {:X}", chip.opcode),
        }

        println!("0{:X}: {}", chip.pc, self.asm);
    }
}
