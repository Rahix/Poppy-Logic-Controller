MicroPython Firmware for Poppy Logic Controller
-----------------------------------------------
In this directory, you can find a bare-bones MicroPython firmware for the Poppy
Logic Controller.  You can use this as a starting point for developing your own
programs.

To get started, you need to install MicroPython on the Raspberry Pi Pico.
Information about this can be found here:
<https://www.raspberrypi.com/documentation/microcontrollers/micropython.html>

Next, install the `mpremote` tool to interact with the Pico:

```bash
pin install mpremote
```

Next, if you just want to try out some code, you can upload it to the Pico like
this:

```bash
mpremote run poppy.py
```

If you want to upload the program permanently, so the Pico automatically runs
it on power-on, use this command:

```bash
mpremote cp poppy.py :main.py
```

**Note the `:` before `main.py`!**
