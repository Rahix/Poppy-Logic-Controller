IEC 61131-3 Firmware for Poppy Logic Controller
-----------------------------------------------
The firmware in this directory allows writing control programs for the Poppy
Logic Controller in the IEC 61131-3 textual language _ST_ (_Structured Text_).

This is made possible by the [matiec] open-source 61131-3 compiler.  This
compiler is used to compile the _ST_ code into the C language from where it
is then linked against a Rust firmware which drives the I/O.

### Running It
The build process is entirely driven by cargo.  [matiec] is installed
automatically on first build.  The only dependency you need is, as for the Rust
firmware, `elf2uf2-rs`:

```bash
cargo install elf2uf2-rs
```

Then, put the Pico into bootloader mode by first pressing the `BOOTSEL` button
and then, while keeping it held down, pressing the reset button.

Now you can build and deploy your 61131-3 firmware:

```bash
cargo run
```

### Programming Quickstart
You might want to read up on the basics of _ST_ somewhere first.  I am going to
detail some [matiec] and Poppy Logic specific things in this section of the
README.

The program is found in [`src/main.st`][main.st].  In contrast to a lot of PLC
programming IDEs, the entire _ST_ program must be written here, including the
top-level program and configuration (which is usually handled in GUI).  You can
take the existing code as a basis. Your top-level code goes into the body of
the `PROGRAM`.

Further tasks and function blocks can also be added to `main.st`.
Alternatively, you can put them in separate files and include those using:

```text
{#include "filename.st"}
```

For I/O, the following tags exist and map to the Poppy Logic Controllers 16
inputs and 16 outputs:

- Inputs: `%IX00`, `%IX01`, `%IX02`, `%IX03`, `%IX04`, `%IX05`, `%IX06`,
  `%IX07`, `%IX08`, `%IX09`, `%IX10`, `%IX11`, `%IX12`, `%IX13`, `%IX14`,
  `%IX15`
- Outputs: `%QX00`, `%QX01`, `%QX02`, `%QX03`, `%QX04`, `%QX05`, `%QX06`,
  `%QX07`, `%QX08`, `%QX09`, `%QX10`, `%QX11`, `%QX12`, `%QX13`, `%QX14`,
  `%QX15`

Byte-access or word-access to these tags is not possible.

### Modbus
This firmware also comes with a [Modbus RTU] interface for interaction with
other devices.  The interface is available over USB, where the Poppy Logic
Controller will announce itself as a serial port.

A few registers are mapped to Modbus for communication:

- Modbus Coils 0 - 15 of unit 1 are mapped to `%MX00` - `%MX15`.
- Modbus Holding Registers 0 - 15 of unit 1 are mapped to `%MW00` - `%MW15`.
  Holding registers are 16 bits wide.

Additionally, the state of all inputs and outputs is available _read-only_
through Modbus discrete inputs:

- Inputs `%IX00` - `%IX15` are mapped to discrete inputs 0 - 15.
- Outputs `%QX00` - `%QX15` are mapped to discrete inputs 16 - 31.

(This may be useful for visualization purposes).

As a quick example, using the [pymodbus REPL] you can set coils or dump the
state of inputs and outputs:

```console
$ pymodbus.console serial --port /dev/ttyACM0

> # Set a coil
> client.write_coil unit=1 address=0 value=1
> # And reset it again
> client.write_coil unit=1 address=0 value=0

> # Or dump all physical inputs and outputs
> client.read_discrete_inputs unit=1 address=0 count=32
{
    "bits": [
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        true,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        true,
        false,
        false,
        false,
        false,
        false,
        false
    ]
}
```

[matiec]: https://github.com/beremiz/matiec
[main.st]: src/main.st
[Modbus RTU]: https://en.wikipedia.org/wiki/Modbus
[pymodbus REPL]: https://pymodbus.readthedocs.io/en/latest/source/library/REPL.html
