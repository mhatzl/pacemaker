Contains requirements for a basic pacemaker.
Requirements are taken and simplified from the [pacemaker system specification](https://greg4cr.github.io/courses/fall17csce740/Documents/PACEMAKER.pdf) from Boston Scientific.

# `lead`(manual): Lead Support

- The Atrial Bipolar Pace/Sensing lead system type shall be supported
- The Ventricular Bipolar Pace/Sensing lead system type shall be supported
- The system shall operate normally with atrial pace/sense leads between 100 and 2500 ohms
- The system shall operate normally with the ventricular pace/sense leads between 100 and 2500 ohms

**Note:** All pacing amplitudes and pulse widths are specified using a 750 ohm load.

# `pulse`: Pacing Pulse

The device shall output pulses with programmable voltages and widths (atrial
and ventricular) which provide electrical stimulation to the heart for pacing.

# `mode`: Bradycardia Operating Modes

The following bradycardia operating modes shall be supported: Off, AOO, VVT

| Category | Chambers Paced                      | Chambers Sensed                     | Response to Sensing     |
| -------- | ----------------------------------- | ----------------------------------- | ----------------------- |
| Letters  | O = None, A = Atrium, V = Ventricle | O = None, A = Atrium, V = Ventricle | O = None, T = Triggered |

## `mode.off`: Off Mode

No pacing is done.

## `mode.aoo`: AOO Mode

Pacing without sensing is asynchronous pacing. During asynchronous pacing,
paces shall be delivered to the atrial chamber without regard to senses.

## `mode.vvt`: VVT Mode

During triggered pacing, a sense in the ventricular chamber shall trigger an immediate pace in the ventricular chamber.

### `mode.vvt.test.sensed_beat_outside_vrp`: Ensure supporting pulse on sensed beat outside VRP interval

Ensure the pacemaker pulses the ventricular chamber if

- pulse is sensed in the ventricular chamber
- last pulse outside VRP interval

### `mode.vvt.test.sensed_beat_inside_vrp`: Ensure no pulse on sensed beat inside VRP interval

Ensure the pacemaker does not pulse the ventricular chamber if

- pulse is sensed in the ventricular chamber
- last pulse inside VRP interval

# `store`(manual): Implant Data

The device shall be capable of storing the information listed in the subsections in device memory.

## `store.manufacturer`: Manufacturer Data

The device shall be capable of storing the device model and serial number.

## `store.implant_date`: Implant Date

The device shall be capable of storing the lead implant date.

## `store.lead`: Pacing Lead Data

The device shall be capable of storing the pacing lead impedance.

# `param`(manual): Parameters

Programmable parameters are provided for controlling the delivery of
patient-tailored, bradycardia therapy. These parameters are described in the
following subsections; which parameters are meaningful with which pacing mode
are listed in the following table.

| Parameter               | VVT | AOO |
| ----------------------- | --- | --- |
| Lower Rate Limit        |  X  |  X  |
| Atrial Amplitude        |     |  X  |
| Ventricular Amplitude   |  X  |     |
| Atrial Pulse Width      |     |  X  |
| Ventricular Pulse Width |  X  |     |
| VRP                     |  X  |     |

## `param.lrl`: Lower Rate Limit (LRL)

The Lower Rate Limit (LRL) is the number of generator pace pulses delivered
per minute (atrium or ventricle) in the absence of

- Sensed intrinsic activity
- Sensor-controlled pacing at a higher rate

Consequently, it defines the maximum delay until a pulse is made.

The LRL is affected in the following ways:

- The LRL shall define the longest allowable pacing interval
- In VVT, the LRL interval starts at a ventricular sensed or paced event
- In AOO, the LRL interval starts at an atrial sensed or paced event

**Unit:** Pulse per Minute (ppm)\
**Range:** 50-90ppm\
**Default:** 60ppm

## `param.pulse_amplitude`: Atrial and Ventricular Amplitude

The amplitude of the generated pacing pulse.
The atrial and ventricular pacing pulse amplitudes shall be independently programmable.

**Unit:** Volt (V)\
**Range:** 3.5-7.0V\
**Default:** 3.5V

## `param.pulse_width`: Pulse Width

The width of the generated pacing pulse.
The atrial and ventricular pacing pulse width shall be independently programmable.

**Unit:** Milliseconds (ms)\
**Range:** 0.1-1.9ms\
**Default:** 0.4ms

## `param.vrp`: Ventricular Refractory Period (VRP)

The Ventricular Refractory Period shall be the programmed time interval following a ventricular event during which time ventricular senses shall not inhibit nor trigger pacing.

**Unit:** Milliseconds (ms)\
**Range:** 150-500ms\
**Default:** 320ms
