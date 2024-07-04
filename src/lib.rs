#![no_main]
#![no_std]

use core::cell::Cell;
use critical_section::Mutex;
use defmt::println;
use defmt_rtt as _; // global logger
use mantra_rust_macros::req;
use panic_probe as _;
use xmc4_hal as _;

#[cfg(target_os = "none")]
#[defmt::panic_handler]
fn panic() -> ! {
    panic_probe::hard_fault()
}

#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_FAILURE);
    }
}

/// Information about the device and implant date.
#[req(store)]
pub struct Store {
    /// Device model.
    /// [req(store.manufacturer)]
    pub device_model: &'static str,
    /// Serial number of the device.
    /// [req(store.manufacturer)]
    pub serial_number: &'static str,
    /// Unix timestamp of the implant date.
    /// [req(store.implant_date)]
    pub lead_implant_date: u64,
    /// Lead impedance in Ohm.
    /// [req(store.lead)]
    pub lead_impedance: usize,
}

/// Stored device and implant date information.
#[req(store)]
pub const STORE: Store = Store {
    device_model: "mantra-pacemaker",
    serial_number: "123456",
    lead_implant_date: 1718791844,
    lead_impedance: 500,
};

#[req(mode.aoo)]
fn pulse_aoo(ms_since_last_atrial_pulse: usize, time: usize) -> bool {
    if ms_since_last_atrial_pulse >= LRL_IN_MS {
        println!("@{}ms Pacemaker pulse in atrial chamber.", time * 10);
        pulse_chamber(DEFAULT_PARAM.atrial);
        true
    } else {
        false
    }
}

#[req(mode.vvt)]
fn pulse_vvt(
    ventricular_sensed: bool,
    ms_since_last_ventricular_pulse: usize,
    time: usize,
) -> bool {
    if ms_since_last_ventricular_pulse >= LRL_IN_MS {
        println!("@{}ms Pacemaker pulse in ventricular chamber.", time * 10);
        pulse_chamber(DEFAULT_PARAM.ventricular);
        true
    } else if ventricular_sensed && ms_since_last_ventricular_pulse > DEFAULT_PARAM.vrp {
        println!(
            "@{}ms Supporting pacemaker pulse in ventricular chamber.",
            time * 10
        );
        pulse_chamber(DEFAULT_PARAM.ventricular);
        true
    } else {
        false
    }
}

#[req(pulse)]
fn pulse_chamber(pulse_param: PulseParam) {
    defmt::debug!(
        " => Pulse: amplitude='{}', width='{}'.",
        pulse_param.amplitude,
        pulse_param.width
    );
}

/// Available operating modes for the pacemaker
#[req(mode)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// No operation.
    /// [req(mode.off)]
    Off,
    /// Asynchronous atrial chamber pacing.
    /// [req(mode.aoo)]
    Aoo,
    /// Ventricular sensed pulse triggered ventricular chamber pacing.
    /// [req(mode.vvt)]
    Vvt,
}

impl defmt::Format for Mode {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Mode::Off => defmt::write!(fmt, "off"),
            Mode::Aoo => defmt::write!(fmt, "aoo"),
            Mode::Vvt => defmt::write!(fmt, "vvt"),
        }
    }
}

const DEFAULT_PARAM: Param = Param {
    lrl: 60,
    atrial: PulseParam {
        amplitude: 3.5,
        width: 0.4,
    },
    ventricular: PulseParam {
        amplitude: 3.5,
        width: 0.4,
    },
    vrp: 32,
};
const LRL_IN_MS: usize = (DEFAULT_PARAM.lrl * 100) / 60;

/// Pacemaker parameter.
#[req(param)]
pub struct Param {
    /// Lower rate limit [ppm]
    /// [req(param.lrl)]
    lrl: usize,
    /// Atrial pulse parameters.
    atrial: PulseParam,
    /// Ventricular pulse parameters.
    ventricular: PulseParam,
    /// Ventricular Refractory Period [ms].
    /// [req(param.vrp)]
    vrp: usize,
}

