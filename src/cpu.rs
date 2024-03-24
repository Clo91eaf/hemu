//! The cpu module contains the privileged mode, registers, and CPU.

use std::cmp;
use std::cmp::PartialEq;
use std::num::FpCategory;

use crate::{
  bus::{Bus, DRAM_BASE},
  devices::{
    dram::DRAM_SIZE,
    uart::UART_IRQ,
    virtio_blk::{Virtio, VIRTIO_IRQ},
  },
  exception::Exception,
  instructions::Inst,
  interrupt::Interrupt,
};

pub mod csr;
pub mod fpg;
pub mod gpr;

use csr::*;
use fpg::*;
use gpr::*;

/// The number of registers.
pub const REGISTERS_COUNT: usize = 32;
/// The page size (4 KiB) for the virtual memory system.
const PAGE_SIZE: u64 = 4096;

/// 8 bits. 1 byte.
pub const BYTE: u8 = 8;
/// 16 bits. 2 bytes.
pub const HALFWORD: u8 = 16;
/// 32 bits. 4 bytes.
pub const WORD: u8 = 32;
/// 64 bits. 8 bytes.
pub const DOUBLEWORD: u8 = 64;

/// riscv-pk is passing x10 and x11 registers to kernel. x11 is expected to have the pointer to DTB.
/// https://github.com/riscv/riscv-pk/blob/master/machine/mentry.S#L233-L235
pub const POINTER_TO_DTB: u64 = 0x1020;

/// Access type that is used in the virtual address translation process. It decides which exception
/// should raises (InstructionPageFault, LoadPageFault or StoreAMOPageFault).
#[derive(Debug, PartialEq, PartialOrd)]
pub enum AccessType {
  /// Raises the exception InstructionPageFault. It is used for an instruction fetch.
  Instruction,
  /// Raises the exception LoadPageFault.
  Load,
  /// Raises the exception StoreAMOPageFault.
  Store,
}

/// The privileged mode.
#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
pub enum Mode {
  User = 0b00,
  Supervisor = 0b01,
  Machine = 0b11,
  Debug,
}

/// The CPU to contain registers, a program counter, status, and a privileged mode.
pub struct Cpu {
  /// 64-bit integer registers.
  pub gpr: Gpr,
  /// 64-bit floating-point registers.
  pub fpg: Fpg,
  /// Program counter.
  pub pc: u64,
  /// Instructions.
  pub inst: Inst,
  /// Control and status registers (CSR).
  pub csr: Csr,
  /// Privilege level.
  pub mode: Mode,
  /// System bus.
  pub bus: Bus,
  /// SV39 paging flag.
  enable_paging: bool,
  /// Physical page number (PPN) × PAGE_SIZE (4096).
  page_table: u64,
  /// A set of bytes that subsumes the bytes in the addressed word used in
  /// load-reserved/store-conditional instructions.
  reservation_set: Vec<u64>,
  /// Idle state. True when WFI is called, and becomes false when an interrupt happens.
  pub idle: bool,
}

impl Cpu {
  /// Create a new `Cpu` object.
  pub fn new() -> Cpu {
    Cpu {
      gpr: Gpr::new(),
      fpg: Fpg::new(),
      pc: 0,
      inst: Inst::new(),
      csr: Csr::new(),
      mode: Mode::Machine,
      bus: Bus::new(),
      enable_paging: false,
      page_table: 0,
      reservation_set: Vec::new(),
      idle: false,
    }
  }

  /// Reset CPU states.
  pub fn reset(&mut self) {
    self.pc = 0;
    self.mode = Mode::Machine;
    self.csr.reset();
    for i in 0..REGISTERS_COUNT {
      self.gpr.write(i as u64, 0);
      self.fpg.write(i as u64, 0.0);
    }
  }

  /// Check interrupt flags for all devices that can interrupt.
  pub fn check_pending_interrupt(&mut self) -> Option<Interrupt> {
    // global interrupt: PLIC (Platform Local Interrupt Controller) dispatches global
    //                   interrupts to multiple harts.
    // local interrupt: CLINT (Core Local Interrupter) dispatches local interrupts to a hart
    //                  which directly connected to CLINT.

    // 3.1.6.1 Privilege and Global Interrupt-Enable Stack in mstatus register
    // "When a hart is executing in privilege mode x, interrupts are globally enabled when
    // xIE=1 and globally disabled when xIE=0."
    match self.mode {
      Mode::Machine => {
        // Check if the MIE bit is enabled.
        if self.csr.read_mstatus(MSTATUS_MIE) == 0 {
          return None;
        }
      }
      Mode::Supervisor => {
        // Check if the SIE bit is enabled.
        if self.csr.read_sstatus(XSTATUS_SIE) == 0 {
          return None;
        }
      }
      _ => {}
    }

    // TODO: Take interrupts based on priorities.

    // Check external interrupt for uart and virtio.
    let irq;
    if self.bus.uart.is_interrupting() {
      irq = UART_IRQ;
    } else if self.bus.virtio.is_interrupting() {
      // An interrupt is raised after a disk access is done.
      Virtio::disk_access(self).expect("failed to access the disk");
      irq = VIRTIO_IRQ;
    } else {
      irq = 0;
    }

    if irq != 0 {
      // TODO: assume that hart is 0
      // TODO: write a value to MCLAIM if the mode is machine
      self.bus.plic.update_pending(irq);
      self.csr.write(MIP, self.csr.read(MIP) | SEIP_BIT);
    }

    // 3.1.9 Machine Interrupt Registers (mip and mie)
    // "An interrupt i will be taken if bit i is set in both mip and mie, and if interrupts are
    // globally enabled. By default, M-mode interrupts are globally enabled if the hart’s
    // current privilege mode is less than M, or if the current privilege mode is M and the MIE
    // bit in the mstatus register is set. If bit i in mideleg is set, however, interrupts are
    // considered to be globally enabled if the hart’s current privilege mode equals the
    // delegated privilege mode (S or U) and that mode’s interrupt enable bit (SIE or UIE in
    // mstatus) is set, or if the current privilege mode is less than the delegated privilege
    // mode."
    let pending = self.csr.read(MIE) & self.csr.read(MIP);

    if (pending & MEIP_BIT) != 0 {
      self.csr.write(MIP, self.csr.read(MIP) & !MEIP_BIT);
      return Some(Interrupt::MachineExternalInterrupt);
    }
    if (pending & MSIP_BIT) != 0 {
      self.csr.write(MIP, self.csr.read(MIP) & !MSIP_BIT);
      return Some(Interrupt::MachineSoftwareInterrupt);
    }
    if (pending & MTIP_BIT) != 0 {
      self.csr.write(MIP, self.csr.read(MIP) & !MTIP_BIT);
      return Some(Interrupt::MachineTimerInterrupt);
    }
    if (pending & SEIP_BIT) != 0 {
      self.csr.write(MIP, self.csr.read(MIP) & !SEIP_BIT);
      return Some(Interrupt::SupervisorExternalInterrupt);
    }
    if (pending & SSIP_BIT) != 0 {
      self.csr.write(MIP, self.csr.read(MIP) & !SSIP_BIT);
      return Some(Interrupt::SupervisorSoftwareInterrupt);
    }
    if (pending & STIP_BIT) != 0 {
      self.csr.write(MIP, self.csr.read(MIP) & !STIP_BIT);
      return Some(Interrupt::SupervisorTimerInterrupt);
    }

    return None;
  }

  /// Update the physical page number (PPN) and the addressing mode.
  fn update_paging(&mut self) {
    // Read the physical page number (PPN) of the root page table, i.e., its
    // supervisor physical address divided by 4 KiB.
    self.page_table = self.csr.read_bits(SATP, ..44) * PAGE_SIZE;

    // Read the MODE field, which selects the current address-translation scheme.
    let mode = self.csr.read_bits(SATP, 60..);

    // Enable the SV39 paging if the value of the mode field is 8.
    if mode == 8 {
      self.enable_paging = true;
    } else {
      self.enable_paging = false;
    }
  }

