(* A blinker which can be used to make indicator lights blink. *)

FUNCTION_BLOCK FB_Blink
    VAR_INPUT
        tOnDelay:   TIME;
        tOffDelay:  TIME;
    END_VAR
    VAR_OUTPUT
        Q:          BOOL := FALSE;
    END_VAR
    VAR
        tmrOn:      TON;
        tmrOff:     TON;
    END_VAR

    tmrOn(IN := Q, PT := tOnDelay);
    tmrOff(IN := NOT Q, PT := tOffDelay);

    Q := tmrOff.Q AND NOT tmrOn.Q;
END_FUNCTION_BLOCK
