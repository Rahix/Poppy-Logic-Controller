PROGRAM prog
    VAR (* inputs *)
        btn_start               AT %I0.8: BOOL;
        btn_stop                AT %I0.9: BOOL;
    END_VAR
    VAR (* outputs *)
        led_orange              AT %Q0.8: BOOL;
        led_green               AT %Q0.9: BOOL;

        coil_cylinder           AT %Q0.0: BOOL;
    END_VAR
    VAR (* variables *)
        running: BOOL;
        tmr_start: TON;
        tmr_stop: TON;
    END_VAR

    tmr_start(in := btn_start, pt := t#10ms);
    tmr_stop(in := btn_stop, pt := t#10ms);

    running := (tmr_start.q OR running) AND NOT tmr_stop.q;

    led_green := running;
    led_orange := tmr_start.et <> t#0s OR tmr_stop.et <> t#0s;

    coil_cylinder := running;
END_PROGRAM

CONFIGURATION std_conf
    RESOURCE std_resource on plc
        TASK task_main(interval := T#50ms, priority := 0);
        PROGRAM inst0 with task_main : prog;
    END_RESOURCE
END_CONFIGURATION
