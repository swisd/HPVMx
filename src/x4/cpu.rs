// #[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct M128A {
    pub Low: u64,
    pub High: i64,
}

// #[repr(C)]
#[derive(Clone, Copy)]
pub struct CONDITION_VARIABLE {
    pub Ptr: *mut u64,
}
impl Default for CONDITION_VARIABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}


pub type CONTEXT_FLAGS = u32;

// #[repr(C)]
#[derive(Clone, Copy)]
pub struct CONTEXT {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub P5Home: u64,
    pub P6Home: u64,
    pub ContextFlags: CONTEXT_FLAGS,
    pub MxCsr: u32,
    pub SegCs: u16,
    pub SegDs: u16,
    pub SegEs: u16,
    pub SegFs: u16,
    pub SegGs: u16,
    pub SegSs: u16,
    pub EFlags: u32,
    pub Dr0: u64,
    pub Dr1: u64,
    pub Dr2: u64,
    pub Dr3: u64,
    pub Dr6: u64,
    pub Dr7: u64,
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub Rsp: u64,
    pub Rbp: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub Rip: u64,
    pub Anonymous: CONTEXT_0,
    pub VectorRegister: [M128A; 26],
    pub VectorControl: u64,
    pub DebugControl: u64,
    pub LastBranchToRip: u64,
    pub LastBranchFromRip: u64,
    pub LastExceptionToRip: u64,
    pub LastExceptionFromRip: u64,
}

impl Default for CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
// #[repr(C)]
#[derive(Clone, Copy)]
pub union CONTEXT_0 {
    pub FltSave: XSAVE_FORMAT,
    pub Anonymous: CONTEXT_0_0,
}

impl Default for CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

// #[repr(C)]
#[derive(Clone, Copy)]
pub struct CONTEXT_0_0 {
    pub Header: [M128A; 2],
    pub Legacy: [M128A; 8],
    pub Xmm0: M128A,
    pub Xmm1: M128A,
    pub Xmm2: M128A,
    pub Xmm3: M128A,
    pub Xmm4: M128A,
    pub Xmm5: M128A,
    pub Xmm6: M128A,
    pub Xmm7: M128A,
    pub Xmm8: M128A,
    pub Xmm9: M128A,
    pub Xmm10: M128A,
    pub Xmm11: M128A,
    pub Xmm12: M128A,
    pub Xmm13: M128A,
    pub Xmm14: M128A,
    pub Xmm15: M128A,
}

impl Default for CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

// #[repr(C)]
#[derive(Clone, Copy)]
pub struct XSAVE_FORMAT {
    pub ControlWord: u16,
    pub StatusWord: u16,
    pub TagWord: u8,
    pub Reserved1: u8,
    pub ErrorOpcode: u16,
    pub ErrorOffset: u32,
    pub ErrorSelector: u16,
    pub Reserved2: u16,
    pub DataOffset: u32,
    pub DataSelector: u16,
    pub Reserved3: u16,
    pub MxCsr: u32,
    pub MxCsr_Mask: u32,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 16],
    pub Reserved4: [u8; 96],
}

impl Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}


pub type EXCEPTION_DISPOSITION = i32;
pub const EXCEPTION_MAXIMUM_PARAMETERS: u32 = 15u32;
// #[repr(C)]
#[derive(Clone, Copy)]
pub struct EXCEPTION_POINTERS {
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ContextRecord: *mut CONTEXT,
}
impl Default for EXCEPTION_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
// #[repr(C)]
#[derive(Clone, Copy)]
pub struct EXCEPTION_RECORD {
    pub ExceptionCode: STATUS,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ExceptionAddress: *mut u64,
    pub NumberParameters: u32,
    pub ExceptionInformation: [usize; 15],
}
impl Default for EXCEPTION_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXCEPTION_STACK_OVERFLOW: STATUS = 0xC00000FD_u32 as _;
pub const EXTENDED_STARTUPINFO_PRESENT: PROCESS_CREATION_FLAGS = 524288u32;
pub const E_NOTIMPL: RESULT = 0x80004001_u32 as _;
pub const EXCEPTION_COLLIDED_UNWIND: EXCEPTION_DISPOSITION = 3i32;
pub const EXCEPTION_CONTINUE_EXECUTION: EXCEPTION_DISPOSITION = 0i32;
pub const EXCEPTION_CONTINUE_SEARCH: EXCEPTION_DISPOSITION = 1i32;
pub const EXCEPTION_NESTED_EXCEPTION: EXCEPTION_DISPOSITION = 2i32;


