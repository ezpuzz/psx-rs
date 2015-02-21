use super::{Instruction, RegisterIndex};

pub fn decode(instruction: Instruction) -> String {
    match instruction.function() {
        0b000000 => match instruction.subfunction() {
            0b000000 => op_sll(instruction),
            0b100101 => op_or(instruction),
            _        => format!("!UNKNOWN!"),
        },
        0b000010 => op_j(instruction),
        0b001001 => op_addiu(instruction),
        0b001101 => op_ori(instruction),
        0b001111 => op_lui(instruction),
        0b010000 => op_cop0(instruction),
        0b101011 => op_sw(instruction),
        _        => format!("!UNKNOWN!"),
    }
}

fn reg(idx: RegisterIndex) -> &'static str {
    super::REGISTER_MNEMONICS[idx.0 as usize]
}

fn op_sll(instruction: Instruction) -> String {
    let i = instruction.shift();
    let t = instruction.t();
    let d = instruction.d();

    format!("sll {}, {} << {}", reg(d), reg(t), i)
}

fn op_or(instruction: Instruction) -> String {
    let d = instruction.d();
    let s = instruction.s();
    let t = instruction.t();

    format!("or {}, {}, {}", reg(d), reg(s), reg(t))
}

fn op_j(instruction: Instruction) -> String {
    let i = instruction.imm_jump();

    format!("J (PC & 0xf0000000) | {:x}", i << 2)
}

fn op_addiu(instruction: Instruction) -> String {
    let i = instruction.imm_se();
    let t = instruction.t();
    let s = instruction.s();

    format!("addiu {}, {}, 0x{:x}", reg(t), reg(s), i)
}

fn op_ori(instruction: Instruction) -> String {
    let i = instruction.imm();
    let t = instruction.t();
    let s = instruction.s();

    format!("ori {}, {}, 0x{:x}", reg(t), reg(s), i)
}

fn op_lui(instruction: Instruction) -> String {
    let i = instruction.imm();
    let t = instruction.t();

    format!("lui {}, 0x{:x}", reg(t), i)
}

fn op_cop0(instruction: Instruction) -> String {
    match instruction.cop_opcode() {
        0b00100 => op_mtc0(instruction),
        _       => format!("!UNKNOWN cop0 instruction {}!", instruction)
    }
}

fn op_mtc0(instruction: Instruction) -> String{
    let cpu_r = instruction.t();
    let cop_r = instruction.d().0;

    format!("mtc0 {}, cop0_{}", reg(cpu_r), cop_r)
}

fn op_sw(instruction: Instruction) -> String {
    let i = instruction.imm_se() as i16;
    let t = instruction.t();
    let s = instruction.s();

    format!("sw {}, [{} + 0x{:x}]", reg(t), reg(s), i)
}
