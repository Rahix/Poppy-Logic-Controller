Notes & ToDo List for the IO Board
==================================

### Inputs
- [x] Indicator LEDs
- [x] LED series resistor verification: ~1mA
- [x] ESD safety: 74VHC14 has protection
- [x] Re-verify current and voltage
- [x] Verify limiting values: Maximum input voltage for 5.5V out is 27.6V.
  Limiting value (7V) is reached at 35V.
- [x] Verify logic step-down:  Multiple 74VHC14 data-sheets mention that it can
  be used to step down to Vcc.
- [x] Defined initial state for inverters?
  - 680R pulls down to GND

### Outputs
- [x] Indicator LEDs
- [x] LED series resistor verification ~1mA
- [x] Driver: ULN2003A
- [x] Compatibility with inductive loads
  - Compatible as long as 24V supply is connected (freewheeling diodes included
    in driver package).
- [x] Check drive capability of port-expander (LEDs + drivers)
- [x] ESD safety: ULN2003A has protection
- [x] defined initial state for drivers?
  - Added 10k pull-downs.
- [ ] Must use a smaller polyfuse footprint

### Power Supply
- [ ] Add 24V to 5/3.3V step down or power from 5V?
- [x] Screw-in terminals: Using Phoenix screw terminals for power

### Pico MCU
- [x] Reset Button
- [x] Additional IO?
  - Added a UART for the second board, just in case.

### Misc
- [x] I2C addresses
- [x] Mounting holes
- [x] Board-to-Board Connector Pinout
