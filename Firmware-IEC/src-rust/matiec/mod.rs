//! FFI Interface to the code generated from `matiec`
#![allow(dead_code)]

mod ext;

static mut INITIALIZED: bool = false;

/// Represents the 61131-3 "configuration" from `matiec`
pub struct IecConfiguration {
    tick: cty::c_ulong,
}

impl IecConfiguration {
    pub fn new() -> Option<Self> {
        // Singleton
        cortex_m::interrupt::free(|_| unsafe {
            assert_eq!(INITIALIZED, false);
            INITIALIZED = true;
        });

        // SAFETY: We asserted that this is the only instance of `IecConfiguration` so we can
        // safely call into the initialization.
        unsafe { Some(Self::new_unchecked()) }
    }

    /// ## Safety
    /// Must only be called once in a program.
    pub unsafe fn new_unchecked() -> Self {
        ext::config_init__();

        Self { tick: 0 }
    }

    /// Return the delay between ticks in microseconds.
    ///
    /// This delay is calculated by matiec and is a constant.
    pub fn common_ticktime_us(&self) -> u32 {
        // SAFETY: The value is effectively a constant.
        (unsafe { ext::common_ticktime__ } / 1000)
            .try_into()
            .unwrap()
    }

    /// Return the current tick.
    pub fn tick(&self) -> cty::c_ulong {
        self.tick
    }

    /// Sets the current time to the given microsenconds count.
    pub fn set_current_time_us(&mut self, usecs: u64) -> Result<(), core::num::TryFromIntError> {
        let timespec = ext::IecTimespec {
            tv_sec: (usecs / 1000000).try_into()?,
            tv_nsec: ((usecs % 1000000) * 1000).try_into()?,
        };
        unsafe {
            ext::CURRENT_TIME = timespec;
        }
        Ok(())
    }

    fn eval_raw(&mut self) {
        // SAFETY: Singleton guarantees non-reentrant execution.
        unsafe {
            ext::config_run__(self.tick);
        }
    }

    pub fn eval_tick(&mut self, usecs: u64) -> Result<(), core::num::TryFromIntError> {
        self.set_current_time_us(usecs)?;
        self.eval_raw();
        self.tick += 1;
        Ok(())
    }

    pub fn write_inputs<I: IntoIterator<Item = bool>>(&mut self, inputs: I) {
        let inputs_mut = unsafe { ext::inputs::all_mut() };
        for (v, i) in inputs.into_iter().zip(inputs_mut.into_iter()) {
            *i = if v { 0xff } else { 0x00 };
        }
    }

    pub fn read_outputs<'a, O: IntoIterator<Item = &'a mut bool>>(&mut self, outputs: O) {
        let outputs_mut = unsafe { ext::outputs::all_mut() };
        for (v, o) in outputs.into_iter().zip(outputs_mut.into_iter()) {
            *v = *o != 0x00;
        }
    }

    pub fn sync_to_modbus(&mut self, ctx: &mut rmodbus::server::context::ModbusContext) {
        let mem_bits_mut = unsafe { ext::mem_bits::all_mut() };
        for (plc_bit, modbus_bit) in mem_bits_mut.into_iter().zip(ctx.coils.iter_mut()) {
            *modbus_bit = *plc_bit != 0x00;
        }
        let mem_words_mut = unsafe { ext::mem_words::all_mut() };
        for (plc_word, modbus_word) in mem_words_mut.into_iter().zip(ctx.holdings.iter_mut()) {
            *modbus_word = *plc_word;
        }
        // also synchronize inputs and outputs to discretes
        let inputs_mut = unsafe { ext::inputs::all_mut() };
        for (plc_bit, modbus_bit) in inputs_mut.into_iter().zip(ctx.discretes.iter_mut().take(16)) {
            *modbus_bit = *plc_bit != 0x00;
        }
        let outputs_mut = unsafe { ext::outputs::all_mut() };
        for (plc_bit, modbus_bit) in outputs_mut.into_iter().zip(ctx.discretes.iter_mut().skip(16).take(16)) {
            *modbus_bit = *plc_bit != 0x00;
        }
    }

    pub fn sync_from_modbus(&mut self, ctx: &rmodbus::server::context::ModbusContext) {
        let mem_bits_mut = unsafe { ext::mem_bits::all_mut() };
        for (plc_bit, modbus_bit) in mem_bits_mut.into_iter().zip(ctx.coils.iter()) {
            *plc_bit = if *modbus_bit { 0xff } else { 0x00 };
        }
        let mem_words_mut = unsafe { ext::mem_words::all_mut() };
        for (plc_word, modbus_word) in mem_words_mut.into_iter().zip(ctx.holdings.iter()) {
            *plc_word = *modbus_word;
        }
    }
}
