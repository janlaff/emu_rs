extern crate piston_window;

use piston_window::*;

mod emu;
use emu::core::GPU;

use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Write};

pub const ZOOM_FACTOR: f64 = 10.0;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Please specify a rom to load");
        return;
    }

    let mut rom = Vec::new();
    let mut f = File::open(args[1].clone()).unwrap();
    f.read_to_end(&mut rom);

    let mut cpu = emu::arch::chip8::CPU::new();

    cpu.load_program(&rom[..]);

    let mut last_pc = 0;
    let mut last_iteration = std::time::Instant::now();
    let mut last_tick = std::time::Instant::now();
    let mut last_gpu_toggle = std::time::Instant::now();

    let mut settings = WindowSettings::new("Chip8 Emulator", (64.0 * 10.0, 32.0 * 10.0));

    settings.set_vsync(false);

    let mut window: PistonWindow = settings.build().unwrap();

    let mut texture_ctx = window.create_texture_context();
    let mut texture = None;

    let mut gpu = emu::core::EpxGPU::new();
    let mut output_buf =
        emu::core::FrameBuffer::new(cpu.frame_buf.width() * 2, cpu.frame_buf.height() * 2, 0);

    'main: while let Some(e) = window.next() {
        let diff = last_iteration.elapsed();
        if diff.as_millis() < 1000 / 540 {
            std::thread::sleep(std::time::Duration::from_millis(1000 / 540) - diff);
        }
        last_iteration = std::time::Instant::now();

        let tick_diff = last_tick.elapsed();
        if tick_diff.as_millis() >= 1000 / 60 {
            last_tick = std::time::Instant::now();
            cpu.tick();
        }

        if cpu.pc == last_pc && (cpu.memory[cpu.pc as usize] >> 4) == 0x1 {
            eprintln!("Endless loop detected!");
        //break;
        } else {
            last_pc = cpu.pc;
        }

        cpu.execute();

        if cpu.draw_flag {
            cpu.draw_flag = false;

            gpu.process(&cpu.frame_buf, &mut output_buf);

            let tex_settings = TextureSettings::new().filter(Filter::Nearest);
            texture = Some(
                Texture::from_memory_alpha(
                    &mut texture_ctx,
                    &output_buf.frame(),
                    output_buf.width(),
                    output_buf.height(),
                    &tex_settings,
                )
                .unwrap(),
            );
        }

        if cpu.wait_for_key {
            //cpu.press_key(0);
        }

        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            match &texture {
                Some(tex) => image(tex, c.transform.zoom(5.0), g),
                None => {}
            }
        });
    }
}

#[cfg(test)]
mod test;
