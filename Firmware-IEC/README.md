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

[matiec]: https://github.com/beremiz/matiec
[main.st]: src/main.st
