//! This file is automatically generated using psxunittest:
//! https://github.com/daeken/psxunittest
//!
//! /!\ DO NOT EDIT DIRECTLY /!\

use gpu::{Gpu, VideoClock};
use gpu::renderer::{Renderer, PrimitiveAttributes, Vertex};
use memory::{Interconnect, Addressable};
use memory;
use shared::SharedState;
use bios::Bios;

use super::{Cpu, RegisterIndex};

/// Dummy GPU renderer to run the tests
struct DummyRenderer;

impl Renderer for DummyRenderer {
    fn set_draw_offset(&mut self, _: i16, _: i16) {
    }

    fn set_draw_area(&mut self, _: (u16, u16), _: (u16, u16)) {
    }

    fn set_display_mode(&mut self,
                        _: (u16, u16),
                        _: (u16, u16),
                        _: bool) {
    }

    fn push_line(&mut self, _: &PrimitiveAttributes, _: &[Vertex; 2]) {
    }

    fn push_triangle(&mut self, _: &PrimitiveAttributes, _: &[Vertex; 3]) {
    }

    fn push_quad(&mut self, _: &PrimitiveAttributes, _: &[Vertex; 4]) {
    }

    fn fill_rect(&mut self,
                 _: [u8; 3],
                 _: (u16, u16),
                 _: (u16, u16)) {
    }

    fn load_image(&mut self,
                  _: (u16, u16),
                  _: (u16, u16),
                  _: &[u16]) {
    }
}

fn write_blob(cpu: &mut Cpu,
             address: u32,
             blob: &[u32]) {
    let ram = cpu.interconnect_mut().ram_mut();

    for (i, &w) in blob.iter().enumerate() {
        ram.store::<memory::Word>(address + (i * 4) as u32, w);
    }
}

fn write<T: Addressable>(cpu: &mut Cpu,
                         address: u32,
                         v: u32) {
    let ram = cpu.interconnect_mut().ram_mut();

    ram.store::<T>(address, v);
}

fn read<T: Addressable>(cpu: &mut Cpu, address: u32) -> u32 {

    let ram = cpu.interconnect().ram();

    ram.load::<T>(address)
}

