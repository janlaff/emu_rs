extern crate piston_window;

use piston_window::*;

mod emu;
use emu::core::GPU;

use crate::emu::arch::chip8::Keyboard;
use crate::emu::core::{Clock, FrameBuffer};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Deref;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Please specify a rom to load");
        return;
    }

    let mut rom = Vec::new();
    let mut f = File::open(args[1].clone()).unwrap();
    f.read_to_end(&mut rom).unwrap();

    let mut threads = vec![];

    let (cpu_tx, cpu_rx) = channel();
    let (epx_tx, epx_rx) = channel();

    let frame_buf_ctx = emu::core::FrameBufferContext::new(vec![
        FrameBuffer::new(64, 32, 0u8),
        FrameBuffer::new(128, 64, 0u8),
    ]);

    let gpu = Arc::new(Mutex::new(emu::core::EpxGPU::new()));

    let keyboard = Arc::new(Mutex::new(Keyboard::new()));

    let mut cpu = emu::arch::chip8::CPU::new(frame_buf_ctx.get_buffer(0), keyboard.clone());
    cpu.load_program(&rom[..]);

    let mut settings =
        piston_window::WindowSettings::new("Chip8 Emulator", (64.0 * 10.0, 32.0 * 10.0));
    settings.set_vsync(true);
    let mut window: piston_window::PistonWindow = settings.build().unwrap();
    let mut texture_ctx = window.create_texture_context();
    let mut texture = None;

    let gpu = Arc::new(Mutex::new(emu::core::EpxGPU::new()));
    let cpu_active = Arc::new(Mutex::new(true));
    let gpu_active = Arc::new(Mutex::new(true));

    let local_cpu_active = cpu_active.clone();
    let local_keyboard = keyboard.clone();
    let local_cpu_tx = cpu_tx.clone();
    threads.push(thread::spawn(move || {
        let mut clock = Clock::new(540); // Hz
        let mut timer_clock = Clock::new(60); // Hz

        while *local_cpu_active.lock().unwrap() {
            if clock.tick(true) {
                cpu.execute();

                if timer_clock.tick(false) {
                    cpu.tick();
                }

                if cpu.frame_buf.lock().unwrap().handle_draw() {
                    local_cpu_tx.send(());
                }
            }
        }
    }));

    let local_gpu_active = gpu_active.clone();
    let local_cpu_buf = frame_buf_ctx.get_buffer(0);
    let local_epx_buf = frame_buf_ctx.get_buffer(1);
    let local_gpu = gpu.clone();
    let local_epx_tx = epx_tx.clone();
    threads.push(thread::spawn(move || {
        while *local_gpu_active.lock().unwrap() {
            if let Ok(()) = cpu_rx.recv() {
                let cpu_buf = local_cpu_buf.lock().unwrap();
                let mut epx_buf = local_epx_buf.lock().unwrap();
                local_gpu
                    .lock()
                    .unwrap()
                    .process(cpu_buf.borrow(), epx_buf.borrow_mut());

                if epx_buf.handle_draw() {
                    local_epx_tx.send(());
                }
            }
        }
    }));

    let local_epx_buf = frame_buf_ctx.get_buffer(1);
    while let Some(e) = window.next() {
        if let Ok(()) = epx_rx.try_recv() {
            let buf = local_epx_buf.lock().unwrap();
            let tex_settings =
                piston_window::TextureSettings::new().filter(piston_window::Filter::Nearest);
            texture = Some(
                piston_window::Texture::from_memory_alpha(
                    &mut texture_ctx,
                    &buf.frame(),
                    buf.width(),
                    buf.height(),
                    &tex_settings,
                )
                .unwrap(),
            );
        }

        //println!("{:?}", e);

        let state_to_bool = |state: ButtonState| match (state) {
            ButtonState::Press => true,
            ButtonState::Release => false,
        };
        match &e {
            piston_window::Event::Input(piston_window::Input::Button(args), _) => {
                let res = match args.button {
                    piston_window::Button::Keyboard(val) => match (val) {
                        Key::D1 => Some(0),
                        Key::D2 => Some(1),
                        Key::D3 => Some(2),
                        Key::D4 => Some(3),
                        Key::Q => Some(4),
                        Key::W => Some(5),
                        Key::E => Some(6),
                        Key::R => Some(7),
                        Key::A => Some(8),
                        Key::S => Some(9),
                        Key::D => Some(10),
                        Key::F => Some(11),
                        Key::Y => Some(12),
                        Key::X => Some(13),
                        Key::C => Some(14),
                        Key::V => Some(15),
                        Key::G => {
                            if args.state == ButtonState::Press {
                                let mut g = gpu.lock().unwrap();
                                g.enabled = !g.enabled;
                            }
                            None
                        }
                        _ => None,
                    },
                    _ => None,
                };

                match (res, args.state) {
                    (Some(idx), ButtonState::Press) => keyboard.lock().unwrap().press_key(idx),
                    (Some(idx), ButtonState::Release) => keyboard.lock().unwrap().release_key(idx),
                    _ => {}
                }
            }
            _ => {}
        }

        window.draw_2d(&e, |c, g, _| {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], g);
            match &texture {
                Some(tex) => piston_window::image(tex, c.transform.zoom(5.0), g),
                None => {}
            }
        });
    }

    *cpu_active.lock().unwrap() = false;
    *gpu_active.lock().unwrap() = false;

    // Send signal to unblock gpu thread
    cpu_tx.send(());

    for t in threads {
        t.join().unwrap();
    }
}

#[cfg(test)]
mod test;
