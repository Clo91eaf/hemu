mod cpu;

pub struct CpuState {
    gpr: [usize; 32],
    pc: usize,
}

struct Decode {
    pc: usize,
    snpc: usize,
    dnpc: usize,
    inst: u32,
}

lazy_static! {
    static ref CPU: CpuState = CpuState {
        gpr: [0; 32],
        pc: 0x80000000,
    };
}

fn fetch(pc: u32) -> u32 {
    
}

fn exec_once(s: Decode, pc: usize) {
    s.pc = pc;
    s.snpc = pc;
    s.inst = 
    cpu.pc = s.dnpc;
}