  /// Translate a virtual address to a physical address for the paged virtual-memory system.
  fn translate(&mut self, addr: u64, access_type: AccessType) -> Result<u64, Exception> {
    if !self.enable_paging || self.mode == Mode::Machine {
      return Ok(addr);
    }

    // 4.3.2 Virtual Address Translation Process
    // (The RISC-V Instruction Set Manual Volume II-Privileged Architecture_20190608)
    // A virtual address va is translated into a physical address pa as follows:
    let levels = 3;
    let vpn = [(addr >> 12) & 0x1ff, (addr >> 21) & 0x1ff, (addr >> 30) & 0x1ff];

    // 1. Let a be satp.ppn × PAGESIZE, and let i = LEVELS − 1. (For Sv32, PAGESIZE=212
    //    and LEVELS=2.)
    let mut a = self.page_table;
    let mut i: i64 = levels - 1;
    let mut pte;
    loop {
      // 2. Let pte be the value of the PTE at address a+va.vpn[i]×PTESIZE. (For Sv32,
      //    PTESIZE=4.) If accessing pte violates a PMA or PMP check, raise an access
      //    exception corresponding to the original access type.
      pte = self.bus.read(a + vpn[i as usize] * 8, DOUBLEWORD)?;

      // 3. If pte.v = 0, or if pte.r = 0 and pte.w = 1, stop and raise a page-fault
      //    exception corresponding to the original access type.
      let v = pte & 1;
      let r = (pte >> 1) & 1;
      let w = (pte >> 2) & 1;
      let x = (pte >> 3) & 1;
      if v == 0 || (r == 0 && w == 1) {
        match access_type {
          AccessType::Instruction => return Err(Exception::InstructionPageFault(addr)),
          AccessType::Load => return Err(Exception::LoadPageFault(addr)),
          AccessType::Store => return Err(Exception::StoreAMOPageFault(addr)),
        }
      }

      // 4. Otherwise, the PTE is valid. If pte.r = 1 or pte.x = 1, go to step 5.
      //    Otherwise, this PTE is a pointer to the next level of the page table.
      //    Let i = i − 1. If i < 0, stop and raise a page-fault exception
      //    corresponding to the original access type. Otherwise,
      //    let a = pte.ppn × PAGESIZE and go to step 2.
      if r == 1 || x == 1 {
        break;
      }
      i -= 1;
      let ppn = (pte >> 10) & 0x0fff_ffff_ffff;
      a = ppn * PAGE_SIZE;
      if i < 0 {
        match access_type {
          AccessType::Instruction => return Err(Exception::InstructionPageFault(addr)),
          AccessType::Load => return Err(Exception::LoadPageFault(addr)),
          AccessType::Store => return Err(Exception::StoreAMOPageFault(addr)),
        }
      }
    }
    // TODO: implement step 5
    // 5. A leaf PTE has been found. Determine if the requested memory access is
    //    allowed by the pte.r, pte.w, pte.x, and pte.u bits, given the current
    //    privilege mode and the value of the SUM and MXR fields of the mstatus
    //    register. If not, stop and raise a page-fault exception corresponding
    //    to the original access type.

    // 3.1.6.3 Memory Privilege in mstatus Register
    // "The MXR (Make eXecutable Readable) bit modifies the privilege with which loads access
    // virtual memory. When MXR=0, only loads from pages marked readable (R=1 in Figure 4.15)
    // will succeed. When MXR=1, loads from pages marked either readable or executable
    // (R=1 or X=1) will succeed. MXR has no effect when page-based virtual memory is not in
    // effect. MXR is hardwired to 0 if S-mode is not supported."

    // "The SUM (permit Supervisor User Memory access) bit modifies the privilege with which
    // S-mode loads and stores access virtual memory. When SUM=0, S-mode memory accesses to
    // pages that are accessible by U-mode (U=1 in Figure 4.15) will fault. When SUM=1, these
    // accesses are permitted.  SUM has no effect when page-based virtual memory is not in
    // effect. Note that, while SUM is ordinarily ignored when not executing in S-mode, it is
    // in effect when MPRV=1 and MPP=S. SUM is hardwired to 0 if S-mode is not supported."

    // 6. If i > 0 and pte.ppn[i−1:0] != 0, this is a misaligned superpage; stop and
    //    raise a page-fault exception corresponding to the original access type.
    let ppn = [(pte >> 10) & 0x1ff, (pte >> 19) & 0x1ff, (pte >> 28) & 0x03ff_ffff];
    if i > 0 {
      for j in (0..i).rev() {
        if ppn[j as usize] != 0 {
          // A misaligned superpage.
          match access_type {
            AccessType::Instruction => return Err(Exception::InstructionPageFault(addr)),
            AccessType::Load => return Err(Exception::LoadPageFault(addr)),
            AccessType::Store => return Err(Exception::StoreAMOPageFault(addr)),
          }
        }
      }
    }

    // 7. If pte.a = 0, or if the memory access is a store and pte.d = 0, either raise
    //    a page-fault exception corresponding to the original access type, or:
    //    • Set pte.a to 1 and, if the memory access is a store, also set pte.d to 1.
    //    • If this access violates a PMA or PMP check, raise an access exception
    //    corresponding to the original access type.
    //    • This update and the loading of pte in step 2 must be atomic; in particular,
    //    no intervening store to the PTE may be perceived to have occurred in-between.
    let a = (pte >> 6) & 1;
    let d = (pte >> 7) & 1;
    if a == 0 || (access_type == AccessType::Store && d == 0) {
      // Set pte.a to 1 and, if the memory access is a store, also set pte.d to 1.
      pte = pte | (1 << 6) | if access_type == AccessType::Store { 1 << 7 } else { 0 };

      // TODO: PMA or PMP check.

      // Update the value of address satp.ppn × PAGESIZE + va.vpn[i] × PTESIZE with new pte
      // value.
      // TODO: If this is enabled, running xv6 fails.
      //self.bus.write(self.page_table + vpn[i as usize] * 8, pte, 64)?;
    }

    // 8. The translation is successful. The translated physical address is given as
    //    follows:
    //    • pa.pgoff = va.pgoff.
    //    • If i > 0, then this is a superpage translation and pa.ppn[i−1:0] =
    //    va.vpn[i−1:0].
    //    • pa.ppn[LEVELS−1:i] = pte.ppn[LEVELS−1:i].
    let offset = addr & 0xfff;
    match i {
      0 => {
        let ppn = (pte >> 10) & 0x0fff_ffff_ffff;
        Ok((ppn << 12) | offset)
      }
      1 => {
        // Superpage translation. A superpage is a memory page of larger size than an
        // ordinary page (4 KiB). It reduces TLB misses and improves performance.
        Ok((ppn[2] << 30) | (ppn[1] << 21) | (vpn[0] << 12) | offset)
      }
      2 => {
        // Superpage translation. A superpage is a memory page of larger size than an
        // ordinary page (4 KiB). It reduces TLB misses and improves performance.
        Ok((ppn[2] << 30) | (vpn[1] << 21) | (vpn[0] << 12) | offset)
      }
      _ => match access_type {
        AccessType::Instruction => return Err(Exception::InstructionPageFault(addr)),
        AccessType::Load => return Err(Exception::LoadPageFault(addr)),
        AccessType::Store => return Err(Exception::StoreAMOPageFault(addr)),
      },
    }
  }

  /// Read `size`-bit data from the system bus with the translation a virtual address to a physical address
  /// if it is enabled.
  fn read(&mut self, v_addr: u64, size: u8) -> Result<u64, Exception> {
    let previous_mode = self.mode;

    // 3.1.6.3 Memory Privilege in mstatus Register
    // "When MPRV=1, load and store memory addresses are translated and protected, and
    // endianness is applied, as though the current privilege mode were set to MPP."
    if self.csr.read_mstatus(MSTATUS_MPRV) == 1 {
      self.mode = match self.csr.read_mstatus(MSTATUS_MPP) {
        0b00 => Mode::User,
        0b01 => Mode::Supervisor,
        0b11 => Mode::Machine,
        _ => Mode::Debug,
      };
    }

    let p_addr = self.translate(v_addr, AccessType::Load)?;
    let result = self.bus.read(p_addr, size);

    if self.csr.read_mstatus(MSTATUS_MPRV) == 1 {
      self.mode = previous_mode;
    }

    result
  }

  /// Write `size`-bit data to the system bus with the translation a virtual address to a physical
  /// address if it is enabled.
  fn write(&mut self, v_addr: u64, value: u64, size: u8) -> Result<(), Exception> {
    let previous_mode = self.mode;

    // 3.1.6.3 Memory Privilege in mstatus Register
    // "When MPRV=1, load and store memory addresses are translated and protected, and
    // endianness is applied, as though the current privilege mode were set to MPP."
    if self.csr.read_mstatus(MSTATUS_MPRV) == 1 {
      self.mode = match self.csr.read_mstatus(MSTATUS_MPP) {
        0b00 => Mode::User,
        0b01 => Mode::Supervisor,
        0b11 => Mode::Machine,
        _ => Mode::Debug,
      };
    }

    // "The SC must fail if a write from some other device to the bytes accessed by the LR can
    // be observed to occur between the LR and SC."
    if self.reservation_set.contains(&v_addr) {
      self.reservation_set.retain(|&x| x != v_addr);
    }

    let p_addr = self.translate(v_addr, AccessType::Store)?;
    let result = self.bus.write(p_addr, value, size);

    if self.csr.read_mstatus(MSTATUS_MPRV) == 1 {
      self.mode = previous_mode;
    }

    result
  }

  /// Fetch the `size`-bit next instruction from the memory at the current program counter.
  pub fn fetch(&mut self, size: u8) -> Result<u64, Exception> {
    if size != HALFWORD && size != WORD {
      return Err(Exception::InstructionAccessFault);
    }

    let p_pc = self.translate(self.pc, AccessType::Instruction)?;

    // The result of the read method can be `Exception::LoadAccessFault`. In fetch(), an error
    // should be `Exception::InstructionAccessFault`.
    match self.bus.read(p_pc, size) {
      Ok(value) => Ok(value),
      Err(_) => Err(Exception::InstructionAccessFault),
    }
  }

  /// Execute a cycle on peripheral devices.
  pub fn devices_increment(&mut self) {
    // TODO: mtime in Clint and TIME in CSR should be the same value.
    // Increment the timer register (mtimer) in Clint.
    self.bus.clint.increment(&mut self.csr);
    // Increment the value in the TIME and CYCLE registers in CSR.
    self.csr.increment_time();
  }

  /// Execute an instruction. Raises an exception if something is wrong, otherwise, returns
  /// the instruction executed in this cycle.
  pub fn execute(&mut self) -> Result<u64, Exception> {
    // WFI is called and pending interrupts don't exist.
    if self.idle {
      return Ok(0);
    }

    // Fetch. It can be optimized by only one fetch.
    let inst16 = self.fetch(HALFWORD)?;
    let inst = self.fetch(WORD)?;
    match inst16 & 0b11 {
      0 | 1 | 2 => {
        self.execute_compressed(inst16)?;
        // Add 2 bytes to the program counter.
        self.pc = self.pc.wrapping_add(2);
        Ok(inst16)
      }
      _ => {
        self.execute_general(inst)?;
        // Add 4 bytes to the program counter.
        self.pc = self.pc.wrapping_add(4);
        Ok(inst)
      }
    }
  }

