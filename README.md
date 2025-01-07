# pacemaker

Implementation of a basic pacemaker to showcase requirements tracing using [mantra](https://github.com/mhatzl/mantra).

The demo contains `no_std` Rust code and a C header file
to demonstrate requirements traceability over different programming languages.

## Prerequisite

The following tools and hardware is needed for this demo: 

- Install the [Rust toolchain](https://www.rust-lang.org/tools/install)
- Install [Clang for bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html) to demonstrate C interop
- Ensure a [native C Compiler](https://docs.rs/cc/latest/cc/#compile-time-requirements) is available (needed to install *mantra*)
- Install the [embedded-runner](https://github.com/mhatzl/embedded-runner) via `cargo install embedded-runner`
- Install [mantra](https://github.com/mhatzl/mantra) via `cargo install mantra`
- Install the [ARM GNU toolchain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)
- Install [OpenOCD](https://openocd.org/pages/getting-openocd.html)
- Ensure `arm-none-eabi-gdb` and `openocd` are on `PATH`
- Connect a [XMC4700](https://www.infineon.com/cms/de/product/evaluation-boards/kit_xmc47_relax_v1/) board from Infineon to your PC

## Usage

- Run the demo

  The demo may be run via
  
  ```
  cargo run
  ```
  
  This will print heartbeat or pacemaker pulses as logs to the terminal.

  The pacemaker has three modes: VVT, AOO, Off
  You may change the mode used for the demo in `main.rs`.

  ```rs
  let mode = Mode::Vvt;
  ```

- Run tests and collect requirement coverage

  Tests may be run via

  ```
  cargo test --test integration
  ```

  This will run the integration tests in `tests/integration.rs`.
  Because the `defmt` feature of `mantra-rust-macros` is enabled for this demo,
  requirement coverage information is automatically gathered during test execution.

  The coverage information may be collected and stored inside `coverage.json` via

  ```
  embedded-runner collect coverage.json
  ```

  **Note:** You may skip this step, because coverage of a previous test run already exists in the repository.

- Collect language server information

  To correctly resolve the fully qualified name of a language element, *mantra* uses information stored
  in the [Language Server Index Format](https://microsoft.github.io/language-server-protocol/specifications/lsif/0.6.0/specification/).

  For Rust projects, this information can be generated via `rust-analyzer` by executing the following command:

  ```
  rust-analyzer lsif . > lsif.json
  ```

  This overwrites the existing `lsif.json` file that already contains the information for the unmodified pacemaker project.

- Setting up mantra

  To collect available requirements, traces, coverage, and reviews, run

  ```
  mantra collect
  ```

  This will use the configuration from `mantra.toml` and store everything inside the `mantra.db` file.

- Generating a traceability report

  A default traceability report may be created via

  ```
  mantra report --formats html --project-name pacemaker --project-version 0.1.0 mantra_report.html
  ```

  This will create an HTML report using the default report template provided by *mantra*.
