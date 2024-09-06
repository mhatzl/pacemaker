//! [req(param)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const DEFAULT_PARAM: Param = Param {
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
