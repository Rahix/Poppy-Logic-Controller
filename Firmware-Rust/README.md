Rust Firmware for Poppy Logic Controller
----------------------------------------
In this directory, you can find a bare-bones Rust firmware for the Poppy Logic
Controller.  You can use this as a starting point for developing your own
programs.

To run this firmware, first install `elf2uf2-rs`, the tool to flash firmware
onto the Raspberry Pi Pico:

```bash
cargo install elf2uf2-rs
```

Then, put the Pico into bootloader mode by first pressing the `BOOTSEL` button
and then, while keeping it held down, pressing the reset button.

Now you're ready to upload your Rust program using a simple

```bash
cargo run
```
