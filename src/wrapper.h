/// Pulse parameter for both atrial and ventricular chambers.
/// [req(param.pulse_amplitude, param.pulse_width)]
typedef struct PulseParam {
    /// Pulse amplitude [V].
    float amplitude;
    /// Pulse width [ms].
    float width;
};

/// Pacemaker parameter.
/// [req(param)]
typedef struct Param {
    /// Atrial pulse parameters.
    struct PulseParam atrial;
    /// Ventricular pulse parameters.
    struct PulseParam ventricular;
    /// Lower rate limit [ppm]
    /// [req(param.lrl)]
    unsigned int lrl;
    /// Ventricular Refractory Period [ms].
    /// [req(param.vrp)]
    unsigned int vrp
};