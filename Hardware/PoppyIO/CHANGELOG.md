## UNRELEASED
- Drop _DNP_ note from LM78xx regulator and its capacitors.  The regulator is
  used to step-down the 24V input to 5V for the Raspberry Pi Pico.  Not using
  the regulator means a separate 5V supply is needed which is pretty pointless.
- Removed the screw-terminal J9 for an external 5V supply.
- Switched Screw-Terminal for 24V supply to a 2-pin variant.

## Revision C - 2022-06-26
- Fixup wrong resistor values in DI stage.  The 680Ω resistors were meant to be
  820Ω from the beginning but this was not copied correctly.  With the wrong
  value, the DI limiting values are slightly off from what is specified in the
  schematic.

## Revision B - 2022-04-25
- Switch to a 14-pin IDC connector.

## Revision A - 2022-04-24
Initial.