/// Pulse parameter for both atrial and ventricular chambers.
#[req(param.pulse_amplitude, param.pulse_width)]
pub struct PulseParam {
    /// Pulse amplitude [V].
    amplitude: f32,
    /// Pulse width [ms].
    width: f32,
}

/// --------------------------------------------------------------
///                         TESTS
/// --------------------------------------------------------------
#[cfg(test)]
#[defmt_test::tests]
mod test {
    use defmt::assert;
    use mantra_rust_macros::req;

    use crate::{pulse_aoo, pulse_vvt, simulate_heart, DEFAULT_PARAM, LRL_IN_MS};

    #[test]
    fn aoo_noheartbeat_pacemakerpulse() {
        let ms_since_last_pulse = LRL_IN_MS;

        let pulsed = pulse_aoo(ms_since_last_pulse, 0);

        assert!(
            pulsed,
            "Atrial pulse not set, but last pulse was too long ago."
        );
    }

    #[req(mode.vvt.test.sensed_beat_outside_vrp)]
    #[test]
    fn vvt_pulsesensed_pacemakerpulse() {
        let ventricular_sensed = true;
        let last_pulse = DEFAULT_PARAM.vrp + 1;

        let pulsed = pulse_vvt(ventricular_sensed, last_pulse, 0);

        assert!(
            pulsed,
            "No supporting ventricular pulse, even though a pulse was sensed outside VRP interval"
        );
    }

    #[req(mode.vvt.test.sensed_beat_inside_vrp)]
    #[test]
    fn vvt_pulsesensed_nopacemakerpulse() {
        let ventricular_sensed = true;
        let last_pulse = DEFAULT_PARAM.vrp;

        let pulsed = pulse_vvt(ventricular_sensed, last_pulse, 0);

        assert!(
            !pulsed,
            "Ventricular pulse, even though the sensed pulse was inside VRP interval"
        );
    }

    #[req(mode.off)]
    #[test]
    fn mode_off_simulated_heart_pulses_by_itself() {
        let (atrial_pulsed, ventricular_pulsed) =
            simulate_heart(&super::Mode::Off, LRL_IN_MS - 1, LRL_IN_MS - 1, 0);

        assert!(
            atrial_pulsed,
            "Simulated heart did not pulse atrial chamber by itself in mode 'off'."
        );
        assert!(
            ventricular_pulsed,
            "Simulated heart did not pulse ventricular chamber by itself in mode 'off'."
        );
    }
}

// ---------------------------- SIMULATION SPECIFIC CODE --------------

pub fn demo_loop(mode: &Mode) {
    let mut atrial_pulse_cnt = 0;
    let mut passed_ms = 0;
    let mut ms_since_last_atrial_pulse = 0;
    let mut ms_since_last_ventricular_pulse = 50;

    while atrial_pulse_cnt < 20 {
        let (mut atrial_pulsed, mut ventricular_pulsed) = simulate_heart(
            mode,
            ms_since_last_atrial_pulse,
            ms_since_last_ventricular_pulse,
            passed_ms,
        );

        if mode == &Mode::Aoo {
            atrial_pulsed |= pulse_aoo(ms_since_last_atrial_pulse, passed_ms);
        } else if mode == &Mode::Vvt {
            ventricular_pulsed |= pulse_vvt(
                ventricular_pulsed,
                ms_since_last_ventricular_pulse,
                passed_ms,
            );
        }

        if atrial_pulsed {
            atrial_pulse_cnt += 1;
            ms_since_last_atrial_pulse = 0;
        }
        if ventricular_pulsed {
            ms_since_last_ventricular_pulse = 0;
        }

        simulate_delay();
        ms_since_last_atrial_pulse += 1;
        ms_since_last_ventricular_pulse += 1;
        passed_ms += 1;
    }
}