pub type STATUS = i32;
pub type RESULT = i32;
pub type PROCESS_CREATION_FLAGS = u32;


pub mod misc {
    use super::{EXCEPTION_POINTERS, PROCESS_CREATION_FLAGS, STATUS};

    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct FLOATING_SAVE_AREA {
        pub ControlWord: u32,
        pub StatusWord: u32,
        pub TagWord: u32,
        pub ErrorOffset: u32,
        pub ErrorSelector: u32,
        pub DataOffset: u32,
        pub DataSelector: u32,
        pub RegisterArea: [u8; 80],
        pub Cr0NpxState: u32,
    }

    impl Default for FLOATING_SAVE_AREA {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct GUID {
        pub data1: u32,
        pub data2: u16,
        pub data3: u16,
        pub data4: [u8; 8],
    }
    impl GUID {
        pub const fn from_u128(uuid: u128) -> Self {
            Self {
                data1: (uuid >> 96) as u32,
                data2: (uuid >> 80 & 0xffff) as u16,
                data3: (uuid >> 64 & 0xffff) as u16,
                data4: (uuid as u64).to_be_bytes(),
            }
        }
    }
    pub type HANDLE = *mut u64;
    pub type HANDLE_FLAGS = u32;
    pub const HANDLE_FLAG_INHERIT: HANDLE_FLAGS = 1u32;
    pub const HANDLE_FLAG_PROTECT_FROM_CLOSE: HANDLE_FLAGS = 2u32;
    pub const HIGH_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 128u32;
    pub type HINSTANCE = *mut u64;
    pub type HLOCAL = *mut u64;
    pub type HMODULE = *mut u64;
    pub type HRESULT = i32;
    pub const IDLE_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 64u32;
    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct IN6_ADDR {
        pub u: IN6_ADDR_0,
    }
    impl Default for IN6_ADDR {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub union IN6_ADDR_0 {
        pub Byte: [u8; 16],
        pub Word: [u16; 8],
    }
    impl Default for IN6_ADDR_0 {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub union INIT_ONCE {
        pub Ptr: *mut u64,
    }
    impl Default for INIT_ONCE {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    pub const INIT_ONCE_INIT_FAILED: u32 = 4u32;


    pub type LPOVERLAPPED_COMPLETION_ROUTINE = Option<
        unsafe extern "system" fn(
            dwerrorcode: u32,
            dwnumberofbytestransfered: u32,
            lpoverlapped: *mut OVERLAPPED,
        ),
    >;
    pub type LPPROC_THREAD_ATTRIBUTE_LIST = *mut u64;
    pub type LPPROGRESS_ROUTINE = Option<
        unsafe extern "system" fn(
            totalfilesize: i64,
            totalbytestransferred: i64,
            streamsize: i64,
            streambytestransferred: i64,
            dwstreamnumber: u32,
            dwcallbackreason: LPPROGRESS_ROUTINE_CALLBACK_REASON,
            hsourcefile: HANDLE,
            hdestinationfile: HANDLE,
            lpdata: *const u64,
        ) -> COPYPROGRESSROUTINE_PROGRESS,
    >;
    pub type LPPROGRESS_ROUTINE_CALLBACK_REASON = u32;
    pub type LPTHREAD_START_ROUTINE =
    Option<unsafe extern "system" fn(lpthreadparameter: *mut u64) -> u32>;
    pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = Option<
        unsafe extern "system" fn(
            dwerror: u32,
            cbtransferred: u32,
            lpoverlapped: *mut OVERLAPPED,
            dwflags: u32,
        ),
    >;

