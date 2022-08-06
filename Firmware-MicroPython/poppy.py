# SPDX-License-Identifier: MIT
import time

import machine
from machine import Pin


class Pca9538:
    INPUT = 0x00
    OUTPUT = 0x01
    POLARITY = 0x02
    CONFIG = 0x03


class PoppyIo:
    def __init__(self):
        self.bus = machine.I2C(0, sda=Pin(20), scl=Pin(21), freq=100000)
        self.out = 0x0000

        # Configure all pins appropriately.  Inputs must be inverted...
        self.bus.writeto_mem(0x70, Pca9538.POLARITY, bytes([0xFF]))
        self.bus.writeto_mem(0x71, Pca9538.POLARITY, bytes([0xFF]))

        # ...and outputs zeroes and their mode configured.
        self.bus.writeto_mem(0x72, Pca9538.OUTPUT, bytes([0x00]))
        self.bus.writeto_mem(0x73, Pca9538.OUTPUT, bytes([0x00]))
        self.bus.writeto_mem(0x72, Pca9538.CONFIG, bytes([0x00]))
        self.bus.writeto_mem(0x73, Pca9538.CONFIG, bytes([0x00]))

    def _update_outputs(self):
        self.bus.writeto_mem(0x72, Pca9538.OUTPUT, bytes([self.out & 0xFF]))
        self.bus.writeto_mem(0x73, Pca9538.OUTPUT, bytes([(self.out >> 8) & 0xFF]))

    def read_all_inputs_bytes(self):
        """
        Read all inputs into a 16-bit integer.  The lowest bit corresponds to
        the first input I00.
        """
        low = self.bus.readfrom_mem(0x70, Pca9538.INPUT, 1)[0]
        high = self.bus.readfrom_mem(0x71, Pca9538.INPUT, 1)[0]
        return (high << 8) | low

    def write_all_outputs_bytes(self, values):
        """
        Write all outputs from a 16-bit integer. 0xffff turns all outputs on.
        The lowest bit corresponds to the first output Q00.
        """
        self.out = values
        self._update_outputs()

    def read_all_inputs(self):
        """
        Read all inputs into a list with 16 elements.
        """
        values = self.read_all_inputs_bytes()
        result = [0x00] * 16
        for i in range(16):
            result[i] = (values & (1 << i)) != 0
        return result

    def write_all_outputs(self, values):
        """
        Write all outputs from a list with 16 elements.
        """
        assert len(values) <= 16
        b = 0x0000
        for i, v in enumerate(values):
            if v:
                b |= 1 << i
        self.write_all_outputs_bytes(b)

    def write_single(self, pin, value):
        """
        Write single output ``pin`` to ``value``.

        Note that it is usually a better idea to set all outputs at once using
        ``write_all_outputs()``.
        """
        if value:
            self.out |= 1 << pin
        else:
            self.out &= ~(1 << pin)
        self._update_outputs()

    def read_single(self, pin):
        """
        Read a single input ``pin``.

        Note that it is usually a better idea to read all inputs at once using
        ``read_all_inputs()``.
        """
        result = self.read_all_inputs_bytes()
        return (result & (1 << pin)) != 0


# --- Demo Program Below ---

# status
led = Pin(25, Pin.OUT)
led.on()

# outputs
io = PoppyIo()
while True:
    for i in range(16):
        io.write_all_outputs_bytes(1 << i)
        time.sleep_ms(60)
    led.toggle()
