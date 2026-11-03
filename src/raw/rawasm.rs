use core::ffi::c_void;

pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
pub type FARPROC_TYPE<A, T> = Option<unsafe extern "system" fn(A) -> T>;
pub type FARPTR = c_void;

pub unsafe fn fp_addr_as_func<F, R>(addr: FARPROC) -> Option<extern "system" fn(F) -> R> {
    unsafe {
        core::mem::transmute_copy(&addr)
    }
}

pub unsafe fn fpt_addr_as_func<F, R>(addr: FARPROC_TYPE<F, R>) -> Option<unsafe extern "system" fn(F) -> R> {
    unsafe {
        core::mem::transmute_copy(&addr)
    }
}

pub unsafe fn addr_as_fp_func<A, R>(addr: u64) -> Option<unsafe extern "system" fn (A) -> R> {
    if addr == 0 {
        None
    } else {
        // Cast the u64 address to a raw pointer, then transmute to the function pointer
        let raw_ptr = addr as *const ();
        Some(core::mem::transmute(raw_ptr))
    }
}

#[macro_export]
macro_rules! define_api {
        (
        $name:ident,
        extern "system" fn($($arg_name:ident: $arg_ty:ty),* $(,)?) -> $ret_ty:ty
        ) => {
            // 1. Generate the clean type alias for the concrete function pointer type
            pub type $name = extern "system" fn($($arg_name: $arg_ty),*) -> $ret_ty;

            // 2. Generate a dedicated helper function for safe, type-inferred casting
            // #[inline(always)]
            pub unsafe fn $name(addr: FARPROC) -> Option<$name> {
                if addr.is_none() {
                    None
                } else {
                    Some(core::mem::transmute_copy(&addr))
                }
            }
        };
        }

#[macro_export]
macro_rules! far_fn {
            (
                $name:ident,
                extern $abi:literal fn($($arg_name:ident: $arg_ty:ty),* $(,)?) -> $ret_ty:ty
            ) => {
                // 1. Generate the clean type alias for the concrete function pointer type
            pub type $name = extern $abi fn($($arg_name: $arg_ty),*) -> $ret_ty;

            // 2. Generate a dedicated helper function for safe, type-inferred casting
            // #[inline(always)]
            pub unsafe fn $name(addr: Option<$name>) -> Option<$name> {
                if addr.is_none() {
                    None
                } else {
                    Some(core::mem::transmute_copy(&addr))
                }
            }
            };
        }

#[macro_export]
macro_rules! far_fn_addr {
            (
                $name:ident,
                extern $abi:literal fn($($arg_name:ident: $arg_ty:ty),* $(,)?) -> $ret_ty:ty
            ) => {
                // 1. Generate the clean type alias for the concrete function pointer type
            pub type $name = extern $abi fn($($arg_name: $arg_ty),*) -> $ret_ty;

            // 2. Generate a dedicated helper function for safe, type-inferred casting
            // #[inline(always)]
            pub unsafe fn $name(addr: u64) -> Option<$name> {
                if addr/*.is_zero()*/ == 0 {
                    None
                } else {
                    Some(core::mem::transmute_copy(&addr))
                }
            }
            };
        }

#[macro_export]
macro_rules! close_fn {
            (
                $name:ident,
                fn($($arg_name:ident: $arg_ty:ty),* $(,)?) -> $ret_ty:ty
            ) => {
                pub type $name = fn($($arg_name: $arg_ty),*) -> $ret_ty;

            // 2. Generate a dedicated helper function for safe, type-inferred casting
            // #[inline(always)]
            pub unsafe fn $name(addr: Option<$name>) -> Option<$name> {
                if addr.is_none() {
                    None
                } else {
                    Some(core::mem::transmute_copy(&addr))
                }
            }
            };
        }

#[macro_export]
macro_rules! link_raw {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => {
        #[link(name=$library)]
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    };
}

