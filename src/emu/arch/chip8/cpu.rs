extern crate rand;

use rand::prelude::*;

use super::super::super::core::FrameBuffer;
use super::font::*;
use super::opcode::*;

// General constants
pub const PROGRAM_ENTRY: u16 = 0x200;

// Register Identifiers
pub const V0: usize = 0;
pub const V1: usize = 1;
pub const V2: usize = 2;
pub const V3: usize = 3;
pub const V4: usize = 4;
pub const V5: usize = 5;
pub const V6: usize = 6;
pub const V7: usize = 7;
pub const V8: usize = 8;
pub const V9: usize = 9;
pub const VA: usize = 10;
pub const VB: usize = 11;
pub const VC: usize = 12;
pub const VD: usize = 13;
pub const VE: usize = 14;
pub const VF: usize = 15;

// Timer Register Identifiers
pub const SOUND: usize = 0;
pub const DELAY: usize = 1;

pub struct CPU {
    // Internal registers
    pub regs: [u8; 16],
    pub sp: u16,
    pub pc: u16,
    pub i: u16,
    pub timers: [u8; 2],
    // Internal memory
    pub frame_buf: FrameBuffer<u8>,
    pub memory: [u8; 0xFFFF],
    pub stack: [u16; 64],
    // User input
    pub keyboard: [bool; 16],
    // Emulator specific
    pub draw_flag: bool,
    pub wait_for_key: bool,
    pub key_received: bool,
    pub key: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: [0; 16],
            sp: 0,
            pc: PROGRAM_ENTRY,
            i: 0,
            timers: [0; 2],
            frame_buf: FrameBuffer::new(64, 32, 0u8),
            memory: [0; 0xFFFF],
            stack: [0; 64],
            keyboard: [false; 16],
            draw_flag: false,
            wait_for_key: false,
            key_received: false,
            key: 0,
        }
    }

    pub fn load_program(&mut self, binary: &[u8]) {
        // Load font map
        for i in 0..16 {
            self.memory[i] = FONT_CHARMAP[i];
        }
        // Load program
        let mut address = PROGRAM_ENTRY;
        for byte in binary {
            self.memory[address as usize] = *byte;
            address += 1;
        }
    }

    fn read_opcode(&self) -> Opcode {
        let high = self.memory[self.pc as usize] as u16;
        let low = self.memory[(self.pc + 1) as usize] as u16;

        Opcode {
            value: (high << 8) | low,
        }
    }

    pub fn execute(&mut self) {
        let opcode = self.read_opcode();

        let vx = opcode.x() as usize;
        let vy = opcode.y() as usize;
        let nn = opcode.nn() as u8;

        println!("({:04X}) Executing opcode: {:04X}", self.pc, opcode.value);

        let mut rng = rand::thread_rng();

        match opcode.first() {
            0x0 => match opcode.nn() {
                0xE0 => {
                    // Clear display
                    self.frame_buf.clear(0);
                    self.pc += 2;
                }
                0xEE => {
                    // Return from subroutine
                    self.sp -= 1;
                    let return_address = self.stack[self.sp as usize];
                    self.pc = return_address;
                }
                val => unimplemented!("Unknown opcode ({:04X})", val),
            },
            0x1 => {
                // Goto NNN
                self.pc = opcode.nnn();
            }
            0x2 => {
                // Call NNN
                self.stack[self.sp as usize] = self.pc + 2;
                self.sp += 1;
                self.pc = opcode.nnn();
            }
            0x3 => {
                // Skip next instruction if VX == NN
                if self.regs[vx] == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x4 => {
                // Skip next instruction if VX != NN
                if self.regs[vx] != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x5 => {
                // Skip next instruction if VX == VY
                if self.regs[vx] == self.regs[vy] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x6 => {
                // VX = NN
                self.regs[vx] = nn;
                self.pc += 2;
            }
            0x7 => {
                // VX += NN
                self.regs[vx] = match self.regs[vx].overflowing_add(nn) {
                    (res, _) => res,
                };

                self.pc += 2;
            }
            0x8 => match opcode.last() {
                0x0 => {
                    // VX = VY
                    self.regs[vx] = self.regs[vy];
                    self.pc += 2;
                }
                0x1 => {
                    // VX |= VY
                    self.regs[vx] |= self.regs[vy];
                    self.pc += 2;
                }
                0x2 => {
                    // VX &= VY
                    self.regs[vx] &= self.regs[vy];
                    self.pc += 2;
                }
                0x3 => {
                    // VX ^= VY
                    self.regs[vx] ^= self.regs[vy];
                    self.pc += 2;
                }
                0x4 => {
                    // VX += VY
                    match self.regs[vx].overflowing_add(self.regs[vy]) {
                        (result, overflow) => {
                            self.regs[vx] = result;
                            self.regs[VF] = overflow as u8;
                        }
                    }
                    self.pc += 2;
                }
                0x5 => {
                    // VX -= VY
                    match self.regs[vx].overflowing_sub(self.regs[vy]) {
                        (result, overflow) => {
                            self.regs[vx] = result;
                            self.regs[VF] = overflow as u8;
                        }
                    }
                    self.pc += 2;
                }
                0x6 => {
                    // VX >>= 1
                    self.regs[VF] = self.regs[vx] & 0x1;
                    self.regs[vx] >>= 1;
                    self.pc += 2;
                }
                0x7 => {
                    // VX = VY - VX
                    match self.regs[vy].overflowing_sub(self.regs[vx]) {
                        (result, overflow) => {
                            self.regs[vx] = result;
                            self.regs[VF] = overflow as u8;
                        }
                    }
                    self.pc += 2;
                }
                0xE => {
                    // VX <<= 1
                    self.regs[VF] = self.regs[vx] >> 7 & 0x1;
                    self.regs[vx] <<= 1;
                    self.pc += 2;
                }
                _ => unimplemented!("Unknown opcode: ({:04X})", opcode.value),
            },
            0x9 => {
                // Skip next instruction if VX != VY
                if self.regs[vx] != self.regs[vy] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xA => {
                // I = NNN
                self.i = opcode.nnn();
                self.pc += 2;
            }
            0xB => {
                // Goto NNN + V0
                self.pc = opcode.nnn() + self.regs[V0] as u16
            }
            0xC => {
                // VX = rand() & NN
                let rand: u8 = rng.gen();
                self.regs[vx] = rand & nn;
                self.pc += 2;
            }
            0xD => {
                // Draw sprite
                let flipped = self.draw_sprite(self.regs[vx], self.regs[vy], opcode.last() as u8);
                self.draw_flag = true;
                self.regs[VF] = flipped as u8;
                self.pc += 2;
            }
            0xE => match opcode.nn() {
                0x9E => {
                    // Skip next instruction if key[VX] is pressed
                    if self.keyboard[self.regs[vx] as usize] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    // Skip next instruction if key[VX] is not pressed
                    if !self.keyboard[self.regs[vx] as usize] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                _ => unimplemented!("Unknown opcode: ({:04X})", opcode.value),
            },
            0xF => match opcode.nn() {
                0x07 => {
                    // VX = delay
                    self.regs[vx] = self.timers[DELAY] as u8;
                    self.pc += 2;
                }
                0x0A => {
                    // Wait for keypress
                    if self.wait_for_key && self.key_received {
                        self.wait_for_key = false;
                        self.key_received = false;

                        self.regs[vx] = self.key;
                        self.pc += 2;
                    } else {
                        self.wait_for_key = true;
                    }
                }
                0x15 => {
                    // delay = VX
                    self.timers[DELAY] = self.regs[vx];
                    self.pc += 2;
                }
                0x18 => {
                    // sound = VX
                    self.timers[SOUND] = self.regs[vx];
                    self.pc += 2;
                }
                0x1E => {
                    // I += VX
                    let res = self.i + self.regs[vx] as u16;
                    let overflow = (res > 0xFFF) as u8;
                    self.i = res;
                    self.regs[VF] = overflow;
                    self.pc += 2;
                }
                0x29 => {
                    // I = sprite_addr[VX]
                    self.i = (self.regs[vx] * 5) as u16;
                    self.pc += 2;
                }
                0x33 => {
                    // Store BCD(VX) at I
                    let mut offset = self.i;
                    let mut val = self.regs[vx];

                    if val % 100 > 0 {
                        self.memory[offset as usize] = val / 100;
                        val = val - (val / 100) * 100;
                        offset += 1;
                    }
                    if val % 10 > 0 {
                        self.memory[offset as usize] = val / 10;
                        val = val - (val / 10) * 10;
                        offset += 1;
                    }
                    self.memory[offset as usize] = val;
                    self.pc += 2;
                }
                0x55 => {
                    // Dump V0-VX at I
                    let mut address = self.i as usize;

                    for i in 0..(vx + 1) {
                        self.memory[address] = self.regs[i];
                        address += 1;
                    }

                    self.pc += 2;
                }
                0x65 => {
                    // Read V0-VX from I
                    let mut address = self.i as usize;

                    for i in 0..(vx + 1) {
                        self.regs[i] = self.memory[address];
                        address += 1;
                    }

                    self.pc += 2;
                }
                _ => unimplemented!("Unknown opcode: ({:04X})", opcode.value),
            },
            _ => unimplemented!("Unknown opcode: ({:04X})", opcode.value),
        }

        /*if self.draw_flag {
            self.draw_flag = false;
            self.debug_draw();
        }*/
    }

    pub fn tick(&mut self) {
        if self.timers[DELAY] > 0 {
            self.timers[DELAY] -= 1;
        }

        if self.timers[SOUND] > 0 {
            self.timers[SOUND] -= 1;
        }
    }

    pub fn press_key(&mut self, key: u8) {
        self.keyboard[key as usize] = true;

        if self.wait_for_key {
            self.key_received = true;
            self.key = key;
        }
    }

    pub fn release_key(&mut self, key: u8) {
        self.keyboard[key as usize] = false;
    }

    fn draw_sprite(&mut self, x: u8, y: u8, height: u8) -> bool {
        let buf = [0u8; 8];
        let mut ret = false;

        for i in 0..height {
            // u8 line containing 8 pixels bit encoded
            let line = self.memory[(self.i + i as u16) as usize];

            for j in 0..8 {
                let mut pixel = 0;
                let mask = (1 << (7 - j));

                if line & mask != 0 {
                    pixel = 255;
                }

                let c_x = match x.overflowing_add(j) {
                    (res, _) => res as u32,
                };

                let c_y = match y.overflowing_add(i) {
                    (res, _) => res as u32,
                };

                if pixel == 0xFF {
                    if self.frame_buf.read(c_x, c_y) == 0x00 {
                        self.frame_buf.write(c_x, c_y, 0xFF);
                        ret = true;
                    } else {
                        self.frame_buf.write(c_x, c_y, 0x00);
                    }
                }
            }
        }

        ret
    }
}
