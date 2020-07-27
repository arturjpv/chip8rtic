#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;

use rtic::app;
use rtic::cyccnt::{Duration, U32Ext};
use rtt_target::{rprintln, rtt_init_print};

use crate::blinker::Blinker;

use f3::hal::gpio::GpioExt;
use f3::hal::prelude::*;
use f3::hal::rcc::RccExt;
use f3::hal::time;
use f3::hal::time::{Hertz, MegaHertz};
use f3::led::Leds;

use chip8vm::PROGRAM_SIZE;
mod blinker;
mod keypad;
mod random;

mod screen;
static FREQUENCY: MegaHertz = time::MegaHertz(36);

//const ROOM_MAZE: &[u8; 34] = include_bytes!("../games/MAZE");
const ROOM_PONG: &[u8; 246] = include_bytes!("../games/PONG");

#[app(device = f3::hal::stm32f30x, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        blinker: Blinker,
        chip8: chip8vm::chip::Chip,
        random: random::Random,
        screen: screen::Screen,
        keypad: keypad::Keypad,
    }

    #[init(spawn = [blinky, cpu, timers, display, input])]
    fn init(cx: init::Context) -> init::LateResources {
        rtt_init_print!();
        rprintln!("Init");

        let mut core = cx.core;
        core.DCB.enable_trace();
        core.DWT.enable_cycle_counter();

        let device = cx.device;
        let mut rcc = device.RCC.constrain();
        let mut flash = device.FLASH.constrain();
        rcc.cfgr.sysclk(FREQUENCY).freeze(&mut flash.acr);

        let led = Leds::new(device.GPIOE.split(&mut rcc.ahb));
        let blinker = Blinker::new(led);

        let mut chip8 = chip8vm::chip::Chip::default();
        let random = random::Random::new();
        let screen = screen::Screen::new();
        let keypad = keypad::Keypad::new();

        //
        // Load program
        //
        let mut program = [0; PROGRAM_SIZE];
        program[0..ROOM_PONG.len()].copy_from_slice(ROOM_PONG);
        chip8.load_program(program);

        cx.spawn.blinky().ok();
        cx.spawn.cpu().ok();
        cx.spawn.timers().ok();
        cx.spawn.display().ok();
        cx.spawn.input().ok();

        init::LateResources {
            blinker,
            chip8,
            random,
            screen,
            keypad,
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("Idle");

        loop {
            continue;
        }
    }

    #[task(schedule = [blinky], resources = [blinker])]
    fn blinky(cx: blinky::Context) {
        static TASK_FREQUENCY: Hertz = Hertz(1);

        rprintln!("TICK");

        let blinker = cx.resources.blinker;
        blinker.run();

        cx.schedule
            .blinky(cx.scheduled + plan_task(TASK_FREQUENCY))
            .ok();
    }

    #[task(schedule = [cpu], resources = [chip8, random, screen, keypad])]
    fn cpu(cx: cpu::Context) {
        static TASK_FREQUENCY: Hertz = Hertz(300);

        let chip8 = cx.resources.chip8;
        let random = cx.resources.random;
        let screen = cx.resources.screen;
        let keypad = cx.resources.keypad;

        chip8.tick(random, screen, keypad);

        cx.schedule
            .cpu(cx.scheduled + plan_task(TASK_FREQUENCY))
            .ok();
    }

    #[task(schedule = [timers], resources = [chip8])]
    fn timers(cx: timers::Context) {
        static TASK_FREQUENCY: Hertz = Hertz(60);

        let chip8 = cx.resources.chip8;

        chip8.tick_timers();

        cx.schedule
            .timers(cx.scheduled + plan_task(TASK_FREQUENCY))
            .ok();
    }

    #[task(schedule = [display], resources = [screen])]
    fn display(cx: display::Context) {
        static TASK_FREQUENCY: Hertz = Hertz(15);

        let screen = cx.resources.screen;

        screen.display();

        cx.schedule
            .display(cx.scheduled + plan_task(TASK_FREQUENCY))
            .ok();
    }

    #[task(schedule = [input], resources = [keypad])]
    fn input(cx: input::Context) {
        static TASK_FREQUENCY: Hertz = Hertz(15);

        let keypad = cx.resources.keypad;

        keypad.check();

        cx.schedule
            .input(cx.scheduled + plan_task(TASK_FREQUENCY))
            .ok();
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks. We need need one free interrupt per software task priority level.
    extern "C" {
        fn CAN_RX1();
    }
};

#[inline]
fn plan_task(frequency: Hertz) -> Duration {
    (((1.0 / frequency.0 as f32) * (FREQUENCY.0 * 1_000_000) as f32) as u32).cycles()
}
