#![no_main]
#![no_std]

#[defmt_test::tests]
mod test {
    use defmt::assert;
    use mantra_rust_macros::req;

    use pacemaker::{param::DEFAULT_PARAM, pulse_aoo, pulse_vvt, simulate_heart, LRL_IN_MS};

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
            simulate_heart(&pacemaker::Mode::Off, LRL_IN_MS - 1, LRL_IN_MS - 1, 0);

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
