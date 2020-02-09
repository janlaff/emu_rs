use super::*;

#[test]
fn test_opcode_00E0() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));

    for y in 0..chip8::GFX_BUF_HEIGHT {
        for x in 0..chip8::GFX_BUF_WIDTH {
            gpu.borrow_mut().write(x, y, 255);
        }
    }

    let mut cpu = chip8::CPU::new(&gpu);
    cpu.load_program(&[0x00, 0xE0]);
    cpu.execute();

    for y in 0..chip8::GFX_BUF_HEIGHT {
        for x in 0..chip8::GFX_BUF_WIDTH {
            assert_eq!(gpu.borrow().read(x, y), 0);
        }
    }
}

#[test]
fn test_opcode_00EE() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    // Store address on stack
    cpu.stack[cpu.sp as usize] = 0xFFFF;
    cpu.sp += 1;

    cpu.load_program(&[0x00, 0xEE]);
    cpu.execute();

    assert_eq!(cpu.pc, 0xFFFF);
    assert_eq!(cpu.sp, 0);
}

#[test]
fn test_opcode_1NNN() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.load_program(&[0x1F, 0xFF]);
    cpu.execute();

    assert_eq!(cpu.pc, 0x0FFF);
}

#[test]
fn test_opcode_2NNN() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.load_program(&[0x2F, 0xFF]);
    cpu.execute();

    assert_eq!(cpu.sp, 1);
    assert_eq!(cpu.stack[0], chip8::PROGRAM_ENTRY + 2);
    assert_eq!(cpu.pc, 0x0FFF);
}

#[test]
fn test_opcode_3XNN() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.load_program(&[0x30, 0xFF]);
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 2);

    cpu.pc = 0x200;
    cpu.regs[chip8::V0] = 0xFF;
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 4);
}

#[test]
fn test_opcode_4XNN() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.load_program(&[0x40, 0xFF]);
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 4);

    cpu.pc = 0x200;
    cpu.regs[chip8::V0] = 0xFF;
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 2);
}

#[test]
fn test_opcode_5XY0() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.load_program(&[0x50, 0x10]);
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 4);

    cpu.regs[chip8::V0] = 0xFF;
    cpu.pc = chip8::PROGRAM_ENTRY;
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 2);
}

#[test]
fn test_opcode_6XNN() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.load_program(&[0x60, 0xFF]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 0xFF);
}

#[test]
fn test_opcode_7XNN() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 10;
    cpu.load_program(&[0x70, 20]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 10 + 20);
}

#[test]
fn test_opcode_8XY0() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V1] = 0xFF;
    cpu.load_program(&[0x80, 0x10]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 0xFF);
}

#[test]
fn test_opcode_8XY1() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 0b01010101;
    cpu.regs[chip8::V1] = 0b10101010;

    cpu.load_program(&[0x80, 0x11]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 0xFF);
}

#[test]
fn test_opcode_8XY2() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 0b01010101;
    cpu.regs[chip8::V1] = 0b00000001;

    cpu.load_program(&[0x80, 0x12]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 1);
}

#[test]
fn test_opcode_8XY3() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 0b01010101;
    cpu.regs[chip8::V1] = 0b10101010;

    cpu.load_program(&[0x80, 0x13]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 0xFF);
}

#[test]
fn test_opcode_8XY4() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 10;
    cpu.regs[chip8::V1] = 20;

    cpu.load_program(&[0x80, 0x14]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 10 + 20);
    assert_eq!(cpu.regs[chip8::VF], 0);

    cpu.regs[chip8::V0] = 255;
    cpu.regs[chip8::V1] = 1;

    cpu.pc = chip8::PROGRAM_ENTRY;
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 0);
    assert_eq!(cpu.regs[chip8::VF], 1);
}

#[test]
fn test_opcode_8XY5() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 20;
    cpu.regs[chip8::V1] = 10;

    cpu.load_program(&[0x80, 0x15]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 20 - 10);
    assert_eq!(cpu.regs[chip8::VF], 0);

    cpu.regs[chip8::V0] = 0;
    cpu.regs[chip8::V1] = 1;

    cpu.pc = chip8::PROGRAM_ENTRY;
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 255);
    assert_eq!(cpu.regs[chip8::VF], 1);
}

#[test]
fn test_opcode_8XY6() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 0b00000001;

    cpu.load_program(&[0x80, 0x16]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 0);
    assert_eq!(cpu.regs[chip8::VF], 1);
}

#[test]
fn test_opcode_8XY7() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 5;
    cpu.regs[chip8::V1] = 10;

    cpu.load_program(&[0x80, 0x17]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 5);
    cpu.regs[chip8::V0] = 11;
    cpu.pc = chip8::PROGRAM_ENTRY;
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 255);
    assert_eq!(cpu.regs[chip8::VF], 1);
}

