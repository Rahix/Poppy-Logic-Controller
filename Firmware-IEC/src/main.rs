// SPDX-License-Identifier: MIT
#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::rprintln;

use embedded_time::rate::*;
use rp_pico::hal;
use rp_pico::hal::pac;

use embedded_hal::digital::v2::OutputPin as _;
use embedded_hal::digital::v2::ToggleableOutputPin as _;
use hal::Clock as _;
// use panic_halt as _;

extern "C" {
    fn config_init__();
    fn config_run__(tick: cty::c_ulong);
    static common_ticktime__: cty::c_ulonglong;
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct IecTimespec {
    pub tv_sec: cty::c_long,
    pub tv_nsec: cty::c_long,
}

#[export_name = "__CURRENT_TIME"]
static mut CURRENT_TIME: IecTimespec = IecTimespec {
    tv_sec: 0,
    tv_nsec: 0,
};

// I/O
mod inputs {
    #[export_name = "__I0_0"]
    static mut I00_ADDR: *const u8 = unsafe { &I00 };
    pub static mut I00: u8 = 0;
    #[export_name = "__I0_1"]
    static mut I01_ADDR: *const u8 = unsafe { &I01 };
    pub static mut I01: u8 = 0;
    #[export_name = "__I0_2"]
    static mut I02_ADDR: *const u8 = unsafe { &I02 };
    pub static mut I02: u8 = 0;
    #[export_name = "__I0_3"]
    static mut I03_ADDR: *const u8 = unsafe { &I03 };
    pub static mut I03: u8 = 0;
    #[export_name = "__I0_4"]
    static mut I04_ADDR: *const u8 = unsafe { &I04 };
    pub static mut I04: u8 = 0;
    #[export_name = "__I0_5"]
    static mut I05_ADDR: *const u8 = unsafe { &I05 };
    pub static mut I05: u8 = 0;
    #[export_name = "__I0_6"]
    static mut I06_ADDR: *const u8 = unsafe { &I06 };
    pub static mut I06: u8 = 0;
    #[export_name = "__I0_7"]
    static mut I07_ADDR: *const u8 = unsafe { &I07 };
    pub static mut I07: u8 = 0;
    #[export_name = "__I0_8"]
    static mut I08_ADDR: *const u8 = unsafe { &I08 };
    pub static mut I08: u8 = 0;
    #[export_name = "__I0_9"]
    static mut I09_ADDR: *const u8 = unsafe { &I09 };
    pub static mut I09: u8 = 0;
    #[export_name = "__I1_0"]
    static mut I10_ADDR: *const u8 = unsafe { &I10 };
    pub static mut I10: u8 = 0;
    #[export_name = "__I1_1"]
    static mut I11_ADDR: *const u8 = unsafe { &I11 };
    pub static mut I11: u8 = 0;
    #[export_name = "__I1_2"]
    static mut I12_ADDR: *const u8 = unsafe { &I12 };
    pub static mut I12: u8 = 0;
    #[export_name = "__I1_3"]
    static mut I13_ADDR: *const u8 = unsafe { &I13 };
    pub static mut I13: u8 = 0;
    #[export_name = "__I1_4"]
    static mut I14_ADDR: *const u8 = unsafe { &I14 };
    pub static mut I14: u8 = 0;
    #[export_name = "__I1_5"]
    static mut I15_ADDR: *const u8 = unsafe { &I15 };
    pub static mut I15: u8 = 0;
}

mod outputs {
    #[export_name = "__Q0_0"]
    static mut Q00_ADDR: *const u8 = unsafe { &Q00 };
    pub static mut Q00: u8 = 0;
    #[export_name = "__Q0_1"]
    static mut Q01_ADDR: *const u8 = unsafe { &Q01 };
    pub static mut Q01: u8 = 0;
    #[export_name = "__Q0_2"]
    static mut Q02_ADDR: *const u8 = unsafe { &Q02 };
    pub static mut Q02: u8 = 0;
    #[export_name = "__Q0_3"]
    static mut Q03_ADDR: *const u8 = unsafe { &Q03 };
    pub static mut Q03: u8 = 0;
    #[export_name = "__Q0_4"]
    static mut Q04_ADDR: *const u8 = unsafe { &Q04 };
    pub static mut Q04: u8 = 0;
    #[export_name = "__Q0_5"]
    static mut Q05_ADDR: *const u8 = unsafe { &Q05 };
    pub static mut Q05: u8 = 0;
    #[export_name = "__Q0_6"]
    static mut Q06_ADDR: *const u8 = unsafe { &Q06 };
    pub static mut Q06: u8 = 0;
    #[export_name = "__Q0_7"]
    static mut Q07_ADDR: *const u8 = unsafe { &Q07 };
    pub static mut Q07: u8 = 0;
    #[export_name = "__Q0_8"]
    static mut Q08_ADDR: *const u8 = unsafe { &Q08 };
    pub static mut Q08: u8 = 0;
    #[export_name = "__Q0_9"]
    static mut Q09_ADDR: *const u8 = unsafe { &Q09 };
    pub static mut Q09: u8 = 0;
    #[export_name = "__Q1_0"]
    static mut Q10_ADDR: *const u8 = unsafe { &Q10 };
    pub static mut Q10: u8 = 0;
    #[export_name = "__Q1_1"]
    static mut Q11_ADDR: *const u8 = unsafe { &Q11 };
    pub static mut Q11: u8 = 0;
    #[export_name = "__Q1_2"]
    static mut Q12_ADDR: *const u8 = unsafe { &Q12 };
    pub static mut Q12: u8 = 0;
    #[export_name = "__Q1_3"]
    static mut Q13_ADDR: *const u8 = unsafe { &Q13 };
    pub static mut Q13: u8 = 0;
    #[export_name = "__Q1_4"]
    static mut Q14_ADDR: *const u8 = unsafe { &Q14 };
    pub static mut Q14: u8 = 0;
    #[export_name = "__Q1_5"]
    static mut Q15_ADDR: *const u8 = unsafe { &Q15 };
    pub static mut Q15: u8 = 0;
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

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
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

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
    let mut _o00 = parts.io0.into_output().unwrap();
    let mut _o01 = parts.io1.into_output().unwrap();
    let mut _o02 = parts.io2.into_output().unwrap();
    let mut _o03 = parts.io3.into_output().unwrap();
    let mut _o04 = parts.io4.into_output().unwrap();
    let mut _o05 = parts.io5.into_output().unwrap();
    let mut _o06 = parts.io6.into_output().unwrap();
    let mut _o07 = parts.io7.into_output().unwrap();

    let parts = out1.split();
    let mut _o08 = parts.io0.into_output().unwrap();
    let mut _o09 = parts.io1.into_output().unwrap();
    let mut _o10 = parts.io2.into_output().unwrap();
    let mut _o11 = parts.io3.into_output().unwrap();
    let mut _o12 = parts.io4.into_output().unwrap();
    let mut _o13 = parts.io5.into_output().unwrap();
    let mut _o14 = parts.io6.into_output().unwrap();
    let mut _o15 = parts.io7.into_output().unwrap();

    let tick_delay_us = unsafe { common_ticktime__ } / 1000;

    let mut led = pins.led.into_push_pull_output();

    unsafe {
        config_init__();
    }

    led.set_high().unwrap();

    let mut tick = 0;
    loop {
        let pins0 = [&_i00, &_i01, &_i02, &_i03, &_i04, &_i05, &_i06, &_i07];
        let pins1 = [&_i08, &_i09, &_i10, &_i11, &_i12, &_i13, &_i14, &_i15];
        let inputs0 = port_expander::read_multiple(pins0).unwrap();
        let inputs1 = port_expander::read_multiple(pins1).unwrap();
        unsafe {
            inputs::I00 = inputs0[0].into();
            inputs::I01 = inputs0[1].into();
            inputs::I02 = inputs0[2].into();
            inputs::I03 = inputs0[3].into();
            inputs::I04 = inputs0[4].into();
            inputs::I05 = inputs0[5].into();
            inputs::I06 = inputs0[6].into();
            inputs::I07 = inputs0[7].into();
            inputs::I08 = inputs1[0].into();
            inputs::I09 = inputs1[1].into();
            inputs::I10 = inputs1[2].into();
            inputs::I11 = inputs1[3].into();
            inputs::I12 = inputs1[4].into();
            inputs::I13 = inputs1[5].into();
            inputs::I14 = inputs1[6].into();
            inputs::I15 = inputs1[7].into();
        }

        let count = timer.get_counter();
        let seconds = count / 1000000;
        let nanos = (count % 1000000) * 1000;
        unsafe {
            CURRENT_TIME = IecTimespec {
                tv_sec: seconds.try_into().unwrap(),
                tv_nsec: nanos.try_into().unwrap(),
            };
        }

        unsafe {
            config_run__(tick);
        }

        let output_states0 = unsafe {
            [
                outputs::Q00 != 0,
                outputs::Q01 != 0,
                outputs::Q02 != 0,
                outputs::Q03 != 0,
                outputs::Q04 != 0,
                outputs::Q05 != 0,
                outputs::Q06 != 0,
                outputs::Q07 != 0,
            ]
        };

        let output_states1 = unsafe {
            [
                outputs::Q08 != 0,
                outputs::Q09 != 0,
                outputs::Q10 != 0,
                outputs::Q11 != 0,
                outputs::Q12 != 0,
                outputs::Q13 != 0,
                outputs::Q14 != 0,
                outputs::Q15 != 0,
            ]
        };
        let pins0 = [
            &mut _o00, &mut _o01, &mut _o02, &mut _o03, &mut _o04, &mut _o05, &mut _o06, &mut _o07,
        ];
        let pins1 = [
            &mut _o08, &mut _o09, &mut _o10, &mut _o11, &mut _o12, &mut _o13, &mut _o14, &mut _o15,
        ];
        port_expander::write_multiple(pins0, output_states0).unwrap();
        port_expander::write_multiple(pins1, output_states1).unwrap();

        let _ = led.set_state(((count / 1000000) % 2 == 0).into());

        tick += 1;
        delay.delay_us(tick_delay_us.try_into().unwrap());
    }
}
