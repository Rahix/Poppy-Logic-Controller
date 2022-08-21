{#include "blink.st"}

PROGRAM prog
    VAR (* inputs *)
        btn_start               AT %IX08: BOOL;
        btn_stop                AT %IX09: BOOL;
    END_VAR
    VAR (* outputs *)
        led_orange              AT %QX08: BOOL;
        led_green               AT %QX09: BOOL;
    END_VAR
    VAR (* variables *)
        running: BOOL;
        tmr_start: TON;
        tmr_stop: TON;
        blinker: BLINK;
    END_VAR
    VAR_TEMP
        waiting: BOOL;
    END_VAR

    blinker(on_time := t#300ms, off_time := t#300ms);
    tmr_start(in := btn_start, pt := t#1000ms);
    tmr_stop(in := btn_stop, pt := t#1000ms);

    running := (tmr_start.q OR running) AND NOT tmr_stop.q;
    waiting := (not running and tmr_start.et <> t#0s)
            OR (running and tmr_stop.et <> t#0s);

    led_green := running;
    led_orange := waiting AND blinker.q;
END_PROGRAM

CONFIGURATION std_conf
    RESOURCE std_resource on plc
        TASK task_main(interval := T#10ms, priority := 0);
        PROGRAM inst0 with task_main : prog;
    END_RESOURCE
END_CONFIGURATION
