target extended-remote :3333

set print asm-demangle on
set backtrace limit 32

load

monitor reset halt

break DefaultHandler
break HardFault
break rust_begin_unwind
break main

continue
