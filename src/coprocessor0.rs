/// First of four MIPS coprocessors (CP0 - CP3).
/// This is an internal system control coprocessor
/// used to convert virtual memory addresses to
/// physical memory addresses, and for exception
/// processing.
/// Datasheet: http://datasheets.chipdb.org/NEC/Vr-Series/Vr43xx/U10504EJ7V0UMJ1.pdf
/// Datasheet: Chapter 1, pages 44 - 46

#[derive(Debug, Default)]
pub struct Cp0 {
  reg_index: u64,      // (0) Pointer into Transition Lookaside Buffer (TLB) array
  reg_random: u64,     // (1) Psuedorandom pointer into TLB array (read only)
  reg_entry_lo_0: u64, // (2) Low half of TLB entry for even virtual address (VPN)
  reg_entry_lo_1: u64, // (3) Low half of TLB entry for odd virtual address (VPN)
  reg_context: u64,    // (4) Pointer to kernel virtual Page Table Entry (PTE) in 32-bit mode
  reg_page_mask: u64,  // (5) Page size specification
  reg_wired: u64,      // (6) Number of wired TLB entries
  // (7) not used
  reg_bad_v_addr: u64, // (8) Display of virtual address that occurred an error last
  reg_count: u64,      // (9) Timer Count
  reg_entry_hi: u64,   // (10) High half of TLB entry (including Address Space Id (ASID))
  reg_compare: u64,    // (11) Timer Compare Value
  reg_status: u64,     // (12) Operation status setting
  reg_cause: u64,      // (13) Display of cause of last exception
  reg_epc: u64,        // (14) Exception Program Counter
  reg_pr_id: u64,      // (15) Processor Revision Identifier
  reg_config: u64,     // (16) Memory system mode setting
  reg_ll_addr: u64,    // (17) Load Linked instruction address display
  reg_watch_lo: u64,   // (18) Memory reference trap address low bits
  reg_watch_hi: u64,   // (19) Memory reference trap address high bits
  reg_x_context: u64,  // (20) Pointer to Kernel virtual PTE table in 64-bit mode
  // (20 - 25) not used
  // (26) Cache parity bits - not used
  // (27) Cache Error and Status register - not used
  reg_tag_lo: u64, // (28) Cache tag register low
  reg_tag_hi: u64, // (29) Cache tag register hi
  reg_error_epc: u64, // (30) Error Exception Program Counter
                   // (31) not used
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
    // TODO
  }
}
