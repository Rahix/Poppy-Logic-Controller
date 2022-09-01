#!/usr/bin/env python3
"""
A GUI that displays status of all inputs, outputs, and modbus registers of the
Poppy Logic Controller.
"""

import argparse
import tkinter as tk
from tkinter import ttk

from pymodbus.client.sync import ModbusSerialClient

FONT = ("osifont", 12, "normal")


class App:
    def __init__(self, parent, modbus_client):
        self.frame = ttk.Frame(parent, padding=(40, 40))
        self.modbus_client = modbus_client

        # Variables
        self.inputs = [tk.BooleanVar(value=False) for _ in range(16)]
        self.outputs = [tk.BooleanVar(value=False) for _ in range(16)]
        self.mb_coils = [tk.BooleanVar(value=False) for _ in range(16)]
        self.mb_regs = [[tk.StringVar(value="0")] for _ in range(16)]
        self.do_full_io_update()

        # Widgets
        label = ttk.Label(text="Inputs", padding=(5, 0))
        self.input_frame = ttk.LabelFrame(
            self.frame, labelwidget=label, padding=(20, 10)
        )
        self.input_frame.grid(row=0, column=0)
        ttk.Label(self.input_frame, text="Status", padding=(10, 0)).grid(
            row=0, column=0
        )
        for i, inp in enumerate(self.inputs):
            rb = ttk.Radiobutton(self.input_frame, variable=inp)
            rb.grid(row=i + 1, column=0)
            rb.state(["disabled"])
            lbl = ttk.Label(self.input_frame, text=f"%IX{i:02}")
            lbl.grid(row=i + 1, column=1)

        self.poppy_img = tk.PhotoImage(file="../Docs/img/poppy-small.png")
        lbl = ttk.Label(self.frame, image=self.poppy_img, padding=(20, 20))
        lbl.grid(row=0, column=1)

        label = ttk.Label(text="Outputs", padding=(5, 0))
        self.output_frame = ttk.LabelFrame(
            self.frame, labelwidget=label, padding=(20, 10)
        )
        self.output_frame.grid(row=0, column=2)
        ttk.Label(self.output_frame, text="Active", padding=(10, 0)).grid(
            row=0, column=0
        )
        for i, out in enumerate(self.outputs):
            rb = ttk.Radiobutton(self.output_frame, variable=out)
            rb.grid(row=i + 1, column=0)
            rb.state(["disabled"])
            ttk.Label(self.output_frame, text=f"%QX{i:02}").grid(row=i + 1, column=1)

        label = ttk.Label(text="Modbus Coils", padding=(5, 0))
        self.modbus_coil_frame = ttk.LabelFrame(
            self.frame, labelwidget=label, padding=(20, 10)
        )
        self.modbus_coil_frame.grid(row=0, column=3)
        ttk.Label(self.modbus_coil_frame, text="Active", padding=(10, 0)).grid(
            row=0, column=0
        )
        for i, coil in enumerate(self.mb_coils):
            cb = ttk.Checkbutton(
                self.modbus_coil_frame,
                variable=coil,
                command=lambda num=i: self.update_coil(num),
            )
            cb.grid(row=i + 1, column=0)
            lbl = ttk.Label(self.modbus_coil_frame, text=f"%MX{i:02}")
            lbl.grid(row=i + 1, column=1)

        label = ttk.Label(text="Modbus Holding Registers", padding=(5, 0))
        self.modbus_regs_frame = ttk.LabelFrame(
            self.frame, labelwidget=label, padding=(20, 10)
        )
        self.modbus_regs_frame.grid(row=1, column=0, columnspan=4)
        validation = self.frame.register(str.isdigit)
        for i, reg in enumerate(self.mb_regs):
            lbl = ttk.Label(self.modbus_regs_frame, text=f"%MW{i:02}")
            lbl.grid(row=0 if i < 8 else 2, column=i % 8)
            e = ttk.Entry(
                self.modbus_regs_frame,
                textvariable=reg[0],
                validate="key",
                validatecommand=(validation, "%S"),
            )
            e.grid(row=1 if i < 8 else 3, column=i % 8)
            e.bind("<Return>", lambda *args, num=i: self.update_register(num))
            reg.append(e)

        self.frame.grid()

        self.frame.after(1, self.io_update_cycle)

    def update_coil(self, i):
        value = 1 if self.mb_coils[i].get() else 0
        print(f"Setting coil {i} to {value}.")
        self.modbus_client.write_coil(unit=1, address=i, value=value)

    def update_register(self, i):
        value = int(self.mb_regs[i][0].get())
        if value > 65535:
            print(f"Value for register {i} is too big, ignoring! ({value})")
            return

        print(f"Setting holding register {i} to {value}.")
        self.modbus_client.write_register(unit=1, address=i, value=value)

    def io_update_cycle(self):
        self.do_full_io_update()
        self.frame.after(200, self.io_update_cycle)

    def do_full_io_update(self):
        response = self.modbus_client.read_discrete_inputs(unit=1, address=0, count=16)
        for inp, val in zip(self.inputs, response.bits):
            inp.set(val)

        response = self.modbus_client.read_discrete_inputs(unit=1, address=16, count=16)
        for out, val in zip(self.outputs, response.bits):
            out.set(val)

        response = self.modbus_client.read_coils(unit=1, address=0, count=16)
        for coil, val in zip(self.mb_coils, response.bits):
            coil.set(val)

        response = self.modbus_client.read_holding_registers(
            unit=1, address=0, count=16
        )
        for reg, val in zip(self.mb_regs, response.registers):
            # Don't update this element when it is currently focused as the
            # user is probably trying to update the value...
            try:
                if "focus" in reg[1].state():
                    continue
            except IndexError:
                pass
            reg[0].set(val)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "port",
        nargs="?",
        default="/dev/ttyACM0",
        help="Serial port (default /dev/ttyACM0)",
    )
    args = parser.parse_args()

    modbus_client = ModbusSerialClient(method="rtu", port=args.port)
    modbus_client.connect()

    root = tk.Tk()
    root.title("PLC GUI")

    root.rowconfigure(0, weight=1)
    root.columnconfigure(0, weight=1)

    ttk.Style().theme_use("clam")
    root.option_add("*Font", FONT)

    app = App(root, modbus_client)

    root.update_idletasks()  # Make sure every screen redrawing is done

    root.mainloop()


if __name__ == "__main__":
    main()
