#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;

use rtic::app;
use rtic::cyccnt::U32Ext;
use rtt_target::{rprintln, rtt_init_print};

use crate::blinker::Blinker;
use f3::hal::gpio::GpioExt;
use f3::hal::rcc::RccExt;
use f3::led::Leds;

mod blinker;

#[app(device = f3::hal::stm32f30x, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        blinker: Blinker,
    }

    #[init(spawn = [blinky])]
    fn init(cx: init::Context) -> init::LateResources {
        rtt_init_print!();
        rprintln!("Init");

        let mut core = cx.core;
        core.DCB.enable_trace();
        core.DWT.enable_cycle_counter();

        let device = cx.device;
        let mut rcc = device.RCC.constrain();

        let led = Leds::new(device.GPIOE.split(&mut rcc.ahb));
        let blinker = Blinker::new(led);

        cx.spawn.blinky().ok();

        init::LateResources { blinker }
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
        rprintln!("Blink");

        let blinker = cx.resources.blinker;
        blinker.run();

        cx.schedule.blinky(cx.scheduled + 2_000_000.cycles()).ok();
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn CAN_RX1();
    }
};
