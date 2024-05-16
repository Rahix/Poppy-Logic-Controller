extern "C" {
    pub fn config_init__();
    pub fn config_run__(tick: cty::c_ulong);
    pub static common_ticktime__: cty::c_ulonglong;
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IecTimespec {
    pub tv_sec: cty::c_long,
    pub tv_nsec: cty::c_long,
}

#[export_name = "__CURRENT_TIME"]
pub static mut CURRENT_TIME: IecTimespec = IecTimespec {
    tv_sec: 0,
    tv_nsec: 0,
};

macro_rules! io_tags_def {
    ($N:expr, $Ty:ty, $($IO:ident,)*) => {
        paste::paste! {
            $(
                #[export_name = concat!("__", stringify!($IO))]
                static mut [<$IO _ADDR>]: *const $Ty = unsafe { ::core::ptr::addr_of!($IO) };
                pub static mut $IO: $Ty = 0;
            )*

            pub unsafe fn all_mut<'a>() -> [&'a mut $Ty; $N] {
                [
                    $(&mut *::core::ptr::addr_of_mut!($IO),)*
                ]
            }
        }
    }
}

pub mod inputs {
    io_tags_def! {
        16, u8,
        IX00, IX01, IX02, IX03, IX04, IX05, IX06, IX07,
        IX08, IX09, IX10, IX11, IX12, IX13, IX14, IX15,
    }
}

pub mod outputs {
    io_tags_def! {
        16, u8,
        QX00, QX01, QX02, QX03, QX04, QX05, QX06, QX07,
        QX08, QX09, QX10, QX11, QX12, QX13, QX14, QX15,
    }
}

pub mod mem_bits {
    io_tags_def! {
        16, u8,
        MX00, MX01, MX02, MX03, MX04, MX05, MX06, MX07,
        MX08, MX09, MX10, MX11, MX12, MX13, MX14, MX15,
    }
}

pub mod mem_words {
    io_tags_def! {
        16, u16,
        MW00, MW01, MW02, MW03, MW04, MW05, MW06, MW07,
        MW08, MW09, MW10, MW11, MW12, MW13, MW14, MW15,
    }
}
