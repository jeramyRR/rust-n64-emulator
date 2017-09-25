/// MIPS VR3000 cpu emulator
/// Datasheet: http://datasheets.chipdb.org/NEC/Vr-Series/Vr43xx/U10504EJ7V0UMJ1.pdf

use coprocessor0::Cp0;

const NUM_GRP_REGS: usize = 32; // Number of general purpose registers

#[derive(Debug, Default)]
pub struct Cpu {
  // 64-bit General Purpose Registers
  reg_gprs: [u64; NUM_GRP_REGS], // general purpose registers
  reg_fprs: [f64; NUM_GRP_REGS], // floating point operation registers

  // Special Registers
  reg_pc: u64,     // Program Counter register
  reg_hi: u64,     // Integer multiply & divide high order doubleword result
  reg_lo: u64,     // Integer multiply & divide low order doubleword result
  reg_llbit: bool, // Load/Link LLBit registers
  reg_fcr0: f32,   // Implementation/Revision register
  reg_fcr31: f32,  // Control/Status register

  cp0: Cp0,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu::default()
  }

  pub fn power_on_reset(&mut self) {
    self.cp0.power_on_reset();
  }
}