fn simulate_heart(
    mode: &Mode,
    ms_since_last_atrial_pulse: usize,
    ms_since_last_ventricular_pulse: usize,
    time: usize,
) -> (bool, bool) {
    let mut atrial_pulsed = false;
    let mut ventricular_pulsed = false;

    // -1 to randomly pulse before pacemaker
    if ms_since_last_atrial_pulse == LRL_IN_MS - 1 {
        match mode {
            Mode::Off | Mode::Vvt => {
                println!("@{}ms Heart A.", time * 10);
                atrial_pulsed = true;
            }
            Mode::Aoo => {
                let rand_nr = next_rand_nr();

                if rand_nr >= u64::MAX / 2 {
                    println!("@{}ms Heart A.", time * 10);
                    atrial_pulsed = true;
                }
            }
        }
    }

    // -1 to randomly pulse before pacemaker
    if ms_since_last_ventricular_pulse == LRL_IN_MS - 1 {
        match mode {
            Mode::Off | Mode::Aoo => {
                println!("@{}ms Heart V.", time);
                ventricular_pulsed = true;
            }
            Mode::Vvt => {
                let rand_nr = next_rand_nr();

                if rand_nr >= u64::MAX / 2 {
                    println!("@{}ms Heart V.", time * 10);
                    ventricular_pulsed = true;
                }
            }
        }
    } else if mode == &Mode::Vvt && ms_since_last_ventricular_pulse > DEFAULT_PARAM.vrp {
        // randomly pulse before to trigger sensed ventricular pulse
        let rand_nr = next_rand_nr();

        if rand_nr < u64::MAX / 5000 {
            println!("@{}ms Heart V.", time * 10);
            ventricular_pulsed = true;
        }
    }

    (atrial_pulsed, ventricular_pulsed)
}

static RAND_SEED: Mutex<Cell<u64>> = Mutex::new(Cell::new(123456));

fn next_rand_nr() -> u64 {
    let mut state = critical_section::with(|cs| RAND_SEED.borrow(cs).get());

    if state >= PSEUDO_RANDOM_NR.len() as u64 {
        state = 0;
    }

    let rand_nr = PSEUDO_RANDOM_NR[state as usize];

    critical_section::with(|cs| RAND_SEED.borrow(cs).set(state + 1));

    rand_nr
}

/// waste processor time to simulate delay of approximately one millisecond on target
fn simulate_delay() {
    let mut _j = 0;
    for i in 0..5000 {
        if i % 7 == 0 {
            _j += 1;
        }

        if i % 253 == 0 {
            _j /= 2;
        }
    }
}

const PSEUDO_RANDOM_NR: [u64; 60] = [
    4082856971599685620,
    263328434869347407,
    7576667847654208308,
    12684192323921937922,
    16181197760313166074,
    4993000259016286523,
    7777360851611472270,
    12684858740337089069,
    13792758530809556226,
    9705484450158675180,
    2356434852560640881,
    12991886417691552848,
    14262153250304362608,
    9721139077091098879,
    5340259883797775994,
    1910912361441097101,
    551951547015110708,
    13388081104731861589,
    14561835204930253753,
    1145348537966955514,
    4635642057608043709,
    247279847734376319,
    14475454185974473037,
    5356791289287893950,
    5394722332071153439,
    2763708184518272260,
    4052692358703354570,
    662762972601170563,
    15525522150426148273,
    8713541385896213327,
    7744339156133814078,
    1179486564352551839,
    8186741526410188083,
    17813459038272261655,
    6703836692820609876,
    11887323209793110945,
    14743971615791178849,
    7105582857490582895,
    4947742274484715779,
    16548987461410137580,
    1906977537585733016,
    3491468108978074261,
    7646978571229541228,
    14368653334384094176,
    12561872433439601078,
    17084379079299463301,
    9671403567356947947,
    9780812539195315877,
    1911346566344532911,
    8369580830371596415,
    14706596080775791548,
    15189009629689132555,
    9125462606164200898,
    12481310361510826705,
    542518502924445949,
    14147427910115416641,
    12663770586276171547,
    10758265674907009008,
    17891922577438267520,
    14061335016435054781,
];
