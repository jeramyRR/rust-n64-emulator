/// First of four MIPS coprocessors (CP0 - CP3).
/// This is an internal system control coprocessor
/// used to convert virtual memory addresses to
/// physical memory addresses, and for exception
/// processing.
/// Datasheet: http://datasheets.chipdb.org/NEC/Vr-Series/Vr43xx/U10504EJ7V0UMJ1.pdf
/// Datasheet: Chapter 1, pages 44 - 46

#[derive(Debug)]
enum RegConfigEP {
  D,
  DxxDxx,
  RFU,
}

impl Default for RegConfigEP {
  fn default() -> RegConfigEP {
    RegConfigEP::D
  }
}

#[derive(Debug)]
enum RegConfigBE {
  LittleEndian,
  BigEndian,
}

impl Default for RegConfigBE {
  fn default() -> RegConfigBE {
    RegConfigBE::BigEndian
  }
}

#[derive(Debug, Default)]
struct RegConfig {
  reg_config_ep: RegConfigEP,
  reg_config_be: RegConfigBE,
}

impl RegConfig {
  fn new() -> RegConfig {
    RegConfig::default()
  }

  fn power_on_reset(&mut self) {
    self.reg_config_be = RegConfigBE::default();
    self.reg_config_ep = RegConfigEP::default();
  }
}

#[derive(Debug, Default)]
pub struct Cp0 {
  reg_config: RegConfig
}

impl Cp0 {
  pub fn new() -> Cp0 {
    Cp0::default()
  }

  /// Power-ON Reset completely resets the processor
  /// * TS, SR, and RP bits of reg_status are set to 0
  /// * EP(3:0) of reg_config are set to 0
  /// * ERL and REV bits of reg_status are set to 1
  /// * Upper limit value (31) assigned to reg_random
  pub fn power_on_reset(&mut self) {
    self.reg_config.power_on_reset();
  }
}