#[test]
fn test_beq() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }

    cpu.set_reg(RegisterIndex(1), 0x1);
    cpu.set_reg(RegisterIndex(2), 0x2);
    cpu.set_reg(RegisterIndex(3), -1i32 as u32);
    cpu.set_reg(RegisterIndex(4), 0xffffffff);

    write_blob(&mut cpu, 0x80100000,
               &[0x10220005,
                 0x00000000,
                 0x200a0001,
                 0x10640004,
                 0x00000000,
                 0x200b0001,
                 0x200a0002,
                 0x00000000,
                 0x00000000,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(cpu.regs[10] == 0x1);
    assert!(cpu.regs[11] == 0);
}

#[test]
fn test_branch_in_branch_delay() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }


    write_blob(&mut cpu, 0x80100000,
               &[0x10000002,
                 0x10000004,
                 0x20030001,
                 0x20010001,
                 0x10000002,
                 0x00000000,
                 0x20020001,
                 0x00000000,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(cpu.regs[1] == 0x1);
    assert!(cpu.regs[2] == 0);
    assert!(cpu.regs[3] == 0);
}

#[test]
fn test_lwr_and_lwr_load_delay() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }

    write::<memory::Word>(&mut cpu, 0, 0x76543210);
    write::<memory::Word>(&mut cpu, 0x4, 0xfedcba98);

    write_blob(&mut cpu, 0x80100000,
               &[0x2401ffff,
                 0x98010002,
                 0x88010005,
                 0x00201021,
                 0x2403ffff,
                 0x98030002,
                 0x00000000,
                 0x88030005,
                 0x00602021,
                 0x2405ffff,
                 0x88050005,
                 0x00000000,
                 0x98050002,
                 0x00a03021,
                 0x2407ffff,
                 0x8c070004,
                 0x88070002,
                 0x00e04021,
                 0x2409ffff,
                 0x8c090004,
                 0x00000000,
                 0x88090002,
                 0x01205021,
                 0x240bffff,
                 0x8c0b0004,
                 0x980b0002,
                 0x01606021,
                 0x240dffff,
                 0x8c0d0004,
                 0x00000000,
                 0x980d0002,
                 0x01a07021,
                 0x3c0f067e,
                 0x35ef067e,
                 0x488fc800,
                 0x240fffff,
                 0x480fc800,
                 0x880f0001,
                 0x01e08021,
                 0x2411ffff,
                 0x4811c800,
                 0x00000000,
                 0x98110001,
                 0x02209021,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(cpu.regs[1] == 0xba987654);
    assert!(cpu.regs[2] == 0xffffffff);
    assert!(cpu.regs[3] == 0xba987654);
    assert!(cpu.regs[4] == 0xffff7654);
    assert!(cpu.regs[5] == 0xba987654);
    assert!(cpu.regs[6] == 0xba98ffff);
    assert!(cpu.regs[7] == 0x54321098);
    assert!(cpu.regs[8] == 0xffffffff);
    assert!(cpu.regs[9] == 0x54321098);
    assert!(cpu.regs[10] == 0xfedcba98);
    assert!(cpu.regs[11] == 0xfedc7654);
    assert!(cpu.regs[12] == 0xffffffff);
    assert!(cpu.regs[13] == 0xfedc7654);
    assert!(cpu.regs[14] == 0xfedcba98);
    assert!(cpu.regs[15] == 0x3210067e);
    assert!(cpu.regs[16] == 0xffffffff);
    assert!(cpu.regs[17] == 0x6765432);
    assert!(cpu.regs[18] == 0x67e067e);
}

#[test]
fn test_add_1() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }

    cpu.set_reg(RegisterIndex(1), 0xa);
    cpu.set_reg(RegisterIndex(2), -15i32 as u32);

    write_blob(&mut cpu, 0x80100000,
               &[0x00201820,
                 0x00222020,
                 0x00412820,
                 0x00423020,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(cpu.regs[1] == 0xa);
    assert!(cpu.regs[2] == -15i32 as u32);
    assert!(cpu.regs[3] == 0xa);
    assert!(cpu.regs[4] == -5i32 as u32);
    assert!(cpu.regs[5] == -5i32 as u32);
    assert!(cpu.regs[6] == -30i32 as u32);
}

#[test]
fn test_arithmetic_branching_test() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }

    cpu.set_reg(RegisterIndex(2), 0xdead);
    cpu.set_reg(RegisterIndex(3), 0);
    cpu.set_reg(RegisterIndex(5), 0x1);

    write_blob(&mut cpu, 0x80100000,
               &[0x00451023,
                 0x24630001,
                 0x1c40fffd,
                 0x00000000,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(cpu.regs[2] == 0);
    assert!(cpu.regs[3] == 0xdead);
    assert!(cpu.regs[5] == 0x1);
}

#[test]
fn test_bltzal_and_bgezal() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }


    write_blob(&mut cpu, 0x80100000,
               &[0x3c05ffff,
                 0x34a5ffff,
                 0x00000821,
                 0x0000f821,
                 0x04100002,
                 0x00000000,
                 0x34010001,
                 0x001f102b,
                 0x3c03ffff,
                 0x3463ffff,
                 0x0000f821,
                 0x04710002,
                 0x00000000,
                 0x34030001,
                 0x001f202b,
                 0x3c05ffff,
                 0x34a5ffff,
                 0x0000f821,
                 0x04b00002,
                 0x00000000,
                 0x34050001,
                 0x001f302b,
                 0x00003821,
                 0x0000f821,
                 0x04110002,
                 0x00000000,
                 0x34070001,
                 0x001f402b,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(cpu.regs[1] == 0x1);
    assert!(cpu.regs[2] == 0x1);
    assert!(cpu.regs[3] == 0x1);
    assert!(cpu.regs[4] == 0x1);
    assert!(cpu.regs[5] == -1i32 as u32);
    assert!(cpu.regs[6] == 0x1);
    assert!(cpu.regs[7] == 0);
    assert!(cpu.regs[8] == 0x1);
}

#[test]
fn test_unaligned_loads() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }

    write::<memory::Word>(&mut cpu, 0xbee0, 0xdeadbeef);
    cpu.set_reg(RegisterIndex(30), 0xbee1);

    write_blob(&mut cpu, 0x80100000,
               &[0x83c10000,
                 0x93c20000,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(cpu.regs[1] == -66i32 as u32);
    assert!(cpu.regs[2] == 0xbe);
    assert!(cpu.regs[3] == 0);
    assert!(cpu.regs[4] == 0);
}

#[test]
fn test_load_delay_for_cop() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }

    cpu.set_reg(RegisterIndex(2), 0x80110000);
    write::<memory::Word>(&mut cpu, 0x80110000, 0xdeadbeef);

    write_blob(&mut cpu, 0x80100000,
               &[0x8c430000,
                 0x00000000,
                 0x4803c800,
                 0x10600004,
                 0x00000000,
                 0x20010001,
                 0x0804000a,
                 0x00000000,
                 0x20010002,
                 0x0804000a,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(cpu.regs[3] == 0);
    assert!(cpu.regs[1] == 0x1);
}

#[test]
fn test_swl_and_swr() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }

    cpu.set_reg(RegisterIndex(1), 0);
    cpu.set_reg(RegisterIndex(2), 0x76543210);
    cpu.set_reg(RegisterIndex(3), 0xfedcba98);

    write_blob(&mut cpu, 0x80100000,
               &[0xac220000,
                 0xa8230000,
                 0x24210004,
                 0xac220000,
                 0xa8230001,
                 0x24210004,
                 0xac220000,
                 0xa8230002,
                 0x24210004,
                 0xac220000,
                 0xa8230003,
                 0x24210004,
                 0xac220000,
                 0xb8230000,
                 0x24210004,
                 0xac220000,
                 0xb8230001,
                 0x24210004,
                 0xac220000,
                 0xb8230002,
                 0x24210004,
                 0xac220000,
                 0xb8230003,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(read::<memory::Word>(&mut cpu, 0) == 0x765432fe);
    assert!(read::<memory::Word>(&mut cpu, 0x4) == 0x7654fedc);
    assert!(read::<memory::Word>(&mut cpu, 0x8) == 0x76fedcba);
    assert!(read::<memory::Word>(&mut cpu, 0xc) == 0xfedcba98);
    assert!(read::<memory::Word>(&mut cpu, 0x10) == 0xfedcba98);
    assert!(read::<memory::Word>(&mut cpu, 0x14) == 0xdcba9810);
    assert!(read::<memory::Word>(&mut cpu, 0x18) == 0xba983210);
    assert!(read::<memory::Word>(&mut cpu, 0x1c) == 0x98543210);
}

#[test]
fn test_multiple_load_cancelling() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }

    write::<memory::Word>(&mut cpu, 0, 0x7001a7e);
    cpu.set_reg(RegisterIndex(1), 0x600dc0de);

    write_blob(&mut cpu, 0x80100000,
               &[0x40016000,
                 0x8c010000,
                 0x40017800,
                 0x8c010000,
                 0x8c010000,
                 0x00201021,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(cpu.regs[1] == 0x7001a7e);
    assert!(cpu.regs[2] == 0x600dc0de);
}

#[test]
fn test_lwl_and_lwr() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }

    write::<memory::Word>(&mut cpu, 0, 0x76543210);
    write::<memory::Word>(&mut cpu, 0x4, 0xfedcba98);

    write_blob(&mut cpu, 0x80100000,
               &[0x98010000,
                 0x88010003,
                 0x98020001,
                 0x88020004,
                 0x98030002,
                 0x88030005,
                 0x98040003,
                 0x88040006,
                 0x98050004,
                 0x88050007,
                 0x88060003,
                 0x98060000,
                 0x88070004,
                 0x98070001,
                 0x88080005,
                 0x98080002,
                 0x88090006,
                 0x98090003,
                 0x880a0007,
                 0x980a0004,
                 0x240bffff,
                 0x880b0000,
                 0x240cffff,
                 0x980c0000,
                 0x240dffff,
                 0x880d0001,
                 0x240effff,
                 0x980e0001,
                 0x240fffff,
                 0x880f0002,
                 0x2410ffff,
                 0x98100002,
                 0x2411ffff,
                 0x88110003,
                 0x2412ffff,
                 0x98120003,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(cpu.regs[1] == 0x76543210);
    assert!(cpu.regs[2] == 0x98765432);
    assert!(cpu.regs[3] == 0xba987654);
    assert!(cpu.regs[4] == 0xdcba9876);
    assert!(cpu.regs[5] == 0xfedcba98);
    assert!(cpu.regs[6] == 0x76543210);
    assert!(cpu.regs[7] == 0x98765432);
    assert!(cpu.regs[8] == 0xba987654);
    assert!(cpu.regs[9] == 0xdcba9876);
    assert!(cpu.regs[10] == 0xfedcba98);
    assert!(cpu.regs[11] == 0x10ffffff);
    assert!(cpu.regs[12] == 0x76543210);
    assert!(cpu.regs[13] == 0x3210ffff);
    assert!(cpu.regs[14] == 0xff765432);
    assert!(cpu.regs[15] == 0x543210ff);
    assert!(cpu.regs[16] == 0xffff7654);
    assert!(cpu.regs[17] == 0x76543210);
    assert!(cpu.regs[18] == 0xffffff76);
}

#[test]
fn test_lh_and_lb_sign_extension() {
    let bios = Bios::dummy();
    let gpu = Gpu::new(VideoClock::Ntsc);
    let inter = Interconnect::new(bios, gpu, None);
    let mut cpu = Cpu::new(inter);
    let mut shared = SharedState::new();
    let mut renderer = DummyRenderer;

    for r in 0..31 {
        cpu.set_reg(RegisterIndex(r), 0);
    }

    write::<memory::Word>(&mut cpu, 0, 0x8080);

    write_blob(&mut cpu, 0x80100000,
               &[0x84010000,
                 0x94020000,
                 0x80030000,
                 0x90040000,
                 0x00000000,
                 0x0bab6fb8,
                 0x00000000]);

    cpu.set_pc(0x80100000);

    let mut timeout = true;
    for _ in 0..TIMEOUT {
        if (cpu.pc & 0x0fffffff) == 0xeadbee0 {
            timeout = false;
            break;
        }
        cpu.run_next_instruction(&mut (), &mut shared, &mut renderer);
    }
    assert!(timeout == false);

    assert!(cpu.regs[1] == 0xffff8080);
    assert!(cpu.regs[2] == 0x8080);
    assert!(cpu.regs[3] == 0xffffff80);
    assert!(cpu.regs[4] == 0x80);
}

/// Number of CPU cycles after which we consider the test to be a
/// failure
const TIMEOUT: usize = 1_000_000;

