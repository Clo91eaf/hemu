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

fn fetch(s: Decode) -> u32 {
    let inst = unsafe {
        std::slice::from_raw_parts(s.pc as *const u8, 4)
    };
    let inst = u32::from_le_bytes([inst[0], inst[1], inst[2], inst[3]]);
    s.inst = inst;
    s.dnpc = s.pc + 4;
    inst 
}

fn decode(s: Decode) -> u32 {

}

fn exec_once(s: Decode, pc: usize) {
    s.pc = pc;
    s.snpc = pc;
    // fetch stage
    s.inst = fetch(s);
    // decode stage
    cpu.pc = s.dnpc;
}