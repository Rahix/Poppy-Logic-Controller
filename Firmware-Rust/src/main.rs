// SPDX-License-Identifier: MIT
#![no_std]
#![no_main]

use embedded_time::rate::*;
use rp_pico::hal;
use rp_pico::hal::pac;

use embedded_hal::digital::v2::OutputPin as _;
use panic_halt as _;
use hal::Clock as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
    let _timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    let sio = hal::sio::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let i2c0 = hal::i2c::I2C::i2c0(
        pac.I2C0,
        pins.gpio20.into_mode(),
        pins.gpio21.into_mode(),
        100.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock,
    );

    let bus = shared_bus::BusManagerSimple::new(i2c0);

    let mut inp0 = port_expander::Pca9538::new(bus.acquire_i2c(), false, false);
    let mut inp1 = port_expander::Pca9538::new(bus.acquire_i2c(), true, false);
    let mut out0 = port_expander::Pca9538::new(bus.acquire_i2c(), false, true);
    let mut out1 = port_expander::Pca9538::new(bus.acquire_i2c(), true, true);

    let parts = inp0.split();
    let _i00 = parts.io0.into_inverted().unwrap();
    let _i01 = parts.io1.into_inverted().unwrap();
    let _i02 = parts.io2.into_inverted().unwrap();
    let _i03 = parts.io3.into_inverted().unwrap();
    let _i04 = parts.io4.into_inverted().unwrap();
    let _i05 = parts.io5.into_inverted().unwrap();
    let _i06 = parts.io6.into_inverted().unwrap();
    let _i07 = parts.io7.into_inverted().unwrap();

    let parts = inp1.split();
    let _i08 = parts.io0.into_inverted().unwrap();
    let _i09 = parts.io1.into_inverted().unwrap();
    let _i10 = parts.io2.into_inverted().unwrap();
    let _i11 = parts.io3.into_inverted().unwrap();
    let _i12 = parts.io4.into_inverted().unwrap();
    let _i13 = parts.io5.into_inverted().unwrap();
    let _i14 = parts.io6.into_inverted().unwrap();
    let _i15 = parts.io7.into_inverted().unwrap();

    let parts = out0.split();
    let _o00 = parts.io0.into_output().unwrap();
    let _o01 = parts.io1.into_output().unwrap();
    let _o02 = parts.io2.into_output().unwrap();
    let _o03 = parts.io3.into_output().unwrap();
    let _o04 = parts.io4.into_output().unwrap();
    let _o05 = parts.io5.into_output().unwrap();
    let _o06 = parts.io6.into_output().unwrap();
    let _o07 = parts.io7.into_output().unwrap();

    let parts = out1.split();
    let _o08 = parts.io0.into_output().unwrap();
    let _o09 = parts.io1.into_output().unwrap();
    let _o10 = parts.io2.into_output().unwrap();
    let _o11 = parts.io3.into_output().unwrap();
    let _o12 = parts.io4.into_output().unwrap();
    let _o13 = parts.io5.into_output().unwrap();
    let _o14 = parts.io6.into_output().unwrap();
    let _o15 = parts.io7.into_output().unwrap();

    #[rustfmt::skip]
    let mut all_outputs = [
        _o00, _o01, _o02, _o03, _o04, _o05, _o06, _o07,
        _o08, _o09, _o10, _o11, _o12, _o13, _o14, _o15,
    ];

    let mut led = pins.led.into_push_pull_output();
    led.set_high().unwrap();

    let mut state = false;
    loop {
        // make the outputs blink happily for demo purposes
        for (i, o) in all_outputs.iter_mut().enumerate() {
            if i % 2 == 0 {
                o.set_state(state.into()).unwrap();
            } else {
                o.set_state((!state).into()).unwrap();
            }
        }
        state = !state;
        delay.delay_ms(500);
    }
}