    pub type COPYPROGRESSROUTINE_PROGRESS = u32;

    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct OVERLAPPED {
        pub Internal: usize,
        pub InternalHigh: usize,
        pub Anonymous: OVERLAPPED_0,
        pub hEvent: HANDLE,
    }
    impl Default for OVERLAPPED {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub union OVERLAPPED_0 {
        pub Anonymous: OVERLAPPED_0_0,
        pub Pointer: *mut u64,
    }
    impl Default for OVERLAPPED_0 {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    // #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub struct OVERLAPPED_0_0 {
        pub Offset: u32,
        pub OffsetHigh: u32,
    }
    pub type PCSTR = *const u8;
    pub type PCWSTR = *const u16;
    pub type PIO_APC_ROUTINE = Option<
        unsafe extern "system" fn(
            apccontext: *mut u64,
            iostatusblock: *mut IO_STATUS_BLOCK,
            reserved: u32,
        ),
    >;

    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct IO_STATUS_BLOCK {
        pub Anonymous: IO_STATUS_BLOCK_0,
        pub Information: usize,
    }
    impl Default for IO_STATUS_BLOCK {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub union IO_STATUS_BLOCK_0 {
        pub Status: STATUS,
        pub Pointer: *mut u64,
    }
    impl Default for IO_STATUS_BLOCK_0 {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    pub const PIPE_ACCEPT_REMOTE_CLIENTS: NAMED_PIPE_MODE = 0u32;
    pub const PIPE_ACCESS_DUPLEX: FILE_FLAGS_AND_ATTRIBUTES = 3u32;
    pub const PIPE_ACCESS_INBOUND: FILE_FLAGS_AND_ATTRIBUTES = 1u32;
    pub const PIPE_ACCESS_OUTBOUND: FILE_FLAGS_AND_ATTRIBUTES = 2u32;
    pub const PIPE_CLIENT_END: NAMED_PIPE_MODE = 0u32;
    pub const PIPE_NOWAIT: NAMED_PIPE_MODE = 1u32;
    pub const PIPE_READMODE_BYTE: NAMED_PIPE_MODE = 0u32;
    pub const PIPE_READMODE_MESSAGE: NAMED_PIPE_MODE = 2u32;
    pub const PIPE_REJECT_REMOTE_CLIENTS: NAMED_PIPE_MODE = 8u32;
    pub const PIPE_SERVER_END: NAMED_PIPE_MODE = 1u32;
    pub const PIPE_TYPE_BYTE: NAMED_PIPE_MODE = 0u32;
    pub const PIPE_TYPE_MESSAGE: NAMED_PIPE_MODE = 4u32;
    pub const PIPE_WAIT: NAMED_PIPE_MODE = 0u32;
    pub type PRIORITY_HINT = i32;
    pub type PROCESSOR_ARCHITECTURE = u16;
    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct PROCESS_INFORMATION {
        pub hProcess: HANDLE,
        pub hThread: HANDLE,
        pub dwProcessId: u32,
        pub dwThreadId: u32,
    }
    impl Default for PROCESS_INFORMATION {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    pub type NAMED_PIPE_MODE = u32;
    pub type FILE_FLAGS_AND_ATTRIBUTES = u32;

    pub type PSID = *mut u64;
    pub type PSTR = *mut u8;
    pub type PTIMERAPCROUTINE = Option<
        unsafe extern "system" fn(
            lpargtocompletionroutine: *const u64,
            dwtimerlowvalue: u32,
            dwtimerhighvalue: u32,
        ),
    >;
    pub type PVECTORED_EXCEPTION_HANDLER =
    Option<unsafe extern "system" fn(exceptioninfo: *mut EXCEPTION_POINTERS) -> i32>;
    pub type PWSTR = *mut u16;

    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct SECURITY_ATTRIBUTES {
        pub nLength: u32,
        pub lpSecurityDescriptor: *mut u64,
        pub bInheritHandle: BOOL,
    }
    impl Default for SECURITY_ATTRIBUTES {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    type BOOL = i32;

    // #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct SECURITY_DESCRIPTOR {
        pub Revision: u8,
        pub Sbz1: u8,
        pub Control: SECURITY_DESCRIPTOR_CONTROL,
        pub Owner: PSID,
        pub Group: PSID,
        pub Sacl: *mut ACL,
        pub Dacl: *mut ACL,
    }
    impl Default for SECURITY_DESCRIPTOR {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    pub type SECURITY_DESCRIPTOR_CONTROL = u16;
    pub type SECURITY_IMPERSONATION_LEVEL = i32;
    // #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub struct SECURITY_QUALITY_OF_SERVICE {
        pub Length: u32,
        pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
        pub ContextTrackingMode: u8,
        pub EffectiveOnly: bool,
    }

    // #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub struct ACL {
        pub AclRevision: u8,
        pub Sbz1: u8,
        pub AclSize: u16,
        pub AceCount: u16,
        pub Sbz2: u16,
    }
}

// #[repr(C)]
#[derive(Clone, Copy)]
pub struct SRWLOCK {
    pub Ptr: *mut u64,
}
impl Default for SRWLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

// #[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TIMEVAL {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
