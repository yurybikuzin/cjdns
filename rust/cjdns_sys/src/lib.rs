// This is needed to make sure the linker will pull in sodium here
pub fn init_sodium() {
    sodiumoxide::init().unwrap();
}

#[macro_export]
macro_rules! c_main {
    ($cmain:ident) => {
        use std::ffi::CString;
        use std::os::raw::c_char;
        use std::os::raw::c_int;
        extern "C" {
            fn $cmain(argc: c_int, argv: *const *mut c_char);
        }
        fn main() {
            cjdns_sys::init_sodium();
            let c_args = std::env::args()
                .map(|arg| CString::new(arg).unwrap())
                .map(|arg| arg.into_raw())
                .collect::<Vec<*mut c_char>>();
            unsafe { $cmain(c_args.len() as c_int, c_args.as_ptr()) }
        }
    };
}

/// All the C implementations are gathered under this `external` module.
#[macro_use]
pub mod external {
    pub mod interface {
        pub mod iface;
    }
    #[macro_use]
    pub mod memory {
        #[macro_use]
        pub mod allocator;
    }
    pub mod util {
        pub mod identity;
        pub mod log;
    }
    pub mod wire {
        pub mod error;
        pub mod message;
    }
}

mod interface {
    pub mod tuntap {
        pub mod android;
    }
}