#[test]
fn test_opcode_8XYE() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 0b10000000;

    cpu.load_program(&[0x80, 0x1E]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 0);
    assert_eq!(cpu.regs[chip8::VF], 1);
}

#[test]
fn test_opcode_9XY0() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 0;
    cpu.regs[chip8::V1] = 0;

    cpu.load_program(&[0x90, 0x10]);
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 2);
    cpu.regs[chip8::V1] = 1;
    cpu.pc = chip8::PROGRAM_ENTRY;
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 4);
}

#[test]
fn test_opcode_ANNN() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.load_program(&[0xAF, 0xFF]);
    cpu.execute();

    assert_eq!(cpu.i, 0xFFF);
}

#[test]
fn test_opcode_BNNN() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 0xF;
    cpu.load_program(&[0xBF, 0xF0]);
    cpu.execute();

    assert_eq!(cpu.pc, 0xFFF);
}

#[test]
fn test_opcode_CNNN() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.load_program(&[0xC0, 0x10]);
    cpu.execute();

    assert!(cpu.regs[chip8::V0] <= 0x10);
}

#[test]
fn test_opcode_DXYN() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 0;
    cpu.regs[chip8::V1] = 0;
    cpu.i = 0xFFF;
    cpu.memory[cpu.i as usize] = 0b11000011;
    cpu.load_program(&[0xD0, 0x11]);
    cpu.execute();

    assert_eq!(gpu.borrow().read(0, 0), 255);
    assert_eq!(gpu.borrow().read(1, 0), 255);
    assert_eq!(gpu.borrow().read(2, 0), 0);
    assert_eq!(gpu.borrow().read(3, 0), 0);
    assert_eq!(gpu.borrow().read(4, 0), 0);
    assert_eq!(gpu.borrow().read(5, 0), 0);
    assert_eq!(gpu.borrow().read(6, 0), 255);
    assert_eq!(gpu.borrow().read(7, 0), 255);
}

#[test]
fn test_opcode_EX9E() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.load_program(&[0xE0, 0x9E]);
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 2);

    cpu.keyboard[0] = true;
    cpu.pc = chip8::PROGRAM_ENTRY;
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 4);
}

#[test]
fn test_opcode_EXA1() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.load_program(&[0xE0, 0xA1]);
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 4);

    cpu.keyboard[0] = true;
    cpu.pc = chip8::PROGRAM_ENTRY;
    cpu.execute();

    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 2);
}

#[test]
fn test_opcode_FX07() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.timers[chip8::DELAY] = 10;
    cpu.load_program(&[0xF0, 0x07]);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 10);
}

#[test]
fn test_opcode_FX0A() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.load_program(&[0xF0, 0x0A]);

    for i in 0..100 {
        cpu.execute();
        assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY);
    }

    cpu.press_key(1);
    cpu.execute();

    assert_eq!(cpu.regs[chip8::V0], 1);
    assert_eq!(cpu.pc, chip8::PROGRAM_ENTRY + 2);
}

#[test]
fn test_opcode_FX15() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 0xFF;
    cpu.load_program(&[0xF0, 0x15]);
    cpu.execute();

    assert_eq!(cpu.timers[chip8::DELAY], 0xFF);
}

#[test]
fn test_opcode_FX18() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 0xFF;
    cpu.load_program(&[0xF0, 0x18]);
    cpu.execute();

    assert_eq!(cpu.timers[chip8::SOUND], 0xFF);
}

#[test]
fn test_opcode_FX29() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    cpu.regs[chip8::V0] = 0xF;
    cpu.load_program(&[0xF0, 0x29]);
    cpu.execute();

    assert_eq!(cpu.i, 0xF * 5);
}

#[test]
fn test_opcode_FX33() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    // TODO
    assert!(false);
}

#[test]
fn test_opcode_FX55() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    for i in 0..(chip8::VF + 1) {
        cpu.regs[i] = i as u8;
    }

    cpu.load_program(&[0xFF, 0x55]);
    cpu.execute();

    for i in 0..(chip8::VF + 1) {
        assert_eq!(cpu.memory[i], i as u8);
    }
}

#[test]
fn test_opcode_FX65() {
    let gpu: chip8::GPUref = RefCell::new(Box::new(chip8::ArrayGPU::new()));
    let mut cpu = chip8::CPU::new(&gpu);

    for i in 0..(chip8::VF + 1) {
        cpu.regs[i] = i as u8;
    }

    cpu.i = 0xFFF;
    cpu.load_program(&[0xFF, 0x65]);
    cpu.execute();

    for i in 0..(chip8::VF + 1) {
        assert_eq!(cpu.regs[i], 0);
    }
}
