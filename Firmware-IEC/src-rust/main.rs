// SPDX-License-Identifier: MIT
#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::rprintln;

use embedded_time::rate::*;
use rp_pico::hal;
use rp_pico::hal::pac;

use rmodbus::server::context::ModbusContext;

use embedded_hal::digital::v2::OutputPin as _;
use hal::Clock as _;

mod matiec;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    rprintln!("Hello Poppy Logic!");

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

    let mut led = pins.led.into_push_pull_output();

    let mut modbus_context = cortex_m::singleton!(:ModbusContext = ModbusContext::new()).unwrap();

    let mut iec_config = matiec::IecConfiguration::new().unwrap();

    led.set_high().unwrap();

    let usb_bus = usb_device::bus::UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut serial_port = usbd_serial::SerialPort::new(&usb_bus);

    let usb_id = usb_device::device::UsbVidPid(0xde5f, 0x3d21);
    let mut usb_dev = usb_device::device::UsbDeviceBuilder::new(&usb_bus, usb_id)
        .manufacturer("Rahix")
        .product("Poppy Logic Controller")
        .serial_number("POPPY123")
        .device_class(2)
        .build();

    loop {
        // Read potential changes from modbus
        iec_config.sync_from_modbus(&modbus_context);

        // Reading all inputs
        let pins0 = [&_i00, &_i01, &_i02, &_i03, &_i04, &_i05, &_i06, &_i07];
        let pins1 = [&_i08, &_i09, &_i10, &_i11, &_i12, &_i13, &_i14, &_i15];
        let inputs0 = port_expander::read_multiple(pins0).unwrap_or_else(|e| {
            rprintln!(
                "Failed reading inputs %IX00 - %IX07: {:?} - Faulting to zero state!",
                e
            );
            [false; 8]
        });
        let inputs1 = port_expander::read_multiple(pins1).unwrap_or_else(|e| {
            rprintln!(
                "Failed reading inputs %IX08 - %IX15: {:?} - Faulting to zero state!",
                e
            );
            [false; 8]
        });
        iec_config.write_inputs(inputs0.into_iter().chain(inputs1.into_iter()));

        // Logic evaluation
        let usecs = timer.get_counter();
        iec_config.eval_tick(usecs).unwrap_or_else(|e| {
            rprintln!("Error in logic evaluation: {:?}", e);
        });

        // Writing all outputs
        let mut outputs0 = [false; 8];
        let mut outputs1 = [false; 8];
        iec_config.read_outputs(outputs0.iter_mut().chain(outputs1.iter_mut()));
        let pins0 = [
            &mut _o00, &mut _o01, &mut _o02, &mut _o03, &mut _o04, &mut _o05, &mut _o06, &mut _o07,
        ];
        let pins1 = [
            &mut _o08, &mut _o09, &mut _o10, &mut _o11, &mut _o12, &mut _o13, &mut _o14, &mut _o15,
        ];
        port_expander::write_multiple(pins0, outputs0).unwrap_or_else(|e| {
            rprintln!(
                "Failed writing outputs %QX00 - %QX07: {:?} - State is unknown!",
                e
            );
        });
        port_expander::write_multiple(pins1, outputs1).unwrap_or_else(|e| {
            rprintln!(
                "Failed writing outputs %QX00 - %QX07: {:?} - State is unknown!",
                e
            );
        });

        // Write potential changes to modbus
        iec_config.sync_to_modbus(&mut modbus_context);

        // Wait for next tick
        let _ = led.set_state(((usecs / 1000000) % 2 == 0).into());
        let ticktime: u64 = iec_config.common_ticktime_us().try_into().unwrap();
        let wakeup = usecs + ticktime;

        // While waiting for the next PLC tick, handle USB
        while wakeup > timer.get_counter() {
            if usb_dev.poll(&mut [&mut serial_port]) {
                let mut request_buf = [0x00u8; 256];
                if let Ok(b) = serial_port.read(&mut request_buf) {
                    rprintln!("Message: {:?}", &request_buf[..b]);
                    let mut response = heapless::Vec::<u8, 256>::new();
                    let mut frame = rmodbus::server::ModbusFrame::new(
                        1,
                        &request_buf,
                        rmodbus::ModbusProto::Rtu,
                        &mut response,
                    );
                    if let Err(e) = frame.parse() {
                        rprintln!("Modbus Error: {}", e);
                    } else {
                        if frame.processing_required {
                            let res = match frame.readonly {
                                true => frame.process_read(&modbus_context),
                                false => frame.process_write(&mut modbus_context),
                            };
                            if let Err(e) = res {
                                rprintln!("Modbus Processing Error: {}", e);
                            } else {
                                if frame.response_required {
                                    frame.finalize_response().unwrap();
                                    serial_port.write(&response).unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
