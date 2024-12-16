
#![no_std]
#![no_main]

use riscv_pac::{Gpio, Peripherals};
use riscv_rt::entry;
use panic_halt as _; // Panic handler
use riscv::asm::nop;

extern crate panic_halt;

#[entry]
fn main() ->!{
    let p: Gpio = unsafe{riscv_pac::Peripherals::steal().gpio};

    p.gpio_direction().write(|w| unsafe {w.bits(0 << 24)});

    p.gpio_set().write(|w| unsafe{w.bits(1 << 24)});
    loop{
        p.gpio_clear().write(|w| unsafe{w.bits(1 << 24)});
        delay(1_000_000); // Delay
        p.gpio_set().write(|w| unsafe{w.bits(1 << 24)});
        delay(1_000_000); // Delay
    }

}

// Simple delay function
fn delay(cycles: u64) {
    for _ in 0..cycles {
        nop();
    }
}

#[unsafe(no_mangle)]
fn InstructionMisaligned() {
    loop {}
}

#[unsafe(no_mangle)]
fn IllegalInstruction() {
    loop {}
}
