# About chip8rtic
chip8rtic is a [rust embedded](https://www.rust-lang.org/what/embedded) learning project. A video of the embedded emulator working can be found in this [tweet](https://twitter.com/i/status/1298543916007018496).

# Hardware
This emulator runs on a STM32F3DISCOVERY board with a STM32F303VCT6 chip and a 128x64 I2C display with the sh1106 chip.

# Hardware interface
The emulator uses the following GPIOs with a special purpose:

| GPIO | Purpose                        |
|:-----|:-------------------------------|
| PB6  | SCK signal for the IoC display |
| PB7  | SDA signal for the IoC display |
| PD8  | Button 1                       |
| PB9  | Button 2                       |
| PD10 | Button 3                       |
| PB11 | Button 4                       |
| PD12 | Button 5                       |
| PB13 | Button 6                       |
| PD14 | Button 7                       |
| PB15 | Button 8                       |

The different buttons are mapped to different CHIP-8 Keypad keys for each ROM. If you plan to execute for ROMs that are not currently mapped, you will need to provide the appropiate mappings.

# Running the software
Follow the basic guide provided in the [The Discovery book](https://docs.rust-embedded.org/discovery/) or [The Embedded Rust book](https://docs.rust-embedded.org/book/) to have a working environment for rust embedded.

## Installing the target
The needed target for this project is `thumbv7em-none-eabihf`, install it using the following rustup command:
```
rustup target install thumbv7em-none-eabihf
```

## Installing cargo-embed
The project is prepared to be flashed and executed using cargo-embed. Install it as follows:
```
cargo install cargo-embed
```

## Run the emulator
Build, flash and run the project with the following command:
```
cargo embed --release
```

# Future work
* Make a small GUI to select ROM.
* Add a reset button.
* Optimize the system for power consumption, as it should work with batteries.
* Design a small PCB for the system and make it open hardware.
* Design a case for it and release the STL files.

# ROM Credits
ROMs are provided in the `games` folder for testing purposes. Credits of ROMs go to individual creators:

* 15PUZZLE - Roger Ivie
* BLINKY - Hans Christian Egeberg
* BLITZ - David Winter
* BRIX - Andreas Gustafsson
* CONNECT4 - David Winter
* GUESS - David Winter
* HIDDEN - David Winter
* INVADERS - David Winter
* KALEID - Joseph Weisbecker
* MAZE - ?
* MERLIN - David Winter
* MISSILE - David Winter
* PONG - Paul Vervalin
* PONG2 - David Winter
* PUZZLE - ?
* SYZYGY - Roy Trevino
* TANK - ?
* TETRIS - Fran Dachille
* TICTAC - David Winter
* UFO - Lutz V
* VBRIX - Paul Robson
* VERS - JMN
* WIPEOFF - Joseph Weisbecker