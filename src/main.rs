#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;

use rtic::app;
use rtic::cyccnt::{Duration, U32Ext};
use rtt_target::{rprintln, rtt_init_print};

use stm32f3xx_hal::flash::FlashExt;
use stm32f3xx_hal::gpio::{gpiod, GpioExt};
use stm32f3xx_hal::i2c::I2c;
use stm32f3xx_hal::rcc::RccExt;
use stm32f3xx_hal::time::{self, Hertz, MegaHertz};

use crate::keypad::Buttons;
use chip8vm::PROGRAM_SIZE;

mod keypad;
mod random;

mod screen;
static FREQUENCY: MegaHertz = time::MegaHertz(36);

//const ROM_MAZE: &[u8; 34] = include_bytes!("../games/MAZE");
//const ROM_PONG: &[u8; 246] = include_bytes!("../games/PONG");
//const ROM_BRIX: &[u8; 280] = include_bytes!("../games/BRIX");
//const ROM_VBRIX: &[u8; 507] = include_bytes!("../games/VBRIX");
//const ROM_TETRIS: &[u8; 494] = include_bytes!("../games/TETRIS");
const ROM: &[u8; 494] = include_bytes!("../games/TETRIS");

#[app(device = stm32f3xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        chip8: chip8vm::chip::Chip,
        random: random::Random,
        screen: screen::Screen,
        keypad: keypad::Keypad,
        buttons: Buttons,
    }

    #[init(spawn = [cpu, timers, display, input])]
    fn init(cx: init::Context) -> init::LateResources {
        rtt_init_print!();
        rprintln!("Init Start");

        //
        // Acquire and configure STM resources
        //
        let mut core = cx.core;
        core.DCB.enable_trace();
        core.DWT.enable_cycle_counter();

        let device: stm32f3xx_hal::stm32::Peripherals = cx.device;
        let mut rcc = device.RCC.constrain();
        let mut flash = device.FLASH.constrain();
        let clocks = rcc.cfgr.sysclk(FREQUENCY).freeze(&mut flash.acr);

        //
        // Enable GPIOs
        //
        let mut input: gpiod::Parts = device.GPIOD.split(&mut rcc.ahb);

        let button1 = input
            .pd8
            .into_pull_up_input(&mut input.moder, &mut input.pupdr);

        let button2 = input
            .pd9
            .into_pull_up_input(&mut input.moder, &mut input.pupdr);

        let button3 = input
            .pd10
            .into_pull_up_input(&mut input.moder, &mut input.pupdr);

        let button4 = input
            .pd11
            .into_pull_up_input(&mut input.moder, &mut input.pupdr);

        let button5 = input
            .pd12
            .into_pull_up_input(&mut input.moder, &mut input.pupdr);

        let button6 = input
            .pd13
            .into_pull_up_input(&mut input.moder, &mut input.pupdr);

        let button7 = input
            .pd14
            .into_pull_up_input(&mut input.moder, &mut input.pupdr);

        let button8 = input
            .pd15
            .into_pull_up_input(&mut input.moder, &mut input.pupdr);

        let buttons = Buttons {
            button1,
            button2,
            button3,
            button4,
            button5,
            button6,
            button7,
            button8,
        };

        //
        // Get I2C
        //
        let mut gpiob = device.GPIOB.split(&mut rcc.ahb);
        let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
        let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
        let i2c = I2c::i2c1(
            device.I2C1,
            (scl, sda),
            time::MegaHertz(1),
            clocks,
            &mut rcc.apb1,
        );

        //
        // Create chip8 resources
        //
        let mut chip8 = chip8vm::chip::Chip::default();
        let random = random::Random::new();
        let keypad = keypad::Keypad::new();
        let mut screen = screen::Screen::new(i2c);
        screen.init();

        //
        // Load program
        //
        let mut program = [0; PROGRAM_SIZE];
        program[0..ROM.len()].copy_from_slice(ROM);
        chip8.load_program(program);

        cx.spawn.cpu().ok();
        cx.spawn.timers().ok();
        cx.spawn.display().ok();
        cx.spawn.input().ok();

        rprintln!("Init End");

        //
        // Init RTIC resources
        //
        init::LateResources {
            chip8,
            random,
            screen,
            keypad,
            buttons,
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("Idle");

        loop {
            continue;
        }
    }

    #[task(schedule = [cpu], resources = [chip8, random, screen, keypad])]
    fn cpu(cx: cpu::Context) {
        static TASK_FREQUENCY: Hertz = Hertz(600);

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
        static TASK_FREQUENCY: Hertz = Hertz(60);

        let screen = cx.resources.screen;

        screen.display();

        cx.schedule
            .display(cx.scheduled + plan_task(TASK_FREQUENCY))
            .ok();
    }

    #[task(schedule = [input], resources = [keypad, buttons])]
    fn input(cx: input::Context) {
        static TASK_FREQUENCY: Hertz = Hertz(15);

        let keypad = cx.resources.keypad;
        let buttons = cx.resources.buttons;

        keypad.check(
            &buttons.button1,
            &buttons.button2,
            &buttons.button3,
            &buttons.button4,
            &buttons.button5,
            &buttons.button6,
            &buttons.button7,
            &buttons.button8,
        );

        cx.schedule
            .input(cx.scheduled + plan_task(TASK_FREQUENCY))
            .ok();
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks. We need need one free interrupt per software task priority level.
    extern "C" {
        fn CAN_RX1();
    //fn CAN_TX1();
    //fn CAN_RX2();
    //fn CAN_TX2();
    }
};

#[inline]
fn plan_task(frequency: Hertz) -> Duration {
    (((1.0 / frequency.0 as f32) * (FREQUENCY.0 * 1_000_000) as f32) as u32).cycles()
}
