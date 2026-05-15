use core::arch::asm;

macro_rules! HALT_AND_DEBUG {
    () => {
        unsafe {
            asm!("int3");
            asm!("ud2");
        }
    };
}


// sizeof=0x10
#[repr(C)]
struct _GUID {
    Data1: u32,       // 4 byte
    Data2: u16,       // 2 byte
    Data3: u16,       // 2 byte
    Data4: [u8; 8],   // 8 byte
}



static mut __SECURITY_COOKIE: usize = 0x4850564D78;

unsafe fn __security_check_cookie(StackCookie: usize) {
    if StackCookie != __SECURITY_COOKIE {
        HALT_AND_DEBUG!();
    }
}

type HALF_PTR = i16;


type HANDLE = *mut core::ffi::c_void;
type HANDLE64 = *mut core::ffi::c_void;
type HANDLE_PTR = u32;


type ULONG_PTR = u32;
type SIZE_T = ULONG_PTR;
type LPVOID = *mut u8;
type _DWORD = u32;
type _WORD = u16;

#[repr(u32)]
enum HANDLE_FLAGS {
    HANDLE_FLAG_UNK = 0x0,
}

unsafe fn ThreadSafeAddRef(pointer: *mut i32) -> i32 {
    let value: i32; // eax

    value = core::sync::atomic::AtomicI32::fetch_add(&mut *(pointer.offset(1) as *mut core::sync::atomic::AtomicI32), 1, core::sync::atomic::Ordering::SeqCst);
    if value <= 0 {
        breakpoint();
        BUG();
    }
    if value == 0x7FFFFFFF {
        breakpoint();
        BUG();
    }
    1
}


fn breakpoint(){}
fn BUG(){}



pub unsafe fn validate_and_execute(p_callback: i32, p_data: *const u32, p_out_status: *mut u32) -> i32 {
    *p_out_status = 0;

    let data = core::slice::from_raw_parts(p_data, 4);

    const SIG_A: [u32; 4] = [0, 0, 0x000000C0, 0x46000000];
    const SIG_B: [u32; 4] = [0x4850564D, 0x582D5349, 0x474E2D76, 0x322E3078];


    if data == SIG_A || data == SIG_B {
        *p_out_status = p_callback as u32;


        let vtable = *(p_callback as *const *const fn());
        let callback_fn = *vtable.add(1);

        callback_fn();
        return 0;
    }

    -255
}