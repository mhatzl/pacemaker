#![no_main]
#![no_std]

use defmt_rtt as _; // global logger
use panic_probe as _;
use xmc4_hal as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    exit();
}

/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_SUCCESS);
    }
}
