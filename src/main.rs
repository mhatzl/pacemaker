#![no_main]
#![no_std]

use defmt::println;
use pacemaker::Mode;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mode = Mode::Vvt;
    println!("Demo start with mode: {}", mode);

    pacemaker::demo_loop(&mode);

    println!("Demo end.");

    exit();
}

/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
fn exit() -> ! {
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_SUCCESS);
    }
}
