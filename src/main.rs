#![no_main]
#![no_std]

// use core::cell::Cell;
// use critical_section::Mutex;
// use defmt::println;
// use defmt_rtt as _; // global logger
// use mantra_rust_macros::req;
// use panic_probe as _;
// use xmc4_hal as _;

// #[cfg(target_os = "none")]
// #[defmt::panic_handler]
// fn panic() -> ! {
//     panic_probe::hard_fault()
// }

// #[cortex_m_rt::exception]
// unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
//     loop {
//         cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_FAILURE);
//     }
// }

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
