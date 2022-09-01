(*
 * Sample Program
 * ==============
 * This program demos a simple start-stop circuit where the start and stop
 * buttens have to be held for 0.8s before the operation is performed.  While a
 * button is pressed, an indicator blinks to give immediate feedback.
 *)

{#include "blink.st"}

PROGRAM MAIN
    VAR (* Inputs *)
        xBtnStart           AT %IX08:   BOOL;
        xBtnStop            AT %IX09:   BOOL;

        (* Can be controlled via modbus coils 0 and 1 *)
        xBtnStartModbus     AT %MX00:   BOOL;
        xBtnStopModbus      AT %MX01:   BOOL;
    END_VAR
    VAR (* Outputs *)
        xIndicatorOrange    AT %QX08:   BOOL := FALSE;
        xIndicatorGreen     AT %QX09:   BOOL := FALSE;

        xMotor              AT %QX00:   BOOL := FALSE;
    END_VAR
    VAR (* Variables *)
        xRunning:                       BOOL;
        xWaiting:                       BOOL;
        tmrStart:                       TON;
        tmrStop:                        TON;
        fbBlink:                        FB_Blink;
    END_VAR
    VAR_TEMP
        xWaitingStart:                  BOOL;
        xWaitingStop:                   BOOL;
    END_VAR

    fbBlink(tOnDelay := t#300ms, tOffDelay := t#300ms);
    tmrStart(IN := xBtnStart OR xBtnStartModbus, PT := t#800ms);
    tmrStop(IN := xBtnStop OR xBtnStopModbus, PT := t#800ms);

    xWaitingStart := tmrStart.IN AND NOT tmrStart.Q;
    xWaitingStop := tmrStop.IN AND NOT tmrStop.Q;
    xWaiting := xWaitingStart OR xWaitingStop;

    xRunning := (tmrStart.Q OR xRunning) AND NOT tmrStop.Q;

    xMotor := xRunning;

    xIndicatorGreen := xRunning;
    xIndicatorOrange := xWaiting AND fbBlink.Q;
END_PROGRAM

(*
 * It is important to keep this configuration named `STD_CONF` and the resource
 * named `STD_RESOURCE`.  Everything else can be modified. For example, you can
 * add new tasks or change the interval here.
 *)
CONFIGURATION STD_CONF
    RESOURCE STD_RESOURCE ON POPPY_LOGIC_CONTROLLER
        TASK taskMain(INTERVAL := T#50ms, PRIORITY := 0);
        PROGRAM INST0 WITH taskMain : MAIN;
    END_RESOURCE
END_CONFIGURATION
