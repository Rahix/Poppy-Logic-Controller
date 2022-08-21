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
    ($N:expr, $($IO:ident,)*) => {
        paste::paste! {
            $(
                #[export_name = concat!("__", stringify!($IO))]
                static mut [<$IO _ADDR>]: *const u8 = unsafe { &$IO };
                pub static mut $IO: u8 = 0;
            )*

            pub unsafe fn all_mut<'a>() -> [&'a mut u8; $N] {
                [
                    $(&mut $IO,)*
                ]
            }
        }
    }
}

pub mod inputs {
    io_tags_def! {
        16,
        IX00, IX01, IX02, IX03, IX04, IX05, IX06, IX07,
        IX08, IX09, IX10, IX11, IX12, IX13, IX14, IX15,
    }
}

pub mod outputs {
    io_tags_def! {
        16,
        QX00, QX01, QX02, QX03, QX04, QX05, QX06, QX07,
        QX08, QX09, QX10, QX11, QX12, QX13, QX14, QX15,
    }
}