  /// Execute a compressed instruction. Raised an exception if something is wrong, otherwise,
  /// returns a fetched instruction. It also increments the program counter by 2 bytes.
  fn execute_compressed(&mut self, inst: u64) -> Result<(), Exception> {
    // 2. Decode.
    let opcode = inst & 0x3;
    let funct3 = (inst >> 13) & 0x7;

    // 3. Execute.
    // Compressed instructions have 3-bit field for popular registers, which correspond to
    // registers x8 to x15.
    match opcode {
      0 => {
        // Quadrant 0.
        match funct3 {
          0x0 => {
            // c.addi4spn
            // Expands to addi rd, x2, nzuimm, where rd=rd'+8.

            let rd = ((inst >> 2) & 0x7) + 8;
            // nzuimm[5:4|9:6|2|3] = inst[12:11|10:7|6|5]
            let nzuimm = ((inst >> 1) & 0x3c0) // znuimm[9:6]
                            | ((inst >> 7) & 0x30) // znuimm[5:4]
                            | ((inst >> 2) & 0x8) // znuimm[3]
                            | ((inst >> 4) & 0x4); // znuimm[2]
            if nzuimm == 0 {
              return Err(Exception::IllegalInstruction(inst));
            }
            self.gpr.write(rd, self.gpr.read(2).wrapping_add(nzuimm));
          }
          0x1 => {
            // c.fld
            // Expands to fld rd, offset(rs1), where rd=rd'+8 and rs1=rs1'+8.
            let rd = ((inst >> 2) & 0x7) + 8;
            let rs1 = ((inst >> 7) & 0x7) + 8;
            // offset[5:3|7:6] = isnt[12:10|6:5]
            let offset = ((inst << 1) & 0xc0) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
            let val = f64::from_bits(self.read(self.gpr.read(rs1).wrapping_add(offset), DOUBLEWORD)?);
            self.fpg.write(rd, val);
          }
          0x2 => {
            // c.lw
            // Expands to lw rd, offset(rs1), where rd=rd'+8 and rs1=rs1'+8.

            let rd = ((inst >> 2) & 0x7) + 8;
            let rs1 = ((inst >> 7) & 0x7) + 8;
            // offset[5:3|2|6] = isnt[12:10|6|5]
            let offset = ((inst << 1) & 0x40) // imm[6]
                            | ((inst >> 7) & 0x38) // imm[5:3]
                            | ((inst >> 4) & 0x4); // imm[2]
            let addr = self.gpr.read(rs1).wrapping_add(offset);
            let val = self.read(addr, WORD)?;
            self.gpr.write(rd, val as i32 as i64 as u64);
          }
          0x3 => {
            // c.ld
            // Expands to ld rd, offset(rs1), where rd=rd'+8 and rs1=rs1'+8.

            let rd = ((inst >> 2) & 0x7) + 8;
            let rs1 = ((inst >> 7) & 0x7) + 8;
            // offset[5:3|7:6] = isnt[12:10|6:5]
            let offset = ((inst << 1) & 0xc0) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
            let addr = self.gpr.read(rs1).wrapping_add(offset);
            let val = self.read(addr, DOUBLEWORD)?;
            self.gpr.write(rd, val);
          }
          0x4 => {
            // Reserved.
            panic!("reserved");
          }
          0x5 => {
            // c.fsd
            // Expands to fsd rs2, offset(rs1), where rs2=rs2'+8 and rs1=rs1'+8.

            let rs2 = ((inst >> 2) & 0x7) + 8;
            let rs1 = ((inst >> 7) & 0x7) + 8;
            // offset[5:3|7:6] = isnt[12:10|6:5]
            let offset = ((inst << 1) & 0xc0) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
            let addr = self.gpr.read(rs1).wrapping_add(offset);
            self.write(addr, self.fpg.read(rs2).to_bits() as u64, DOUBLEWORD)?;
          }
          0x6 => {
            // c.sw
            // Expands to sw rs2, offset(rs1), where rs2=rs2'+8 and rs1=rs1'+8.

            let rs2 = ((inst >> 2) & 0x7) + 8;
            let rs1 = ((inst >> 7) & 0x7) + 8;
            // offset[5:3|2|6] = isnt[12:10|6|5]
            let offset = ((inst << 1) & 0x40) // imm[6]
                            | ((inst >> 7) & 0x38) // imm[5:3]
                            | ((inst >> 4) & 0x4); // imm[2]
            let addr = self.gpr.read(rs1).wrapping_add(offset);
            self.write(addr, self.gpr.read(rs2), WORD)?;
          }
          0x7 => {
            // c.sd
            // Expands to sd rs2, offset(rs1), where rs2=rs2'+8 and rs1=rs1'+8.

            let rs2 = ((inst >> 2) & 0x7) + 8;
            let rs1 = ((inst >> 7) & 0x7) + 8;
            // offset[5:3|7:6] = isnt[12:10|6:5]
            let offset = ((inst << 1) & 0xc0) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
            let addr = self.gpr.read(rs1).wrapping_add(offset);
            self.write(addr, self.gpr.read(rs2), DOUBLEWORD)?;
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      1 => {
        // Quadrant 1.
        match funct3 {
          0x0 => {
            // c.addi
            // Expands to addi rd, rd, nzimm.

            let rd = (inst >> 7) & 0x1f;
            // nzimm[5|4:0] = inst[12|6:2]
            let mut nzimm = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
            // Sign-extended.
            nzimm = match (nzimm & 0x20) == 0 {
              true => nzimm,
              false => (0xc0 | nzimm) as i8 as i64 as u64,
            };
            if rd != 0 {
              self.gpr.write(rd, self.gpr.read(rd).wrapping_add(nzimm));
            }
          }
          0x1 => {
            // c.addiw
            // Expands to addiw rd, rd, imm
            // "The immediate can be zero for C.ADDIW, where this corresponds to sext.w
            // rd"

            let rd = (inst >> 7) & 0x1f;
            // imm[5|4:0] = inst[12|6:2]
            let mut imm = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
            // Sign-extended.
            imm = match (imm & 0x20) == 0 {
              true => imm,
              false => (0xc0 | imm) as i8 as i64 as u64,
            };
            if rd != 0 {
              self
                .gpr
                .write(rd, self.gpr.read(rd).wrapping_add(imm) as i32 as i64 as u64);
            }
          }
          0x2 => {
            // c.li
            // Expands to addi rd, x0, imm.

            let rd = (inst >> 7) & 0x1f;
            // imm[5|4:0] = inst[12|6:2]
            let mut imm = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
            // Sign-extended.
            imm = match (imm & 0x20) == 0 {
              true => imm,
              false => (0xc0 | imm) as i8 as i64 as u64,
            };
            if rd != 0 {
              self.gpr.write(rd, imm);
            }
          }
          0x3 => {
            let rd = (inst >> 7) & 0x1f;
            match rd {
              0 => {}
              2 => {
                // c.addi16sp
                // Expands to addi x2, x2, nzimm

                // nzimm[9|4|6|8:7|5] = inst[12|6|5|4:3|2]
                let mut nzimm = ((inst >> 3) & 0x200) // nzimm[9]
                                    | ((inst >> 2) & 0x10) // nzimm[4]
                                    | ((inst << 1) & 0x40) // nzimm[6]
                                    | ((inst << 4) & 0x180) // nzimm[8:7]
                                    | ((inst << 3) & 0x20); // nzimm[5]
                nzimm = match (nzimm & 0x200) == 0 {
                  true => nzimm,
                  // Sign-extended.
                  false => (0xfc00 | nzimm) as i16 as i32 as i64 as u64,
                };
                if nzimm != 0 {
                  self.gpr.write(2, self.gpr.read(2).wrapping_add(nzimm));
                }
              }
              _ => {
                // c.lui
                // Expands to lui rd, nzimm.

                // nzimm[17|16:12] = inst[12|6:2]
                let mut nzimm = ((inst << 5) & 0x20000) | ((inst << 10) & 0x1f000);
                // Sign-extended.
                nzimm = match (nzimm & 0x20000) == 0 {
                  true => nzimm,
                  false => (0xfffc0000 | nzimm) as i32 as i64 as u64,
                };
                if nzimm != 0 {
                  self.gpr.write(rd, nzimm);
                }
              }
            }
          }
          0x4 => {
            let funct2 = (inst >> 10) & 0x3;
            match funct2 {
              0x0 => {
                // c.srli
                // Expands to srli rd, rd, shamt, where rd=rd'+8.

                let rd = ((inst >> 7) & 0b111) + 8;
                // shamt[5|4:0] = inst[12|6:2]
                let shamt = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
                self.gpr.write(rd, self.gpr.read(rd) >> shamt);
              }
              0x1 => {
                // c.srai
                // Expands to srai rd, rd, shamt, where rd=rd'+8.

                let rd = ((inst >> 7) & 0b111) + 8;
                // shamt[5|4:0] = inst[12|6:2]
                let shamt = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
                self.gpr.write(rd, ((self.gpr.read(rd) as i64) >> shamt) as u64);
              }
              0x2 => {
                // c.andi
                // Expands to andi rd, rd, imm, where rd=rd'+8.

                let rd = ((inst >> 7) & 0b111) + 8;
                // imm[5|4:0] = inst[12|6:2]
                let mut imm = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
                // Sign-extended.
                imm = match (imm & 0x20) == 0 {
                  true => imm,
                  false => (0xc0 | imm) as i8 as i64 as u64,
                };
                self.gpr.write(rd, self.gpr.read(rd) & imm);
              }
              0x3 => {
                match ((inst >> 12) & 0b1, (inst >> 5) & 0b11) {
                  (0x0, 0x0) => {
                    // c.sub
                    // Expands to sub rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.

                    let rd = ((inst >> 7) & 0b111) + 8;
                    let rs2 = ((inst >> 2) & 0b111) + 8;
                    self.gpr.write(rd, self.gpr.read(rd).wrapping_sub(self.gpr.read(rs2)));
                  }
                  (0x0, 0x1) => {
                    // c.xor
                    // Expands to xor rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.

                    let rd = ((inst >> 7) & 0b111) + 8;
                    let rs2 = ((inst >> 2) & 0b111) + 8;
                    self.gpr.write(rd, self.gpr.read(rd) ^ self.gpr.read(rs2));
                  }
                  (0x0, 0x2) => {
                    // c.or
                    // Expands to or rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.

                    let rd = ((inst >> 7) & 0b111) + 8;
                    let rs2 = ((inst >> 2) & 0b111) + 8;
                    self.gpr.write(rd, self.gpr.read(rd) | self.gpr.read(rs2));
                  }
                  (0x0, 0x3) => {
                    // c.and
                    // Expands to and rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.

                    let rd = ((inst >> 7) & 0b111) + 8;
                    let rs2 = ((inst >> 2) & 0b111) + 8;
                    self.gpr.write(rd, self.gpr.read(rd) & self.gpr.read(rs2));
                  }
                  (0x1, 0x0) => {
                    // c.subw
                    // Expands to subw rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.

                    let rd = ((inst >> 7) & 0b111) + 8;
                    let rs2 = ((inst >> 2) & 0b111) + 8;
                    self.gpr.write(
                      rd,
                      self.gpr.read(rd).wrapping_sub(self.gpr.read(rs2)) as i32 as i64 as u64,
                    );
                  }
                  (0x1, 0x1) => {
                    // c.addw
                    // Expands to addw rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.

                    let rd = ((inst >> 7) & 0b111) + 8;
                    let rs2 = ((inst >> 2) & 0b111) + 8;
                    self.gpr.write(
                      rd,
                      self.gpr.read(rd).wrapping_add(self.gpr.read(rs2)) as i32 as i64 as u64,
                    );
                  }
                  _ => {
                    return Err(Exception::IllegalInstruction(inst));
                  }
                }
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x5 => {
            // c.j
            // Expands to jal x0, offset.

            // offset[11|4|9:8|10|6|7|3:1|5] = inst[12|11|10:9|8|7|6|5:3|2]
            let mut offset = ((inst >> 1) & 0x800) // offset[11]
                            | ((inst << 2) & 0x400) // offset[10]
                            | ((inst >> 1) & 0x300) // offset[9:8]
                            | ((inst << 1) & 0x80) // offset[7]
                            | ((inst >> 1) & 0x40) // offset[6]
                            | ((inst << 3) & 0x20) // offset[5]
                            | ((inst >> 7) & 0x10) // offset[4]
                            | ((inst >> 2) & 0xe); // offset[3:1]

            // Sign-extended.
            offset = match (offset & 0x800) == 0 {
              true => offset,
              false => (0xf000 | offset) as i16 as i64 as u64,
            };
            self.pc = self.pc.wrapping_add(offset).wrapping_sub(2);
          }
          0x6 => {
            // c.beqz
            // Expands to beq rs1, x0, offset, rs1=rs1'+8.

            let rs1 = ((inst >> 7) & 0b111) + 8;
            // offset[8|4:3|7:6|2:1|5] = inst[12|11:10|6:5|4:3|2]
            let mut offset = ((inst >> 4) & 0x100) // offset[8]
                            | ((inst << 1) & 0xc0) // offset[7:6]
                            | ((inst << 3) & 0x20) // offset[5]
                            | ((inst >> 7) & 0x18) // offset[4:3]
                            | ((inst >> 2) & 0x6); // offset[2:1]
                                                   // Sign-extended.
            offset = match (offset & 0x100) == 0 {
              true => offset,
              false => (0xfe00 | offset) as i16 as i64 as u64,
            };
            if self.gpr.read(rs1) == 0 {
              self.pc = self.pc.wrapping_add(offset).wrapping_sub(2);
            }
          }
          0x7 => {
            // c.bnez
            // Expands to bne rs1, x0, offset, rs1=rs1'+8.

            let rs1 = ((inst >> 7) & 0b111) + 8;
            // offset[8|4:3|7:6|2:1|5] = inst[12|11:10|6:5|4:3|2]
            let mut offset = ((inst >> 4) & 0x100) // offset[8]
                            | ((inst << 1) & 0xc0) // offset[7:6]
                            | ((inst << 3) & 0x20) // offset[5]
                            | ((inst >> 7) & 0x18) // offset[4:3]
                            | ((inst >> 2) & 0x6); // offset[2:1]
                                                   // Sign-extended.
            offset = match (offset & 0x100) == 0 {
              true => offset,
              false => (0xfe00 | offset) as i16 as i64 as u64,
            };
            if self.gpr.read(rs1) != 0 {
              self.pc = self.pc.wrapping_add(offset).wrapping_sub(2);
            }
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      2 => {
        // Quadrant 2.
        match funct3 {
          0x0 => {
            // c.slli
            // Expands to slli rd, rd, shamt.

            let rd = (inst >> 7) & 0x1f;
            // shamt[5|4:0] = inst[12|6:2]
            let shamt = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
            if rd != 0 {
              self.gpr.write(rd, self.gpr.read(rd) << shamt);
            }
          }
          0x1 => {
            // c.fldsp
            // Expands to fld rd, offset(x2).

            let rd = (inst >> 7) & 0x1f;
            // offset[5|4:3|8:6] = inst[12|6:5|4:2]
            let offset = ((inst << 4) & 0x1c0) // offset[8:6]
                            | ((inst >> 7) & 0x20) // offset[5]
                            | ((inst >> 2) & 0x18); // offset[4:3]
            let val = f64::from_bits(self.read(self.gpr.read(2) + offset, DOUBLEWORD)?);
            self.fpg.write(rd, val);
          }
          0x2 => {
            // c.lwsp
            // Expands to lw rd, offset(x2).

            let rd = (inst >> 7) & 0x1f;
            // offset[5|4:2|7:6] = inst[12|6:4|3:2]
            let offset = ((inst << 4) & 0xc0) // offset[7:6]
                            | ((inst >> 7) & 0x20) // offset[5]
                            | ((inst >> 2) & 0x1c); // offset[4:2]
            let val = self.read(self.gpr.read(2).wrapping_add(offset), WORD)?;
            self.gpr.write(rd, val as i32 as i64 as u64);
          }
          0x3 => {
            // c.ldsp
            // Expands to ld rd, offset(x2).

            let rd = (inst >> 7) & 0x1f;
            // offset[5|4:3|8:6] = inst[12|6:5|4:2]
            let offset = ((inst << 4) & 0x1c0) // offset[8:6]
                            | ((inst >> 7) & 0x20) // offset[5]
                            | ((inst >> 2) & 0x18); // offset[4:3]
            let val = self.read(self.gpr.read(2).wrapping_add(offset), DOUBLEWORD)?;
            self.gpr.write(rd, val);
          }
          0x4 => {
            match ((inst >> 12) & 0x1, (inst >> 2) & 0x1f) {
              (0, 0) => {
                // c.jr
                // Expands to jalr x0, 0(rs1).

                let rs1 = (inst >> 7) & 0x1f;
                if rs1 != 0 {
                  self.pc = self.gpr.read(rs1).wrapping_sub(2);
                }
              }
              (0, _) => {
                // c.mv
                // Expands to add rd, x0, rs2.

                let rd = (inst >> 7) & 0x1f;
                let rs2 = (inst >> 2) & 0x1f;
                if rs2 != 0 {
                  self.gpr.write(rd, self.gpr.read(rs2));
                }
              }
              (1, 0) => {
                let rd = (inst >> 7) & 0x1f;
                if rd == 0 {
                  // c.ebreak
                  // Expands to ebreak.

                  return Err(Exception::Breakpoint);
                } else {
                  // c.jalr
                  // Expands to jalr x1, 0(rs1).

                  let rs1 = (inst >> 7) & 0x1f;
                  let t = self.pc.wrapping_add(2);
                  self.pc = self.gpr.read(rs1).wrapping_sub(2);
                  self.gpr.write(1, t);
                }
              }
              (1, _) => {
                // c.add
                // Expands to add rd, rd, rs2.

                let rd = (inst >> 7) & 0x1f;
                let rs2 = (inst >> 2) & 0x1f;
                if rs2 != 0 {
                  self.gpr.write(rd, self.gpr.read(rd).wrapping_add(self.gpr.read(rs2)));
                }
              }
              (_, _) => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x5 => {
            // c.fsdsp
            // Expands to fsd rs2, offset(x2).

            let rs2 = (inst >> 2) & 0x1f;
            // offset[5:3|8:6] = isnt[12:10|9:7]
            let offset = ((inst >> 1) & 0x1c0) // offset[8:6]
                            | ((inst >> 7) & 0x38); // offset[5:3]
            let addr = self.gpr.read(2).wrapping_add(offset);
            self.write(addr, self.fpg.read(rs2).to_bits(), DOUBLEWORD)?;
          }
          0x6 => {
            // c.swsp
            // Expands to sw rs2, offset(x2).

            let rs2 = (inst >> 2) & 0x1f;
            // offset[5:2|7:6] = inst[12:9|8:7]
            let offset = ((inst >> 1) & 0xc0) // offset[7:6]
                            | ((inst >> 7) & 0x3c); // offset[5:2]
            let addr = self.gpr.read(2).wrapping_add(offset);
            self.write(addr, self.gpr.read(rs2), WORD)?;
          }
          0x7 => {
            // c.sdsp
            // Expands to sd rs2, offset(x2).

            let rs2 = (inst >> 2) & 0x1f;
            // offset[5:3|8:6] = isnt[12:10|9:7]
            let offset = ((inst >> 1) & 0x1c0) // offset[8:6]
                            | ((inst >> 7) & 0x38); // offset[5:3]
            let addr = self.gpr.read(2).wrapping_add(offset);
            self.write(addr, self.gpr.read(rs2), DOUBLEWORD)?;
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      _ => {
        return Err(Exception::IllegalInstruction(inst));
      }
    }
    Ok(())
  }

  /// Execute a general-purpose instruction. Raises an exception if something is wrong,
  /// otherwise, returns a fetched instruction. It also increments the program counter by 4 bytes.
  fn execute_general(&mut self, inst: u64) -> Result<(), Exception> {
    // match self.inst.set_bits(inst as u32) {
    //   Ok(_) => {}
    //   Err(_) => {
    //     panic!("unknown inst, pc: {:x}, inst: {:x}", self.pc, self.inst.bits);
    //   }
    // }
    // 2. Decode.
    let opcode = inst & 0x0000007f;
    // let rd = self.inst.rd as u64;
    // let rs1 = self.inst.rs1 as u64;
    // let rs2 = self.inst.rs2 as u64;
    let rd = (inst & 0x00000f80) >> 7;
    let rs1 = (inst & 0x000f8000) >> 15;
    let rs2 = (inst & 0x01f00000) >> 20;
    // assert_eq!(rd, self.inst.rd as u64);
    // assert_eq!(rs1, self.inst.rs1 as u64);
    // assert_eq!(rs2, self.inst.rs2 as u64);
    let funct3 = (inst & 0x00007000) >> 12;
    let funct7 = (inst & 0xfe000000) >> 25;

    // 3. Execute.
    match opcode {
      0x03 => {
        // RV32I and RV64I
        // imm[11:0] = inst[31:20]
        let offset = ((inst as i32 as i64) >> 20) as u64;
        let addr = self.gpr.read(rs1).wrapping_add(offset);
        match funct3 {
          0x0 => {
            // lb

            let val = self.read(addr, BYTE)?;
            self.gpr.write(rd, val as i8 as i64 as u64);
          }
          0x1 => {
            // lh

            let val = self.read(addr, HALFWORD)?;
            self.gpr.write(rd, val as i16 as i64 as u64);
          }
          0x2 => {
            // lw

            let val = self.read(addr, WORD)?;
            self.gpr.write(rd, val as i32 as i64 as u64);
          }
          0x3 => {
            // ld

            let val = self.read(addr, DOUBLEWORD)?;
            self.gpr.write(rd, val);
          }
          0x4 => {
            // lbu

            let val = self.read(addr, BYTE)?;
            self.gpr.write(rd, val);
          }
          0x5 => {
            // lhu

            let val = self.read(addr, HALFWORD)?;
            self.gpr.write(rd, val);
          }
          0x6 => {
            // lwu

            let val = self.read(addr, WORD)?;
            self.gpr.write(rd, val);
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x07 => {
        // RV32D and RV64D
        // imm[11:0] = inst[31:20]
        let offset = ((inst as i32 as i64) >> 20) as u64;
        let addr = self.gpr.read(rs1).wrapping_add(offset);
        match funct3 {
          0x2 => {
            // flw

            let val = f32::from_bits(self.read(addr, WORD)? as u32);
            self.fpg.write(rd, val as f64);
          }
          0x3 => {
            // fld

            let val = f64::from_bits(self.read(addr, DOUBLEWORD)?);
            self.fpg.write(rd, val);
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x0f => {
        // RV32I and RV64I
        // fence instructions are not supported yet because this emulator executes an
        // instruction sequentially on a single thread.
        // fence.i is a part of the Zifencei extension.
        match funct3 {
          0x0 => {
            // fence
          }
          0x1 => {
            // fence.i
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x13 => {
        // RV32I and RV64I
        // imm[11:0] = inst[31:20]
        let imm = ((inst as i32 as i64) >> 20) as u64;
        let funct6 = funct7 >> 1;
        match funct3 {
          0x0 => {
            // addi

            self.gpr.write(rd, self.gpr.read(rs1).wrapping_add(imm));
          }
          0x1 => {
            // slli

            // shamt size is 5 bits for RV32I and 6 bits for RV64I.
            let shamt = (inst >> 20) & 0x3f;
            self.gpr.write(rd, self.gpr.read(rs1) << shamt);
          }
          0x2 => {
            // slti

            self.gpr.write(
              rd,
              if (self.gpr.read(rs1) as i64) < (imm as i64) {
                1
              } else {
                0
              },
            );
          }
          0x3 => {
            // sltiu

            self.gpr.write(rd, if self.gpr.read(rs1) < imm { 1 } else { 0 });
          }
          0x4 => {
            // xori

            self.gpr.write(rd, self.gpr.read(rs1) ^ imm);
          }
          0x5 => {
            match funct6 {
              0x00 => {
                // srli

                // shamt size is 5 bits for RV32I and 6 bits for RV64I.
                let shamt = (inst >> 20) & 0x3f;
                self.gpr.write(rd, self.gpr.read(rs1) >> shamt);
              }
              0x10 => {
                // srai

                // shamt size is 5 bits for RV32I and 6 bits for RV64I.
                let shamt = (inst >> 20) & 0x3f;
                self.gpr.write(rd, ((self.gpr.read(rs1) as i64) >> shamt) as u64);
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x6 => {
            // ori

            self.gpr.write(rd, self.gpr.read(rs1) | imm);
          }
          0x7 => {
            // andi

            self.gpr.write(rd, self.gpr.read(rs1) & imm);
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x17 => {
        // RV32I
        // auipc

        // AUIPC forms a 32-bit offset from the 20-bit U-immediate, filling
        // in the lowest 12 bits with zeros.
        // imm[31:12] = inst[31:12]
        let imm = (inst & 0xfffff000) as i32 as i64 as u64;
        self.gpr.write(rd, self.pc.wrapping_add(imm));
      }
      0x1b => {
        // RV64I
        // imm[11:0] = inst[31:20]
        let imm = ((inst as i32 as i64) >> 20) as u64;
        match funct3 {
          0x0 => {
            // addiw

            self
              .gpr
              .write(rd, self.gpr.read(rs1).wrapping_add(imm) as i32 as i64 as u64);
          }
          0x1 => {
            // slliw

            // "SLLIW, SRLIW, and SRAIW encodings with imm[5] ̸= 0 are reserved."
            let shamt = (imm & 0x1f) as u32;
            self.gpr.write(rd, (self.gpr.read(rs1) << shamt) as i32 as i64 as u64);
          }
          0x5 => {
            match funct7 {
              0x00 => {
                // srliw

                // "SLLIW, SRLIW, and SRAIW encodings with imm[5] ̸= 0 are reserved."
                let shamt = (imm & 0x1f) as u32;
                self
                  .gpr
                  .write(rd, ((self.gpr.read(rs1) as u32) >> shamt) as i32 as i64 as u64)
              }
              0x20 => {
                // sraiw

                // "SLLIW, SRLIW, and SRAIW encodings with imm[5] ̸= 0 are reserved."
                let shamt = (imm & 0x1f) as u32;
                self.gpr.write(rd, ((self.gpr.read(rs1) as i32) >> shamt) as i64 as u64);
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x23 => {
        // RV32I
        // offset[11:5|4:0] = inst[31:25|11:7]
        let offset = (((inst & 0xfe000000) as i32 as i64 >> 20) as u64) | ((inst >> 7) & 0x1f);
        let addr = self.gpr.read(rs1).wrapping_add(offset);
        match funct3 {
          0x0 => {
            // sb

            self.write(addr, self.gpr.read(rs2), BYTE)?
          }
          0x1 => {
            // sh

            self.write(addr, self.gpr.read(rs2), HALFWORD)?
          }
          0x2 => {
            // sw

            self.write(addr, self.gpr.read(rs2), WORD)?
          }
          0x3 => {
            // sd

            self.write(addr, self.gpr.read(rs2), DOUBLEWORD)?
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x27 => {
        // RV32F and RV64F
        // offset[11:5|4:0] = inst[31:25|11:7]
        let offset = ((((inst as i32 as i64) >> 20) as u64) & 0xfe0) | ((inst >> 7) & 0x1f);
        let addr = self.gpr.read(rs1).wrapping_add(offset);
        match funct3 {
          0x2 => {
            // fsw

            self.write(addr, (self.fpg.read(rs2) as f32).to_bits() as u64, WORD)?
          }
          0x3 => {
            // fsd

            self.write(addr, self.fpg.read(rs2).to_bits() as u64, DOUBLEWORD)?
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x2f => {
        // RV32A and RV64A
        let funct5 = (funct7 & 0b1111100) >> 2;
        // TODO: Handle `aq` and `rl`.
        let _aq = (funct7 & 0b0000010) >> 1; // acquire access
        let _rl = funct7 & 0b0000001; // release access
        match (funct3, funct5) {
          (0x2, 0x00) => {
            // amoadd.w

            let addr = self.gpr.read(rs1);
            // "For AMOs, the A extension requires that the address held in rs1 be
            // naturally aligned to the size of the operand (i.e., eight-byte aligned
            // for 64-bit words and four-byte aligned for 32-bit words). If the
            // address is not naturally aligned, an address-misaligned exception or
            // an access-fault exception will be generated."
            if addr % 4 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, WORD)?;
            self.write(addr, t.wrapping_add(self.gpr.read(rs2)), WORD)?;
            self.gpr.write(rd, t as i32 as i64 as u64);
          }
          (0x3, 0x00) => {
            // amoadd.d

            let addr = self.gpr.read(rs1);
            if addr % 8 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, DOUBLEWORD)?;
            self.write(addr, t.wrapping_add(self.gpr.read(rs2)), DOUBLEWORD)?;
            self.gpr.write(rd, t);
          }
          (0x2, 0x01) => {
            // amoswap.w

            let addr = self.gpr.read(rs1);
            if addr % 4 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, WORD)?;
            self.write(addr, self.gpr.read(rs2), WORD)?;
            self.gpr.write(rd, t as i32 as i64 as u64);
          }
          (0x3, 0x01) => {
            // amoswap.d

            let addr = self.gpr.read(rs1);
            if addr % 8 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, DOUBLEWORD)?;
            self.write(addr, self.gpr.read(rs2), DOUBLEWORD)?;
            self.gpr.write(rd, t);
          }
          (0x2, 0x02) => {
            // lr.w

            let addr = self.gpr.read(rs1);
            // "For LR and SC, the A extension requires that the address held in rs1 be
            // naturally aligned to the size of the operand (i.e., eight-byte aligned
            // for 64-bit words and four-byte aligned for 32-bit words)."
            if addr % 4 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let value = self.read(addr, WORD)?;
            self.gpr.write(rd, value as i32 as i64 as u64);
            self.reservation_set.push(addr);
          }
          (0x3, 0x02) => {
            // lr.d

            let addr = self.gpr.read(rs1);
            // "For LR and SC, the A extension requires that the address held in rs1 be
            // naturally aligned to the size of the operand (i.e., eight-byte aligned for
            // 64-bit words and four-byte aligned for 32-bit words)."
            if addr % 8 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let value = self.read(addr, DOUBLEWORD)?;
            self.gpr.write(rd, value);
            self.reservation_set.push(addr);
          }
          (0x2, 0x03) => {
            // sc.w

            let addr = self.gpr.read(rs1);
            // "For LR and SC, the A extension requires that the address held in rs1 be
            // naturally aligned to the size of the operand (i.e., eight-byte aligned for
            // 64-bit words and four-byte aligned for 32-bit words)."
            if addr % 4 != 0 {
              return Err(Exception::StoreAMOAddressMisaligned);
            }
            if self.reservation_set.contains(&addr) {
              // "Regardless of success or failure, executing an SC.W instruction
              // invalidates any reservation held by this hart. "
              self.reservation_set.retain(|&x| x != addr);
              self.write(addr, self.gpr.read(rs2), WORD)?;
              self.gpr.write(rd, 0);
            } else {
              self.reservation_set.retain(|&x| x != addr);
              self.gpr.write(rd, 1);
            };
          }
          (0x3, 0x03) => {
            // sc.d

            let addr = self.gpr.read(rs1);
            // "For LR and SC, the A extension requires that the address held in rs1 be
            // naturally aligned to the size of the operand (i.e., eight-byte aligned for
            // 64-bit words and four-byte aligned for 32-bit words)."
            if addr % 8 != 0 {
              return Err(Exception::StoreAMOAddressMisaligned);
            }
            if self.reservation_set.contains(&addr) {
              self.reservation_set.retain(|&x| x != addr);
              self.write(addr, self.gpr.read(rs2), DOUBLEWORD)?;
              self.gpr.write(rd, 0);
            } else {
              self.reservation_set.retain(|&x| x != addr);
              self.gpr.write(rd, 1);
            }
          }
          (0x2, 0x04) => {
            // amoxor.w

            let addr = self.gpr.read(rs1);
            if addr % 4 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, WORD)?;
            self.write(addr, (t as i32 ^ (self.gpr.read(rs2) as i32)) as i64 as u64, WORD)?;
            self.gpr.write(rd, t as i32 as i64 as u64);
          }
          (0x3, 0x04) => {
            // amoxor.d

            let addr = self.gpr.read(rs1);
            if addr % 8 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, DOUBLEWORD)?;
            self.write(addr, t ^ self.gpr.read(rs2), DOUBLEWORD)?;
            self.gpr.write(rd, t);
          }
          (0x2, 0x08) => {
            // amoor.w

            let addr = self.gpr.read(rs1);
            if addr % 4 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, WORD)?;
            self.write(addr, (t as i32 | (self.gpr.read(rs2) as i32)) as i64 as u64, WORD)?;
            self.gpr.write(rd, t as i32 as i64 as u64);
          }
          (0x3, 0x08) => {
            // amoor.d

            let addr = self.gpr.read(rs1);
            if addr % 8 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, DOUBLEWORD)?;
            self.write(addr, t | self.gpr.read(rs2), DOUBLEWORD)?;
            self.gpr.write(rd, t);
          }
          (0x2, 0x0c) => {
            // amoand.w

            let addr = self.gpr.read(rs1);
            if addr % 4 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, WORD)?;
            self.write(addr, (t as i32 & (self.gpr.read(rs2) as i32)) as u32 as u64, WORD)?;
            self.gpr.write(rd, t as i32 as i64 as u64);
          }
          (0x3, 0x0c) => {
            // amoand.d

            let addr = self.gpr.read(rs1);
            if addr % 8 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, DOUBLEWORD)?;
            self.write(addr, t & self.gpr.read(rs2), DOUBLEWORD)?;
            self.gpr.write(rd, t);
          }
          (0x2, 0x10) => {
            // amomin.w

            let addr = self.gpr.read(rs1);
            if addr % 4 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, WORD)?;
            self.write(addr, cmp::min(t as i32, self.gpr.read(rs2) as i32) as i64 as u64, WORD)?;
            self.gpr.write(rd, t as i32 as i64 as u64);
          }
          (0x3, 0x10) => {
            // amomin.d

            let addr = self.gpr.read(rs1);
            if addr % 8 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, DOUBLEWORD)?;
            self.write(addr, cmp::min(t as i64, self.gpr.read(rs2) as i64) as u64, DOUBLEWORD)?;
            self.gpr.write(rd, t as u64);
          }
          (0x2, 0x14) => {
            // amomax.w

            let addr = self.gpr.read(rs1);
            if addr % 4 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, WORD)?;
            self.write(addr, cmp::max(t as i32, self.gpr.read(rs2) as i32) as i64 as u64, WORD)?;
            self.gpr.write(rd, t as i32 as i64 as u64);
          }
          (0x3, 0x14) => {
            // amomax.d

            let addr = self.gpr.read(rs1);
            if addr % 8 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, DOUBLEWORD)?;
            self.write(addr, cmp::max(t as i64, self.gpr.read(rs2) as i64) as u64, DOUBLEWORD)?;
            self.gpr.write(rd, t);
          }
          (0x2, 0x18) => {
            // amominu.w

            let addr = self.gpr.read(rs1);
            if addr % 4 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, WORD)?;
            self.write(addr, cmp::min(t as u32, self.gpr.read(rs2) as u32) as u64, WORD)?;
            self.gpr.write(rd, t as i32 as i64 as u64);
          }
          (0x3, 0x18) => {
            // amominu.d

            let addr = self.gpr.read(rs1);
            if addr % 8 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, DOUBLEWORD)?;
            self.write(addr, cmp::min(t, self.gpr.read(rs2)), DOUBLEWORD)?;
            self.gpr.write(rd, t);
          }
          (0x2, 0x1c) => {
            // amomaxu.w

            let addr = self.gpr.read(rs1);
            if addr % 4 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, WORD)?;
            self.write(addr, cmp::max(t as u32, self.gpr.read(rs2) as u32) as u64, WORD)?;
            self.gpr.write(rd, t as i32 as i64 as u64);
          }
          (0x3, 0x1c) => {
            // amomaxu.d

            let addr = self.gpr.read(rs1);
            if addr % 8 != 0 {
              return Err(Exception::LoadAddressMisaligned);
            }
            let t = self.read(addr, DOUBLEWORD)?;
            self.write(addr, cmp::max(t, self.gpr.read(rs2)), DOUBLEWORD)?;
            self.gpr.write(rd, t);
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x33 => {
        // RV64I and RV64M
        match (funct3, funct7) {
          (0x0, 0x00) => {
            // add

            self.gpr.write(rd, self.gpr.read(rs1).wrapping_add(self.gpr.read(rs2)));
          }
          (0x0, 0x01) => {
            // mul

            self.gpr.write(
              rd,
              (self.gpr.read(rs1) as i64).wrapping_mul(self.gpr.read(rs2) as i64) as u64,
            );
          }
          (0x0, 0x20) => {
            // sub

            self.gpr.write(rd, self.gpr.read(rs1).wrapping_sub(self.gpr.read(rs2)));
          }
          (0x1, 0x00) => {
            // sll

            // "SLL, SRL, and SRA perform logical left, logical right, and arithmetic
            // right shifts on the value in register rs1 by the shift amount held in
            // register rs2. In RV64I, only the low 6 bits of rs2 are considered for the
            // shift amount."
            let shamt = self.gpr.read(rs2) & 0x3f;
            self.gpr.write(rd, self.gpr.read(rs1) << shamt);
          }
          (0x1, 0x01) => {
            // mulh

            // signed × signed
            self.gpr.write(
              rd,
              ((self.gpr.read(rs1) as i64 as i128).wrapping_mul(self.gpr.read(rs2) as i64 as i128) >> 64) as u64,
            );
          }
          (0x2, 0x00) => {
            // slt

            self.gpr.write(
              rd,
              if (self.gpr.read(rs1) as i64) < (self.gpr.read(rs2) as i64) {
                1
              } else {
                0
              },
            );
          }
          (0x2, 0x01) => {
            // mulhsu

            // signed × unsigned
            self.gpr.write(
              rd,
              ((self.gpr.read(rs1) as i64 as i128 as u128).wrapping_mul(self.gpr.read(rs2) as u128) >> 64) as u64,
            );
          }
          (0x3, 0x00) => {
            // sltu

            self
              .gpr
              .write(rd, if self.gpr.read(rs1) < self.gpr.read(rs2) { 1 } else { 0 });
          }
          (0x3, 0x01) => {
            // mulhu

            // unsigned × unsigned
            self.gpr.write(
              rd,
              ((self.gpr.read(rs1) as u128).wrapping_mul(self.gpr.read(rs2) as u128) >> 64) as u64,
            );
          }
          (0x4, 0x00) => {
            // xor

            self.gpr.write(rd, self.gpr.read(rs1) ^ self.gpr.read(rs2));
          }
          (0x4, 0x01) => {
            // div

            let dividend = self.gpr.read(rs1) as i64;
            let divisor = self.gpr.read(rs2) as i64;
            self.gpr.write(
              rd,
              if divisor == 0 {
                // Division by zero
                // Set DZ (Divide by Zero) flag to 1.
                self.csr.write_bit(FCSR, 3, 1);
                // "The quotient of division by zero has all bits set"
                u64::MAX
              } else if dividend == i64::MIN && divisor == -1 {
                // Overflow
                // "The quotient of a signed division with overflow is equal to the
                // dividend"
                dividend as u64
              } else {
                // "division of rs1 by rs2, rounding towards zero"
                dividend.wrapping_div(divisor) as u64
              },
            );
          }
          (0x5, 0x00) => {
            // srl

            // "SLL, SRL, and SRA perform logical left, logical right, and arithmetic
            // right shifts on the value in register rs1 by the shift amount held in
            // register rs2. In RV64I, only the low 6 bits of rs2 are considered for the
            // shift amount."
            let shamt = self.gpr.read(rs2) & 0x3f;
            self.gpr.write(rd, self.gpr.read(rs1) >> shamt);
          }
          (0x5, 0x01) => {
            // divu

            let dividend = self.gpr.read(rs1);
            let divisor = self.gpr.read(rs2);
            self.gpr.write(
              rd,
              if divisor == 0 {
                // Division by zero
                // Set DZ (Divide by Zero) flag to 1.
                self.csr.write_bit(FCSR, 3, 1);
                // "The quotient of division by zero has all bits set"
                u64::MAX
              } else {
                // "division of rs1 by rs2, rounding towards zero"
                dividend.wrapping_div(divisor)
              },
            );
          }
          (0x5, 0x20) => {
            // sra

            // "SLL, SRL, and SRA perform logical left, logical right, and arithmetic
            // right shifts on the value in register rs1 by the shift amount held in
            // register rs2. In RV64I, only the low 6 bits of rs2 are considered for the
            // shift amount."
            let shamt = self.gpr.read(rs2) & 0x3f;
            self.gpr.write(rd, ((self.gpr.read(rs1) as i64) >> shamt) as u64);
          }
          (0x6, 0x00) => {
            // or

            self.gpr.write(rd, self.gpr.read(rs1) | self.gpr.read(rs2));
          }
          (0x6, 0x01) => {
            // rem

            let dividend = self.gpr.read(rs1) as i64;
            let divisor = self.gpr.read(rs2) as i64;
            self.gpr.write(
              rd,
              if divisor == 0 {
                // Division by zero
                // "the remainder of division by zero equals the dividend"
                dividend as u64
              } else if dividend == i64::MIN && divisor == -1 {
                // Overflow
                // "the remainder is zero"
                0
              } else {
                // "provide the remainder of the corresponding division
                // operation"
                dividend.wrapping_rem(divisor) as u64
              },
            );
          }
          (0x7, 0x00) => {
            // and

            self.gpr.write(rd, self.gpr.read(rs1) & self.gpr.read(rs2));
          }
          (0x7, 0x01) => {
            // remu

            let dividend = self.gpr.read(rs1);
            let divisor = self.gpr.read(rs2);
            self.gpr.write(
              rd,
              if divisor == 0 {
                // Division by zero
                // "the remainder of division by zero equals the dividend"
                dividend
              } else {
                // "provide the remainder of the corresponding division
                // operation"
                dividend.wrapping_rem(divisor)
              },
            );
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        };
      }
      0x37 => {
        // RV32I
        // lui

        // "LUI places the U-immediate value in the top 20 bits of the destination
        // register rd, filling in the lowest 12 bits with zeros."
        self.gpr.write(rd, (inst & 0xfffff000) as i32 as i64 as u64);
      }
      0x3b => {
        // RV64I and RV64M
        match (funct3, funct7) {
          (0x0, 0x00) => {
            // addw

            self.gpr.write(
              rd,
              self.gpr.read(rs1).wrapping_add(self.gpr.read(rs2)) as i32 as i64 as u64,
            );
          }
          (0x0, 0x01) => {
            // mulw

            let n1 = self.gpr.read(rs1) as i32;
            let n2 = self.gpr.read(rs2) as i32;
            let result = n1.wrapping_mul(n2);
            self.gpr.write(rd, result as i64 as u64);
          }
          (0x0, 0x20) => {
            // subw

            self.gpr.write(
              rd,
              ((self.gpr.read(rs1).wrapping_sub(self.gpr.read(rs2))) as i32) as u64,
            );
          }
          (0x1, 0x00) => {
            // sllw

            // The shift amount is given by rs2[4:0].
            let shamt = self.gpr.read(rs2) & 0x1f;
            self.gpr.write(rd, ((self.gpr.read(rs1)) << shamt) as i32 as i64 as u64);
          }
          (0x4, 0x01) => {
            // divw

            let dividend = self.gpr.read(rs1) as i32;
            let divisor = self.gpr.read(rs2) as i32;
            self.gpr.write(
              rd,
              if divisor == 0 {
                // Division by zero
                // Set DZ (Divide by Zero) flag to 1.
                self.csr.write_bit(FCSR, 3, 1);
                // "The quotient of division by zero has all bits set"
                u64::MAX
              } else if dividend == i32::MIN && divisor == -1 {
                // Overflow
                // "The quotient of a signed division with overflow is equal to the
                // dividend"
                dividend as i64 as u64
              } else {
                // "division of rs1 by rs2, rounding towards zero"
                dividend.wrapping_div(divisor) as i64 as u64
              },
            );
          }
          (0x5, 0x00) => {
            // srlw

            // The shift amount is given by rs2[4:0].
            let shamt = self.gpr.read(rs2) & 0x1f;
            self
              .gpr
              .write(rd, ((self.gpr.read(rs1) as u32) >> shamt) as i32 as i64 as u64);
          }
          (0x5, 0x01) => {
            // divuw

            let dividend = self.gpr.read(rs1) as u32;
            let divisor = self.gpr.read(rs2) as u32;
            self.gpr.write(
              rd,
              if divisor == 0 {
                // Division by zero
                // Set DZ (Divide by Zero) flag to 1.
                self.csr.write_bit(FCSR, 3, 1);
                // "The quotient of division by zero has all bits set"
                u64::MAX
              } else {
                // "division of rs1 by rs2, rounding towards zero"
                dividend.wrapping_div(divisor) as i32 as i64 as u64
              },
            );
          }
          (0x5, 0x20) => {
            // sraw

            // The shift amount is given by rs2[4:0].
            let shamt = self.gpr.read(rs2) & 0x1f;
            self.gpr.write(rd, ((self.gpr.read(rs1) as i32) >> shamt) as i64 as u64);
          }
          (0x6, 0x01) => {
            // remw

            let dividend = self.gpr.read(rs1) as i32;
            let divisor = self.gpr.read(rs2) as i32;
            self.gpr.write(
              rd,
              if divisor == 0 {
                // Division by zero
                // "the remainder of division by zero equals the dividend"
                dividend as i64 as u64
              } else if dividend == i32::MIN && divisor == -1 {
                // Overflow
                // "the remainder is zero"
                0
              } else {
                // "provide the remainder of the corresponding division
                // operation"
                dividend.wrapping_rem(divisor) as i64 as u64
              },
            );
          }
          (0x7, 0x01) => {
            // remuw

            let dividend = self.gpr.read(rs1) as u32;
            let divisor = self.gpr.read(rs2) as u32;
            self.gpr.write(
              rd,
              if divisor == 0 {
                // Division by zero
                // "the remainder of division by zero equals the dividend"
                dividend as i32 as i64 as u64
              } else {
                // "provide the remainder of the corresponding division
                // operation"
                dividend.wrapping_rem(divisor) as i32 as i64 as u64
              },
            );
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x43 => {
        // RV32F and RV64F
        // TODO: support the rounding mode encoding (rm).
        let rs3 = ((inst & 0xf8000000) >> 27) as u64;
        let funct2 = (inst & 0x03000000) >> 25;
        match funct2 {
          0x0 => {
            // fmadd.s

            self.fpg.write(
              rd,
              (self.fpg.read(rs1) as f32).mul_add(self.fpg.read(rs2) as f32, self.fpg.read(rs3) as f32) as f64,
            );
          }
          0x1 => {
            // fmadd.d

            self
              .fpg
              .write(rd, self.fpg.read(rs1).mul_add(self.fpg.read(rs2), self.fpg.read(rs3)));
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x47 => {
        // RV32F and RV64F
        // TODO: support the rounding mode encoding (rm).
        let rs3 = ((inst & 0xf8000000) >> 27) as u64;
        let funct2 = (inst & 0x03000000) >> 25;
        match funct2 {
          0x0 => {
            // fmsub.s

            self.fpg.write(
              rd,
              (self.fpg.read(rs1) as f32).mul_add(self.fpg.read(rs2) as f32, -self.fpg.read(rs3) as f32) as f64,
            );
          }
          0x1 => {
            // fmsub.d

            self
              .fpg
              .write(rd, self.fpg.read(rs1).mul_add(self.fpg.read(rs2), -self.fpg.read(rs3)));
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x4b => {
        // RV32F and RV64F
        // TODO: support the rounding mode encoding (rm).
        let rs3 = ((inst & 0xf8000000) >> 27) as u64;
        let funct2 = (inst & 0x03000000) >> 25;
        match funct2 {
          0x0 => {
            // fnmadd.s

            self.fpg.write(
              rd,
              (-self.fpg.read(rs1) as f32).mul_add(self.fpg.read(rs2) as f32, self.fpg.read(rs3) as f32) as f64,
            );
          }
          0x1 => {
            // fnmadd.d

            self.fpg.write(
              rd,
              (-self.fpg.read(rs1)).mul_add(self.fpg.read(rs2), self.fpg.read(rs3)),
            );
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x4f => {
        // RV32F and RV64F
        // TODO: support the rounding mode encoding (rm).
        let rs3 = ((inst & 0xf8000000) >> 27) as u64;
        let funct2 = (inst & 0x03000000) >> 25;
        match funct2 {
          0x0 => {
            // fnmsub.s

            self.fpg.write(
              rd,
              (-self.fpg.read(rs1) as f32).mul_add(self.fpg.read(rs2) as f32, -self.fpg.read(rs3) as f32) as f64,
            );
          }
          0x1 => {
            // fnmsub.d

            self.fpg.write(
              rd,
              (-self.fpg.read(rs1)).mul_add(self.fpg.read(rs2), -self.fpg.read(rs3)),
            );
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x53 => {
        // RV32F and RV64F
        // TODO: support the rounding mode encoding (rm).
        // TODO: NaN Boxing of Narrower Values (Spec 12.2).
        // TODO: set exception flags.

        /*
         * Floating-point instructions align with the IEEE 754 (1985).
         * The format consist of three fields: a sign bit, a biased exponent, and a fraction.
         *
         * | sign(1) | exponent(8) | fraction(23) |
         * Ok => {}
         * 31                                     0
         *
         */

        // Check the frm field is valid.
        match self.csr.read_bits(FCSR, 5..8) {
          0b000 => {}
          0b001 => {}
          0b010 => {}
          0b011 => {}
          0b100 => {}
          0b111 => {}
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }

        match funct7 {
          0x00 => {
            // fadd.s

            self
              .fpg
              .write(rd, (self.fpg.read(rs1) as f32 + self.fpg.read(rs2) as f32) as f64)
          }
          0x01 => {
            // fadd.d

            self.fpg.write(rd, self.fpg.read(rs1) + self.fpg.read(rs2));
            self.fpg.fflags(rd);
          }
          0x04 => {
            // fsub.s

            self
              .fpg
              .write(rd, (self.fpg.read(rs1) as f32 - self.fpg.read(rs2) as f32) as f64)
          }
          0x05 => {
            // fsub.d

            self.fpg.write(rd, self.fpg.read(rs1) - self.fpg.read(rs2));
          }
          0x08 => {
            // fmul.s

            self
              .fpg
              .write(rd, (self.fpg.read(rs1) as f32 * self.fpg.read(rs2) as f32) as f64)
          }
          0x09 => {
            // fmul.d

            self.fpg.write(rd, self.fpg.read(rs1) * self.fpg.read(rs2));
          }
          0x0c => {
            // fdiv.s

            self
              .fpg
              .write(rd, (self.fpg.read(rs1) as f32 / self.fpg.read(rs2) as f32) as f64)
          }
          0x0d => {
            // fdiv.d

            self.fpg.write(rd, self.fpg.read(rs1) / self.fpg.read(rs2));
          }
          0x10 => {
            match funct3 {
              0x0 => {
                // fsgnj.s

                self.fpg.write(rd, self.fpg.read(rs1).copysign(self.fpg.read(rs2)));
              }
              0x1 => {
                // fsgnjn.s

                self.fpg.write(rd, self.fpg.read(rs1).copysign(-self.fpg.read(rs2)));
              }
              0x2 => {
                // fsgnjx.s

                let sign1 = (self.fpg.read(rs1) as f32).to_bits() & 0x80000000;
                let sign2 = (self.fpg.read(rs2) as f32).to_bits() & 0x80000000;
                let other = (self.fpg.read(rs1) as f32).to_bits() & 0x7fffffff;
                self.fpg.write(rd, f32::from_bits((sign1 ^ sign2) | other) as f64);
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x11 => {
            match funct3 {
              0x0 => {
                // fsgnj.d

                self.fpg.write(rd, self.fpg.read(rs1).copysign(self.fpg.read(rs2)));
              }
              0x1 => {
                // fsgnjn.d

                self.fpg.write(rd, self.fpg.read(rs1).copysign(-self.fpg.read(rs2)));
              }
              0x2 => {
                // fsgnjx.d

                let sign1 = self.fpg.read(rs1).to_bits() & 0x80000000_00000000;
                let sign2 = self.fpg.read(rs2).to_bits() & 0x80000000_00000000;
                let other = self.fpg.read(rs1).to_bits() & 0x7fffffff_ffffffff;
                self.fpg.write(rd, f64::from_bits((sign1 ^ sign2) | other));
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x14 => {
            match funct3 {
              0x0 => {
                // fmin.s

                self.fpg.write(rd, self.fpg.read(rs1).min(self.fpg.read(rs2)));
              }
              0x1 => {
                // fmax.s

                self.fpg.write(rd, self.fpg.read(rs1).max(self.fpg.read(rs2)));
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x15 => {
            match funct3 {
              0x0 => {
                // fmin.d

                self.fpg.write(rd, self.fpg.read(rs1).min(self.fpg.read(rs2)));
              }
              0x1 => {
                // fmax.d

                self.fpg.write(rd, self.fpg.read(rs1).max(self.fpg.read(rs2)));
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x20 => {
            // fcvt.s.d

            self.fpg.write(rd, self.fpg.read(rs1));
          }
          0x21 => {
            // fcvt.d.s

            self.fpg.write(rd, (self.fpg.read(rs1) as f32) as f64);
          }
          0x2c => {
            // fsqrt.s

            self.fpg.write(rd, (self.fpg.read(rs1) as f32).sqrt() as f64);
          }
          0x2d => {
            // fsqrt.d

            self.fpg.write(rd, self.fpg.read(rs1).sqrt());
          }
          0x50 => {
            match funct3 {
              0x0 => {
                // fle.s

                self
                  .gpr
                  .write(rd, if self.fpg.read(rs1) <= self.fpg.read(rs2) { 1 } else { 0 });
              }
              0x1 => {
                // flt.s

                self
                  .gpr
                  .write(rd, if self.fpg.read(rs1) < self.fpg.read(rs2) { 1 } else { 0 });
              }
              0x2 => {
                // feq.s

                self
                  .gpr
                  .write(rd, if self.fpg.read(rs1) == self.fpg.read(rs2) { 1 } else { 0 });
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x51 => {
            match funct3 {
              0x0 => {
                // fle.d

                self
                  .gpr
                  .write(rd, if self.fpg.read(rs1) <= self.fpg.read(rs2) { 1 } else { 0 });
              }
              0x1 => {
                // flt.d

                self
                  .gpr
                  .write(rd, if self.fpg.read(rs1) < self.fpg.read(rs2) { 1 } else { 0 });
              }
              0x2 => {
                // feq.d

                self
                  .gpr
                  .write(rd, if self.fpg.read(rs1) == self.fpg.read(rs2) { 1 } else { 0 });
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x60 => {
            match rs2 {
              0x0 => {
                // fcvt.w.s

                self.gpr.write(rd, ((self.fpg.read(rs1) as f32).round() as i32) as u64);
              }
              0x1 => {
                // fcvt.wu.s

                self
                  .gpr
                  .write(rd, (((self.fpg.read(rs1) as f32).round() as u32) as i32) as u64);
              }
              0x2 => {
                // fcvt.l.s

                self.gpr.write(rd, (self.fpg.read(rs1) as f32).round() as u64);
              }
              0x3 => {
                // fcvt.lu.s

                self.gpr.write(rd, (self.fpg.read(rs1) as f32).round() as u64);
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x61 => {
            match rs2 {
              0x0 => {
                // fcvt.w.d

                self.gpr.write(rd, (self.fpg.read(rs1).round() as i32) as u64);
              }
              0x1 => {
                // fcvt.wu.d

                self.gpr.write(rd, ((self.fpg.read(rs1).round() as u32) as i32) as u64);
              }
              0x2 => {
                // fcvt.l.d

                self.gpr.write(rd, self.fpg.read(rs1).round() as u64);
              }
              0x3 => {
                // fcvt.lu.d

                self.gpr.write(rd, self.fpg.read(rs1).round() as u64);
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x68 => {
            match rs2 {
              0x0 => {
                // fcvt.s.w

                self.fpg.write(rd, ((self.gpr.read(rs1) as i32) as f32) as f64);
              }
              0x1 => {
                // fcvt.s.wu

                self.fpg.write(rd, ((self.gpr.read(rs1) as u32) as f32) as f64);
              }
              0x2 => {
                // fcvt.s.l

                self.fpg.write(rd, (self.gpr.read(rs1) as f32) as f64);
              }
              0x3 => {
                // fcvt.s.lu

                self.fpg.write(rd, ((self.gpr.read(rs1) as u64) as f32) as f64);
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x69 => {
            match rs2 {
              0x0 => {
                // fcvt.d.w

                self.fpg.write(rd, (self.gpr.read(rs1) as i32) as f64);
              }
              0x1 => {
                // fcvt.d.wu

                self.fpg.write(rd, (self.gpr.read(rs1) as u32) as f64);
              }
              0x2 => {
                // fcvt.d.l

                self.fpg.write(rd, self.gpr.read(rs1) as f64);
              }
              0x3 => {
                // fcvt.d.lu

                self.fpg.write(rd, self.gpr.read(rs1) as f64);
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x70 => {
            match funct3 {
              0x0 => {
                // fmv.x.w

                self
                  .gpr
                  .write(rd, (self.fpg.read(rs1).to_bits() & 0xffffffff) as i32 as i64 as u64);
              }
              0x1 => {
                // fclass.s

                let f = self.fpg.read(rs1);
                match f.classify() {
                  FpCategory::Infinite => {
                    self.gpr.write(rd, if f.is_sign_negative() { 0 } else { 7 });
                  }
                  FpCategory::Normal => {
                    self.gpr.write(rd, if f.is_sign_negative() { 1 } else { 6 });
                  }
                  FpCategory::Subnormal => {
                    self.gpr.write(rd, if f.is_sign_negative() { 2 } else { 5 });
                  }
                  FpCategory::Zero => {
                    self.gpr.write(rd, if f.is_sign_negative() { 3 } else { 4 });
                  }
                  // don't support a signaling nan, only support a quiet nan.
                  FpCategory::Nan => self.gpr.write(rd, 9),
                }
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x71 => {
            match funct3 {
              0x0 => {
                // fmv.x.d

                // "FMV.X.D and FMV.D.X do not modify the bits being transferred"
                self.gpr.write(rd, self.fpg.read(rs1).to_bits());
              }
              0x1 => {
                // fclass.d

                let f = self.fpg.read(rs1);
                match f.classify() {
                  FpCategory::Infinite => {
                    self.gpr.write(rd, if f.is_sign_negative() { 0 } else { 7 });
                  }
                  FpCategory::Normal => {
                    self.gpr.write(rd, if f.is_sign_negative() { 1 } else { 6 });
                  }
                  FpCategory::Subnormal => {
                    self.gpr.write(rd, if f.is_sign_negative() { 2 } else { 5 });
                  }
                  FpCategory::Zero => {
                    self.gpr.write(rd, if f.is_sign_negative() { 3 } else { 4 });
                  }
                  // don't support a signaling nan, only support a quiet nan.
                  FpCategory::Nan => self.gpr.write(rd, 9),
                }
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x78 => {
            // fmv.w.x

            self.fpg.write(rd, f64::from_bits(self.gpr.read(rs1) & 0xffffffff));
          }
          0x79 => {
            // fmv.d.x

            // "FMV.X.D and FMV.D.X do not modify the bits being transferred"
            self.fpg.write(rd, f64::from_bits(self.gpr.read(rs1)));
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x63 => {
        // RV32I
        // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
        let imm = (((inst & 0x80000000) as i32 as i64 >> 19) as u64)
                    | ((inst & 0x80) << 4) // imm[11]
                    | ((inst >> 20) & 0x7e0) // imm[10:5]
                    | ((inst >> 7) & 0x1e); // imm[4:1]

        match funct3 {
          0x0 => {
            // beq

            if self.gpr.read(rs1) == self.gpr.read(rs2) {
              self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
          }
          0x1 => {
            // bne

            if self.gpr.read(rs1) != self.gpr.read(rs2) {
              self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
          }
          0x4 => {
            // blt

            if (self.gpr.read(rs1) as i64) < (self.gpr.read(rs2) as i64) {
              self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
          }
          0x5 => {
            // bge

            if (self.gpr.read(rs1) as i64) >= (self.gpr.read(rs2) as i64) {
              self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
          }
          0x6 => {
            // bltu

            if self.gpr.read(rs1) < self.gpr.read(rs2) {
              self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
          }
          0x7 => {
            // bgeu

            if self.gpr.read(rs1) >= self.gpr.read(rs2) {
              self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      0x67 => {
        // jalr

        let t = self.pc.wrapping_add(4);

        let offset = (inst as i32 as i64) >> 20;
        let target = ((self.gpr.read(rs1) as i64).wrapping_add(offset)) & !1;

        self.pc = (target as u64).wrapping_sub(4);

        self.gpr.write(rd, t);
      }
      0x6F => {
        // jal

        self.gpr.write(rd, self.pc.wrapping_add(4));

        // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
        let offset = (((inst & 0x80000000) as i32 as i64 >> 11) as u64) // imm[20]
                    | (inst & 0xff000) // imm[19:12]
                    | ((inst >> 9) & 0x800) // imm[11]
                    | ((inst >> 20) & 0x7fe); // imm[10:1]

        self.pc = self.pc.wrapping_add(offset).wrapping_sub(4);
      }
      0x73 => {
        // RV32I, RVZicsr, and supervisor ISA
        let csr_addr = ((inst >> 20) & 0xfff) as u16;
        match funct3 {
          0x0 => {
            match (rs2, funct7) {
              (0x0, 0x0) => {
                // ecall

                // Makes a request of the execution environment by raising an
                // environment call exception.
                match self.mode {
                  Mode::User => {
                    return Err(Exception::EnvironmentCallFromUMode);
                  }
                  Mode::Supervisor => {
                    return Err(Exception::EnvironmentCallFromSMode);
                  }
                  Mode::Machine => {
                    return Err(Exception::EnvironmentCallFromMMode);
                  }
                  _ => {
                    return Err(Exception::IllegalInstruction(inst));
                  }
                }
              }
              (0x1, 0x0) => {
                // ebreak

                // Makes a request of the debugger bu raising a Breakpoint
                // exception.
                return Err(Exception::Breakpoint);
              }
              (0x2, 0x0) => {
                // uret
                panic!("uret: not implemented yet. pc {}", self.pc);
              }
              (0x2, 0x8) => {
                // sret

                // "The RISC-V Reader" book says:
                // "Returns from a supervisor-mode exception handler. Sets the pc to
                // CSRs[sepc], the privilege mode to CSRs[sstatus].SPP,
                // CSRs[sstatus].SIE to CSRs[sstatus].SPIE, CSRs[sstatus].SPIE to
                // 1, and CSRs[sstatus].SPP to 0.", but the implementation in QEMU
                // and Spike use `mstatus` instead of `sstatus`.

                // Set the program counter to the supervisor exception program
                // counter (SEPC).
                self.pc = self.csr.read(SEPC).wrapping_sub(4);

                // TODO: Check TSR field

                // Set the current privileged mode depending on a previous
                // privilege mode for supervisor mode (SPP, 8).
                self.mode = match self.csr.read_sstatus(XSTATUS_SPP) {
                  0b0 => Mode::User,
                  0b1 => {
                    // If SPP != M-mode, SRET also sets MPRV=0.
                    self.csr.write_mstatus(MSTATUS_MPRV, 0);
                    Mode::Supervisor
                  }
                  _ => Mode::Debug,
                };

                // Read a previous interrupt-enable bit for supervisor mode (SPIE,
                // 5), and set a global interrupt-enable bit for supervisor mode
                // (SIE, 1) to it.
                self.csr.write_sstatus(XSTATUS_SIE, self.csr.read_sstatus(XSTATUS_SPIE));

                // Set a previous interrupt-enable bit for supervisor mode (SPIE,
                // 5) to 1.
                self.csr.write_sstatus(XSTATUS_SPIE, 1);
                // Set a previous privilege mode for supervisor mode (SPP, 8) to 0.
                self.csr.write_sstatus(XSTATUS_SPP, 0);
              }
              (0x2, 0x18) => {
                // mret

                // "The RISC-V Reader" book says:
                // "Returns from a machine-mode exception handler. Sets the pc to
                // CSRs[mepc], the privilege mode to CSRs[mstatus].MPP,
                // CSRs[mstatus].MIE to CSRs[mstatus].MPIE, and CSRs[mstatus].MPIE
                // to 1; and, if user mode is supported, sets CSRs[mstatus].MPP to
                // 0".

                // Set the program counter to the machine exception program
                // counter (MEPC).
                self.pc = self.csr.read(MEPC).wrapping_sub(4);

                // Set the current privileged mode depending on a previous
                // privilege mode for machine  mode (MPP, 11..13).
                self.mode = match self.csr.read_mstatus(MSTATUS_MPP) {
                  0b00 => {
                    // If MPP != M-mode, MRET also sets MPRV=0.
                    self.csr.write_mstatus(MSTATUS_MPRV, 0);
                    Mode::User
                  }
                  0b01 => {
                    // If MPP != M-mode, MRET also sets MPRV=0.
                    self.csr.write_mstatus(MSTATUS_MPRV, 0);
                    Mode::Supervisor
                  }
                  0b11 => Mode::Machine,
                  _ => Mode::Debug,
                };

                // Read a previous interrupt-enable bit for machine mode (MPIE, 7),
                // and set a global interrupt-enable bit for machine mode (MIE, 3)
                // to it.
                self.csr.write_mstatus(MSTATUS_MIE, self.csr.read_mstatus(MSTATUS_MPIE));

                // Set a previous interrupt-enable bit for machine mode (MPIE, 7)
                // to 1.
                self.csr.write_mstatus(MSTATUS_MPIE, 1);

                // Set a previous privilege mode for machine mode (MPP, 11..13) to
                // 0.
                self.csr.write_mstatus(MSTATUS_MPP, Mode::User as u64);
              }
              (0x5, 0x8) => {
                // wfi
                // "provides a hint to the implementation that the current
                // hart can be stalled until an interrupt might need servicing."
                self.idle = true;
              }
              (_, 0x9) => {
                // sfence.vma
                // "SFENCE.VMA is used to synchronize updates to in-memory
                // memory-management data structures with current execution"
              }
              (_, 0x11) => {
                // hfence.bvma
              }
              (_, 0x51) => {
                // hfence.gvma
              }
              _ => {
                return Err(Exception::IllegalInstruction(inst));
              }
            }
          }
          0x1 => {
            // csrrw

            let t = self.csr.read(csr_addr);
            self.csr.write(csr_addr, self.gpr.read(rs1));
            self.gpr.write(rd, t);

            if csr_addr == SATP {
              self.update_paging();
            }
          }
          0x2 => {
            // csrrs

            let t = self.csr.read(csr_addr);
            self.csr.write(csr_addr, t | self.gpr.read(rs1));
            self.gpr.write(rd, t);

            if csr_addr == SATP {
              self.update_paging();
            }
          }
          0x3 => {
            // csrrc

            let t = self.csr.read(csr_addr);
            self.csr.write(csr_addr, t & (!self.gpr.read(rs1)));
            self.gpr.write(rd, t);

            if csr_addr == SATP {
              self.update_paging();
            }
          }
          0x5 => {
            // csrrwi

            let zimm = rs1;
            self.gpr.write(rd, self.csr.read(csr_addr));
            self.csr.write(csr_addr, zimm);

            if csr_addr == SATP {
              self.update_paging();
            }
          }
          0x6 => {
            // csrrsi

            let zimm = rs1;
            let t = self.csr.read(csr_addr);
            self.csr.write(csr_addr, t | zimm);
            self.gpr.write(rd, t);

            if csr_addr == SATP {
              self.update_paging();
            }
          }
          0x7 => {
            // csrrci

            let zimm = rs1;
            let t = self.csr.read(csr_addr);
            self.csr.write(csr_addr, t & (!zimm));
            self.gpr.write(rd, t);

            if csr_addr == SATP {
              self.update_paging();
            }
          }
          _ => {
            return Err(Exception::IllegalInstruction(inst));
          }
        }
      }
      _ => {
        return Err(Exception::IllegalInstruction(inst));
      }
    }
    Ok(())
  }
}
