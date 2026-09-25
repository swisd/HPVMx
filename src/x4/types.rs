#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes_definitions)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::c_void;
use crate::x4::ops::{__hex, _DWORD};


/// Opaque forward declaration from the IDA type database: `struct IManagedObjectVtbl`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IManagedObjectVtbl { _private: [u8; 0] }

/// Opaque forward declaration from the IDA type database: `struct IStream`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IStream { _private: [u8; 0] }

/// Opaque forward declaration from the IDA type database: `struct SubsystemContext`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubsystemContext { _private: [u8; 0] }

/// Opaque forward declaration from the IDA type database: `struct _TP_TIMER`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _TP_TIMER { _private: [u8; 0] }

pub type GUID = _GUID;
pub type ULONG = u32;
pub type EVENT_DESCRIPTOR = _EVENT_DESCRIPTOR;
pub type USHORT = u16;
pub type UCHAR = u8;
pub type ULONGLONG = u64;
pub type PEVENT_DATA_DESCRIPTOR = *mut _EVENT_DATA_DESCRIPTOR;
pub type PEVENT_FILTER_DESCRIPTOR = *mut _EVENT_FILTER_DESCRIPTOR;
pub type PVOID = *mut c_void;
pub type HMODULE = HINSTANCE;
pub type HINSTANCE = *mut HINSTANCE__;
pub type PRTL_CRITICAL_SECTION_DEBUG = *mut _RTL_CRITICAL_SECTION_DEBUG;
pub type WORD = u16;
pub type LIST_ENTRY = _LIST_ENTRY;
pub type DWORD = u32;
pub type LONG = i32;
pub type HANDLE = *mut c_void;
pub type ULONG_PTR = u64;
pub type CONTEXT = _CONTEXT;
pub type DWORD64 = u64;
pub type XMM_SAVE_AREA32 = _XMM_SAVE_AREA32;
pub type BYTE = u8;
pub type M128A = _M128A;
pub type LONGLONG = i64;
pub type ULONG64 = u64;
pub type PRUNTIME_FUNCTION = *mut _RUNTIME_FUNCTION;
pub type PCONTEXT = *mut CONTEXT;
pub type PULONG64 = *mut u64;
pub type PKNONVOLATILE_CONTEXT_POINTERS = *mut _KNONVOLATILE_CONTEXT_POINTERS;
pub type PM128A = *mut _M128A;
pub type RTL_SRWLOCK = _RTL_SRWLOCK;
pub type RTL_CONDITION_VARIABLE = _RTL_CONDITION_VARIABLE;
pub type LARGE_INTEGER = _LARGE_INTEGER;
pub type WCHAR = u16;
pub type wchar_t = u16;
pub type size_t = u64;
pub type LPWSTR = *mut WCHAR;
pub type CHAR = i8;
pub type LPCWSTR = *const WCHAR;
pub type PTP_TIMER = *mut _TP_TIMER;
pub type TP_TIMER = _TP_TIMER;
pub type LPVOID = *mut c_void;
pub type SIZE_T = ULONG_PTR;
pub type BOOL = i32;
pub type PULONG = *mut ULONG;
pub type HLOCAL = HANDLE;
pub type PSID = PVOID;
pub type WINBOOL = i32;
pub type REGSAM = ACCESS_MASK;
pub type ACCESS_MASK = DWORD;
pub type PHKEY = *mut HKEY;
pub type HKEY = *mut HKEY__;
pub type LPDWORD = *mut DWORD;
pub type PSECURITY_DESCRIPTOR = PVOID;
pub type LPSECURITY_ATTRIBUTES = *mut _SECURITY_ATTRIBUTES;
pub type DWORD_PTR = ULONG_PTR;
pub type BCRYPT_HASH_HANDLE = PVOID;
pub type RPC_WSTR = *mut u16;
pub type UUID = GUID;
pub type LPBYTE = *mut BYTE;
pub type SERVICE_TABLE_ENTRYW = _SERVICE_TABLE_ENTRYW;
pub type LPSERVICE_MAIN_FUNCTIONW = Option<unsafe extern "system" fn(DWORD, *mut LPWSTR) -> ()>;
pub type FILETIME = _FILETIME;
pub type PWSTR = *mut WCHAR;
pub type LPSTREAM = *mut IStream;
pub type IID = GUID;
pub type BCRYPT_KEY_HANDLE = PVOID;
pub type PCCERT_CONTEXT = *const CERT_CONTEXT;
pub type CERT_CONTEXT = _CERT_CONTEXT;
pub type PCERT_INFO = *mut _CERT_INFO;
pub type CRYPT_INTEGER_BLOB = _CRYPTOAPI_BLOB;
pub type CRYPT_ALGORITHM_IDENTIFIER = _CRYPT_ALGORITHM_IDENTIFIER;
pub type LPSTR = *mut CHAR;
pub type CRYPT_OBJID_BLOB = _CRYPTOAPI_BLOB;
pub type CERT_NAME_BLOB = _CRYPTOAPI_BLOB;
pub type CERT_PUBLIC_KEY_INFO = _CERT_PUBLIC_KEY_INFO;
pub type CRYPT_BIT_BLOB = _CRYPT_BIT_BLOB;
pub type PCERT_EXTENSION = *mut _CERT_EXTENSION;
pub type HCERTSTORE = *mut c_void;
pub type CRYPT_XML_BLOB = _CRYPT_XML_BLOB;
pub type CRYPT_XML_CHARSET = __E10363D4F72D146A35BB0350A9DCEB2A;
pub type CRYPT_XML_PROPERTY = _CRYPT_XML_PROPERTY;
pub type CRYPT_XML_PROPERTY_ID = __8F102EBC6F459E2853E0ABE6431CB284;
pub type HCRYPTXML = *mut c_void;
pub type CRYPT_XML_STATUS = _CRYPT_XML_STATUS;
pub type LPOVERLAPPED = *mut _OVERLAPPED;
pub type LPCCH = *const CHAR;
pub type LPCVOID = *const c_void;
pub type PDWORD = *mut DWORD;
pub type PFILETIME = *mut _FILETIME;
pub type VARIANTARG = VARIANT;
pub type VARIANT = tagVARIANT;
pub type VARTYPE = u16;
pub type SHORT = i16;
pub type FLOAT = f32;
pub type DOUBLE = f64;
pub type VARIANT_BOOL = i16;
pub type SCODE = LONG;
pub type CY = tagCY;
pub type DATE = f64;
pub type BSTR = *mut OLECHAR;
pub type OLECHAR = WCHAR;
pub type HRESULT = LONG;
pub type UINT = u32;
pub type LCID = DWORD;
pub type TYPEATTR = tagTYPEATTR;
pub type MEMBERID = DISPID;
pub type DISPID = LONG;
pub type LPOLESTR = *mut OLECHAR;
pub type TYPEKIND = tagTYPEKIND;
pub type TYPEDESC = tagTYPEDESC;
pub type SAFEARRAYBOUND = tagSAFEARRAYBOUND;
pub type HREFTYPE = DWORD;
pub type IDLDESC = tagIDLDESC;
pub type DESCKIND = tagDESCKIND;
pub type BINDPTR = tagBINDPTR;
pub type FUNCDESC = tagFUNCDESC;
pub type ELEMDESC = tagELEMDESC;
pub type PARAMDESC = tagPARAMDESC;
pub type LPPARAMDESCEX = *mut tagPARAMDESCEX;
pub type FUNCKIND = tagFUNCKIND;
pub type INVOKEKIND = tagINVOKEKIND;
pub type CALLCONV = tagCALLCONV;
pub type VARDESC = tagVARDESC;
pub type VARKIND = tagVARKIND;
pub type INT = i32;
pub type DISPPARAMS = tagDISPPARAMS;
pub type EXCEPINFO = tagEXCEPINFO;
pub type TLIBATTR = tagTLIBATTR;
pub type SYSKIND = tagSYSKIND;
pub type SAFEARRAY = tagSAFEARRAY;
pub type DECIMAL = tagDEC;
pub type LPCOLESTR = *const OLECHAR;
pub type SYSTEMTIME = _SYSTEMTIME;
pub type PLSA_UNICODE_STRING = *mut _LSA_UNICODE_STRING;
pub type SERVICE_NOTIFY_2W = _SERVICE_NOTIFY_2W;
pub type PFN_SC_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(PVOID) -> ()>;
pub type SERVICE_STATUS_PROCESS = _SERVICE_STATUS_PROCESS;
pub type LPCRITICAL_SECTION = PRTL_CRITICAL_SECTION;
pub type PRTL_CRITICAL_SECTION = *mut _RTL_CRITICAL_SECTION;
pub type BCRYPT_ALG_HANDLE = PVOID;
pub type LPBOOL = *mut WINBOOL;
pub type HCRYPTPROV = ULONG_PTR;
pub type HCRYPTHASH = ULONG_PTR;
pub type HCRYPTKEY = ULONG_PTR;
pub type PEXCEPTION_RECORD = *mut _EXCEPTION_RECORD;
pub type LPCWCH = *const WCHAR;
pub type STRSAFE_LPWSTR = *mut u16;
pub type uint32_t = u32;
pub type uint8_t = u8;
pub type uint64_t = u64;
pub type CryptoContext = _CRYPTO_CONTEXT;
pub type IMAGE_DOS_HEADER = _IMAGE_DOS_HEADER;
pub type PIMAGE_DOS_HEADER = *mut _IMAGE_DOS_HEADER;
pub type PENABLECALLBACK = Option<unsafe extern "system" fn(LPCGUID, ULONG, UCHAR, ULONGLONG, ULONGLONG, PEVENT_FILTER_DESCRIPTOR, PVOID) -> ()>;
pub type LPCGUID = *const GUID;
pub type CUSTOM_TRACE_CONTEXT = _CUSTOM_TRACE_CONTEXT;
pub type PCUSTOM_TRACE_CONTEXT = *mut _CUSTOM_TRACE_CONTEXT;
pub type TelemetryEvent = _TelemetryEvent;
pub type PTelemetryEvent = *mut _TelemetryEvent;
pub type SRWLOCK = RTL_SRWLOCK;
pub type TelemetryManager = _TelemetryManager;
pub type PTelemetryManager = *mut _TelemetryManager;
pub type STRSAFE_LPCWSTR = *const u16;
pub type LaneSnapshotData = LaneFrameTracker;
pub type CRITICAL_SECTION = RTL_CRITICAL_SECTION;
pub type RTL_CRITICAL_SECTION = _RTL_CRITICAL_SECTION;
pub type int32_t = i32;
pub type uint16_t = u16;
pub type SPP_STRING_BUFFER = _SPP_STRING_BUFFER;
pub type PSPP_STRING_BUFFER = *mut _SPP_STRING_BUFFER;
pub type ISppNamespaceVtbl = _ISppNamespaceVtbl;
pub type ISppCollectionVtbl = _ISppCollectionVtbl;
pub type SPP_VECTOR = _SPP_VECTOR;
pub type PSPP_VECTOR = *mut _SPP_VECTOR;
pub type PSRWLOCK = *mut RTL_SRWLOCK;
pub type __uint8 = u8;
pub type SppPacketControlFlags = [_DWORD; 2];
pub type __int64 = i64;
pub type unsigned_int64 = u64;

pub struct FileTime {
    pub(crate) dw_low_date_time: u32,
    pub(crate) dw_high_date_time: u32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _GUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
}


impl _GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            Data1: (uuid >> 96) as u32,
            Data2: (uuid >> 80 & 0xffff) as u16,
            Data3: (uuid >> 64 & 0xffff) as u16,
            Data4: (uuid as u64).to_be_bytes(),
        }
    }
    pub fn default() -> Self {
        Self {
            Data1: 0,
            Data2: 0,
            Data3: 0,
            Data4: [0; 8],
        }
    }
    pub(crate) fn Data1_mut(&self) -> u32 {
        //self.Data1.clone()
        //&mut 0
        self.Data1.clone()
    }
    pub(crate) fn Data1_ptr(&self) -> *mut *mut u8 {
        self.Data1.clone() as *mut *mut u8
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RUNTIME_FUNCTION {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UNWIND_INFO_HDR {
    pub Version: u8, // C bitfield: 3 bits
    pub Flags: u8, // C bitfield: 5 bits
    pub PrologSize: __hex!(__uint8),
    pub CntUnwindCodes: __hex!(__uint8),
    pub FrameRegister: u8, // C bitfield: 4 bits
    pub FrameOffset: u8, // C bitfield: 4 bits
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UNWIND_CODE {
    pub PrologOff: i8,
    pub UnwindOp: u8, // C bitfield: 4 bits
    pub OpInfo: u8, // C bitfield: 4 bits
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _EVENT_DESCRIPTOR {
    pub Id: USHORT,
    pub Version: UCHAR,
    pub Channel: UCHAR,
    pub Level: UCHAR,
    pub Opcode: UCHAR,
    pub Task: USHORT,
    pub Keyword: ULONGLONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _EVENT_DATA_DESCRIPTOR {
    pub Ptr: ULONGLONG,
    pub Size: ULONG,
    /// Anonymous C member #1: `union _EVENT_DATA_DESCRIPTOR::$535316677C6A15A6ECBA40D88E1D787B`
    pub anonymous_1: __EVENT_DATA_DESCRIPTOR___535316677C6A15A6ECBA40D88E1D787B,
}

/// Original IDA name: `_EVENT_DATA_DESCRIPTOR::$535316677C6A15A6ECBA40D88E1D787B`
#[repr(C)]
#[derive(Copy, Clone)]
pub union __EVENT_DATA_DESCRIPTOR___535316677C6A15A6ECBA40D88E1D787B {
    pub Reserved: ULONG,
    /// Anonymous C member #1: `struct _EVENT_DATA_DESCRIPTOR::$535316677C6A15A6ECBA40D88E1D787B::$AE424CE1AA1E88F41D16EB4A36C24CA2`
    pub anonymous_1: __EVENT_DATA_DESCRIPTOR___535316677C6A15A6ECBA40D88E1D787B___AE424CE1AA1E88F41D16EB4A36C24CA2,
}

/// Original IDA name: `_EVENT_DATA_DESCRIPTOR::$535316677C6A15A6ECBA40D88E1D787B::$AE424CE1AA1E88F41D16EB4A36C24CA2`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __EVENT_DATA_DESCRIPTOR___535316677C6A15A6ECBA40D88E1D787B___AE424CE1AA1E88F41D16EB4A36C24CA2 {
    pub Type: UCHAR,
    pub Reserved1: UCHAR,
    pub Reserved2: USHORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _EVENT_FILTER_DESCRIPTOR {
    pub Ptr: ULONGLONG,
    pub Size: ULONG,
    pub Type: ULONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HINSTANCE__ {
    pub unused: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _RTL_CRITICAL_SECTION {
    pub DebugInfo: PRTL_CRITICAL_SECTION_DEBUG,
    pub LockCount: LONG,
    pub RecursionCount: LONG,
    pub OwningThread: HANDLE,
    pub LockSemaphore: HANDLE,
    pub SpinCount: ULONG_PTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _RTL_CRITICAL_SECTION_DEBUG {
    pub Type: WORD,
    pub CreatorBackTraceIndex: WORD,
    pub CriticalSection: *mut _RTL_CRITICAL_SECTION,
    pub ProcessLocksList: LIST_ENTRY,
    pub EntryCount: DWORD,
    pub ContentionCount: DWORD,
    pub Flags: DWORD,
    pub CreatorBackTraceIndexHigh: WORD,
    pub SpareWORD: WORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _LIST_ENTRY {
    pub Flink: *mut _LIST_ENTRY,
    pub Blink: *mut _LIST_ENTRY,
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct _CONTEXT {
    pub P1Home: DWORD64,
    pub P2Home: DWORD64,
    pub P3Home: DWORD64,
    pub P4Home: DWORD64,
    pub P5Home: DWORD64,
    pub P6Home: DWORD64,
    pub ContextFlags: DWORD,
    pub MxCsr: DWORD,
    pub SegCs: WORD,
    pub SegDs: WORD,
    pub SegEs: WORD,
    pub SegFs: WORD,
    pub SegGs: WORD,
    pub SegSs: WORD,
    pub EFlags: DWORD,
    pub Dr0: DWORD64,
    pub Dr1: DWORD64,
    pub Dr2: DWORD64,
    pub Dr3: DWORD64,
    pub Dr6: DWORD64,
    pub Dr7: DWORD64,
    pub Rax: DWORD64,
    pub Rcx: DWORD64,
    pub Rdx: DWORD64,
    pub Rbx: DWORD64,
    pub Rsp: DWORD64,
    pub Rbp: DWORD64,
    pub Rsi: DWORD64,
    pub Rdi: DWORD64,
    pub R8: DWORD64,
    pub R9: DWORD64,
    pub R10: DWORD64,
    pub R11: DWORD64,
    pub R12: DWORD64,
    pub R13: DWORD64,
    pub R14: DWORD64,
    pub R15: DWORD64,
    pub Rip: DWORD64,
    /// Anonymous C member #1: `union _CONTEXT::$D2ECA93702C646ACAFACD524BE9E8FEB`
    pub anonymous_1: __CONTEXT___D2ECA93702C646ACAFACD524BE9E8FEB,
    pub VectorRegister: [M128A; 26],
    pub VectorControl: DWORD64,
    pub DebugControl: DWORD64,
    pub LastBranchToRip: DWORD64,
    pub LastBranchFromRip: DWORD64,
    pub LastExceptionToRip: DWORD64,
    pub LastExceptionFromRip: DWORD64,
}

/// Original IDA name: `_CONTEXT::$D2ECA93702C646ACAFACD524BE9E8FEB`
#[repr(C)]
#[derive(Copy, Clone)]
pub union __CONTEXT___D2ECA93702C646ACAFACD524BE9E8FEB {
    pub FltSave: XMM_SAVE_AREA32,
    /// Anonymous C member #1: `struct _CONTEXT::$D2ECA93702C646ACAFACD524BE9E8FEB::$897D11C01F73F7E79A06B0B9ED9B9414`
    pub anonymous_1: __CONTEXT___D2ECA93702C646ACAFACD524BE9E8FEB___897D11C01F73F7E79A06B0B9ED9B9414,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _XMM_SAVE_AREA32 {
    pub ControlWord: WORD,
    pub StatusWord: WORD,
    pub TagWord: BYTE,
    pub Reserved1: BYTE,
    pub ErrorOpcode: WORD,
    pub ErrorOffset: DWORD,
    pub ErrorSelector: WORD,
    pub Reserved2: WORD,
    pub DataOffset: DWORD,
    pub DataSelector: WORD,
    pub Reserved3: WORD,
    pub MxCsr: DWORD,
    pub MxCsr_Mask: DWORD,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 16],
    pub Reserved4: [BYTE; 96],
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct _M128A {
    pub Low: ULONGLONG,
    pub High: LONGLONG,
}

/// Original IDA name: `_CONTEXT::$D2ECA93702C646ACAFACD524BE9E8FEB::$897D11C01F73F7E79A06B0B9ED9B9414`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CONTEXT___D2ECA93702C646ACAFACD524BE9E8FEB___897D11C01F73F7E79A06B0B9ED9B9414 {
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _RUNTIME_FUNCTION {
    pub BeginAddress: DWORD,
    pub EndAddress: DWORD,
    pub UnwindData: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _KNONVOLATILE_CONTEXT_POINTERS {
    /// Anonymous C member #1: `union _KNONVOLATILE_CONTEXT_POINTERS::$D197E968A5E3A0101B09542845643980`
    pub anonymous_1: __KNONVOLATILE_CONTEXT_POINTERS___D197E968A5E3A0101B09542845643980,
    /// Anonymous C member #2: `union _KNONVOLATILE_CONTEXT_POINTERS::$9CB3C26C1E47FEC6AD623DA81D0AFEB5`
    pub anonymous_2: __KNONVOLATILE_CONTEXT_POINTERS___9CB3C26C1E47FEC6AD623DA81D0AFEB5,
}

/// Original IDA name: `_KNONVOLATILE_CONTEXT_POINTERS::$D197E968A5E3A0101B09542845643980`
#[repr(C)]
#[derive(Copy, Clone)]
pub union __KNONVOLATILE_CONTEXT_POINTERS___D197E968A5E3A0101B09542845643980 {
    pub FloatingContext: [PM128A; 16],
    /// Anonymous C member #1: `struct _KNONVOLATILE_CONTEXT_POINTERS::$D197E968A5E3A0101B09542845643980::$7E9A1EFC2FF768996EE7B23554CF4D70`
    pub anonymous_1: __KNONVOLATILE_CONTEXT_POINTERS___D197E968A5E3A0101B09542845643980___7E9A1EFC2FF768996EE7B23554CF4D70,
}

/// Original IDA name: `_KNONVOLATILE_CONTEXT_POINTERS::$D197E968A5E3A0101B09542845643980::$7E9A1EFC2FF768996EE7B23554CF4D70`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __KNONVOLATILE_CONTEXT_POINTERS___D197E968A5E3A0101B09542845643980___7E9A1EFC2FF768996EE7B23554CF4D70 {
    pub Xmm0: PM128A,
    pub Xmm1: PM128A,
    pub Xmm2: PM128A,
    pub Xmm3: PM128A,
    pub Xmm4: PM128A,
    pub Xmm5: PM128A,
    pub Xmm6: PM128A,
    pub Xmm7: PM128A,
    pub Xmm8: PM128A,
    pub Xmm9: PM128A,
    pub Xmm10: PM128A,
    pub Xmm11: PM128A,
    pub Xmm12: PM128A,
    pub Xmm13: PM128A,
    pub Xmm14: PM128A,
    pub Xmm15: PM128A,
}

/// Original IDA name: `_KNONVOLATILE_CONTEXT_POINTERS::$9CB3C26C1E47FEC6AD623DA81D0AFEB5`
#[repr(C)]
#[derive(Copy, Clone)]
pub union __KNONVOLATILE_CONTEXT_POINTERS___9CB3C26C1E47FEC6AD623DA81D0AFEB5 {
    pub IntegerContext: [PULONG64; 16],
    /// Anonymous C member #1: `struct _KNONVOLATILE_CONTEXT_POINTERS::$9CB3C26C1E47FEC6AD623DA81D0AFEB5::$476876C01DE46A57019B4E47CE995324`
    pub anonymous_1: __KNONVOLATILE_CONTEXT_POINTERS___9CB3C26C1E47FEC6AD623DA81D0AFEB5___476876C01DE46A57019B4E47CE995324,
}

/// Original IDA name: `_KNONVOLATILE_CONTEXT_POINTERS::$9CB3C26C1E47FEC6AD623DA81D0AFEB5::$476876C01DE46A57019B4E47CE995324`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __KNONVOLATILE_CONTEXT_POINTERS___9CB3C26C1E47FEC6AD623DA81D0AFEB5___476876C01DE46A57019B4E47CE995324 {
    pub Rax: PULONG64,
    pub Rcx: PULONG64,
    pub Rdx: PULONG64,
    pub Rbx: PULONG64,
    pub Rsp: PULONG64,
    pub Rbp: PULONG64,
    pub Rsi: PULONG64,
    pub Rdi: PULONG64,
    pub R8: PULONG64,
    pub R9: PULONG64,
    pub R10: PULONG64,
    pub R11: PULONG64,
    pub R12: PULONG64,
    pub R13: PULONG64,
    pub R14: PULONG64,
    pub R15: PULONG64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _RTL_SRWLOCK {
    pub Ptr: PVOID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _RTL_CONDITION_VARIABLE {
    pub Ptr: PVOID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FILETIME {
    pub dwLowDateTime: DWORD,
    pub dwHighDateTime: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union _LARGE_INTEGER {
    /// Anonymous C member #1: `struct _LARGE_INTEGER::$837407842DC9087486FDFA5FEB63B74E`
    pub anonymous_1: __LARGE_INTEGER___837407842DC9087486FDFA5FEB63B74E,
    pub u: __LARGE_INTEGER___837407842DC9087486FDFA5FEB63B74E,
    pub QuadPart: LONGLONG,
}

/// Original IDA name: `_LARGE_INTEGER::$837407842DC9087486FDFA5FEB63B74E`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __LARGE_INTEGER___837407842DC9087486FDFA5FEB63B74E {
    pub LowPart: DWORD,
    pub HighPart: LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SID_IDENTIFIER_AUTHORITY {
    pub Value: [BYTE; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HKEY__ {
    pub unused: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SECURITY_ATTRIBUTES {
    pub nLength: DWORD,
    pub lpSecurityDescriptor: LPVOID,
    pub bInheritHandle: BOOL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SYSTEM_INFO {
    /// Anonymous C member #1: `union _SYSTEM_INFO::$A707B71C060B6D10F73A71917EA8473F`
    pub anonymous_1: __SYSTEM_INFO___A707B71C060B6D10F73A71917EA8473F,
    pub dwPageSize: DWORD,
    pub lpMinimumApplicationAddress: LPVOID,
    pub lpMaximumApplicationAddress: LPVOID,
    pub dwActiveProcessorMask: DWORD_PTR,
    pub dwNumberOfProcessors: DWORD,
    pub dwProcessorType: DWORD,
    pub dwAllocationGranularity: DWORD,
    pub wProcessorLevel: WORD,
    pub wProcessorRevision: WORD,
}

/// Original IDA name: `_SYSTEM_INFO::$A707B71C060B6D10F73A71917EA8473F`
#[repr(C)]
#[derive(Copy, Clone)]
pub union __SYSTEM_INFO___A707B71C060B6D10F73A71917EA8473F {
    pub dwOemId: DWORD,
    /// Anonymous C member #1: `struct _SYSTEM_INFO::$A707B71C060B6D10F73A71917EA8473F::$AA04DEB0C6383F89F13D312A174572A9`
    pub anonymous_1: __SYSTEM_INFO___A707B71C060B6D10F73A71917EA8473F___AA04DEB0C6383F89F13D312A174572A9,
}

/// Original IDA name: `_SYSTEM_INFO::$A707B71C060B6D10F73A71917EA8473F::$AA04DEB0C6383F89F13D312A174572A9`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __SYSTEM_INFO___A707B71C060B6D10F73A71917EA8473F___AA04DEB0C6383F89F13D312A174572A9 {
    pub wProcessorArchitecture: WORD,
    pub wReserved: WORD,
}



#[repr(C)]
#[derive(Copy, Clone)]
pub struct _OSVERSIONINFOW {
    pub dwOSVersionInfoSize: DWORD,
    pub dwMajorVersion: DWORD,
    pub dwMinorVersion: DWORD,
    pub dwBuildNumber: DWORD,
    pub dwPlatformId: DWORD,
    pub szCSDVersion: [WCHAR; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SERVICE_STATUS {
    pub dwServiceType: DWORD,
    pub dwCurrentState: DWORD,
    pub dwControlsAccepted: DWORD,
    pub dwWin32ExitCode: DWORD,
    pub dwServiceSpecificExitCode: DWORD,
    pub dwCheckPoint: DWORD,
    pub dwWaitHint: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SYSTEMTIME {
    pub wYear: WORD,
    pub wMonth: WORD,
    pub wDayOfWeek: WORD,
    pub wDay: WORD,
    pub wHour: WORD,
    pub wMinute: WORD,
    pub wSecond: WORD,
    pub wMilliseconds: WORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SERVICE_TABLE_ENTRYW {
    pub lpServiceName: LPWSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _UNICODE_STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: PWSTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _OSVERSIONINFOA {
    pub dwOSVersionInfoSize: DWORD,
    pub dwMajorVersion: DWORD,
    pub dwMinorVersion: DWORD,
    pub dwBuildNumber: DWORD,
    pub dwPlatformId: DWORD,
    pub szCSDVersion: [CHAR; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CERT_CONTEXT {
    pub dwCertEncodingType: DWORD,
    pub pbCertEncoded: *mut BYTE,
    pub cbCertEncoded: DWORD,
    pub pCertInfo: PCERT_INFO,
    pub hCertStore: HCERTSTORE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CERT_INFO {
    pub dwVersion: DWORD,
    pub SerialNumber: CRYPT_INTEGER_BLOB,
    pub SignatureAlgorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub Issuer: CERT_NAME_BLOB,
    pub NotBefore: FILETIME,
    pub NotAfter: FILETIME,
    pub Subject: CERT_NAME_BLOB,
    pub SubjectPublicKeyInfo: CERT_PUBLIC_KEY_INFO,
    pub IssuerUniqueId: CRYPT_BIT_BLOB,
    pub SubjectUniqueId: CRYPT_BIT_BLOB,
    pub cExtension: DWORD,
    pub rgExtension: PCERT_EXTENSION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CRYPTOAPI_BLOB {
    pub cbData: DWORD,
    pub pbData: *mut BYTE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CRYPT_ALGORITHM_IDENTIFIER {
    pub pszObjId: LPSTR,
    pub Parameters: CRYPT_OBJID_BLOB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CERT_PUBLIC_KEY_INFO {
    pub Algorithm: CRYPT_ALGORITHM_IDENTIFIER,
    pub PublicKey: CRYPT_BIT_BLOB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CRYPT_BIT_BLOB {
    pub cbData: DWORD,
    pub pbData: *mut BYTE,
    pub cUnusedBits: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CERT_EXTENSION {
    pub pszObjId: LPSTR,
    pub fCritical: BOOL,
    pub Value: CRYPT_OBJID_BLOB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CRYPT_XML_BLOB {
    pub dwCharset: CRYPT_XML_CHARSET,
    pub cbData: ULONG,
    pub pbData: *mut BYTE,
}

/// Original IDA name: `$E10363D4F72D146A35BB0350A9DCEB2A`
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum __E10363D4F72D146A35BB0350A9DCEB2A {
    CRYPT_XML_CHARSET_AUTO = 0x0,
    CRYPT_XML_CHARSET_UTF8 = 0x1,
    CRYPT_XML_CHARSET_UTF16LE = 0x2,
    CRYPT_XML_CHARSET_UTF16BE = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CRYPT_XML_PROPERTY {
    pub dwPropId: CRYPT_XML_PROPERTY_ID,
    pub pvValue: *const c_void,
    pub cbValue: ULONG,
}

/// Original IDA name: `$8F102EBC6F459E2853E0ABE6431CB284`
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum __8F102EBC6F459E2853E0ABE6431CB284 {
    CRYPT_XML_PROPERTY_MAX_HEAP_SIZE = 0x1,
    CRYPT_XML_PROPERTY_SIGNATURE_LOCATION = 0x2,
    CRYPT_XML_PROPERTY_MAX_SIGNATURES = 0x3,
    CRYPT_XML_PROPERTY_DOC_DECLARATION = 0x4,
    CRYPT_XML_PROPERTY_XML_OUTPUT_CHARSET = 0x5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CRYPT_XML_STATUS {
    pub cbSize: ULONG,
    pub dwErrorStatus: DWORD,
    pub dwInfoStatus: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _OVERLAPPED {
    pub Internal: ULONG_PTR,
    pub InternalHigh: ULONG_PTR,
    /// Anonymous C member #1: `union _OVERLAPPED::$742A73540840F318F86F9CEE3D494648`
    pub anonymous_1: __OVERLAPPED___742A73540840F318F86F9CEE3D494648,
    pub hEvent: HANDLE,
}

/// Original IDA name: `_OVERLAPPED::$742A73540840F318F86F9CEE3D494648`
#[repr(C)]
#[derive(Copy, Clone)]
pub union __OVERLAPPED___742A73540840F318F86F9CEE3D494648 {
    /// Anonymous C member #1: `struct _OVERLAPPED::$742A73540840F318F86F9CEE3D494648::$9BFE693EDA487769FDABADE5E43394F7`
    pub anonymous_1: __OVERLAPPED___742A73540840F318F86F9CEE3D494648___9BFE693EDA487769FDABADE5E43394F7,
    pub Pointer: PVOID,
}

/// Original IDA name: `_OVERLAPPED::$742A73540840F318F86F9CEE3D494648::$9BFE693EDA487769FDABADE5E43394F7`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __OVERLAPPED___742A73540840F318F86F9CEE3D494648___9BFE693EDA487769FDABADE5E43394F7 {
    pub Offset: DWORD,
    pub OffsetHigh: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagVARIANT {
    /// Anonymous C member #1: `union tagVARIANT::$E93DC971A089CC95F6C875332324C1E7`
    pub anonymous_1: _tagVARIANT___E93DC971A089CC95F6C875332324C1E7,
}

/// Original IDA name: `tagVARIANT::$E93DC971A089CC95F6C875332324C1E7`
#[repr(C)]
#[derive(Copy, Clone)]
pub union _tagVARIANT___E93DC971A089CC95F6C875332324C1E7 {
    /// Anonymous C member #1: `struct tagVARIANT::$E93DC971A089CC95F6C875332324C1E7::$65D68C826D16CA47CF95571D7BFCD657`
    pub anonymous_1: _tagVARIANT___E93DC971A089CC95F6C875332324C1E7___65D68C826D16CA47CF95571D7BFCD657,
    pub decVal: DECIMAL,
}

/// Original IDA name: `tagVARIANT::$E93DC971A089CC95F6C875332324C1E7::$65D68C826D16CA47CF95571D7BFCD657`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _tagVARIANT___E93DC971A089CC95F6C875332324C1E7___65D68C826D16CA47CF95571D7BFCD657 {
    pub vt: VARTYPE,
    pub wReserved1: WORD,
    pub wReserved2: WORD,
    pub wReserved3: WORD,
    /// Anonymous C member #1: `union tagVARIANT::$::$65D68C826D16CA47CF95571D7BFCD657::$E09503A454170B491AC1C4312CE36FE6`
    pub anonymous_1: _tagVARIANT______65D68C826D16CA47CF95571D7BFCD657___E09503A454170B491AC1C4312CE36FE6,
}

/// Original IDA name: `tagVARIANT::$::$65D68C826D16CA47CF95571D7BFCD657::$E09503A454170B491AC1C4312CE36FE6`
#[repr(C)]
#[derive(Copy, Clone)]
pub union _tagVARIANT______65D68C826D16CA47CF95571D7BFCD657___E09503A454170B491AC1C4312CE36FE6 {
    pub llVal: LONGLONG,
    pub lVal: LONG,
    pub bVal: BYTE,
    pub iVal: SHORT,
    pub fltVal: FLOAT,
    pub dblVal: DOUBLE,
    pub boolVal: VARIANT_BOOL,
    pub scode: SCODE,
    pub cyVal: CY,
    pub date: DATE,
    pub bstrVal: BSTR,
    pub punkVal: *mut IUnknown,
    pub pdispVal: *mut IDispatch,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut BYTE,
    pub piVal: *mut SHORT,
    pub plVal: *mut LONG,
    pub pllVal: *mut LONGLONG,
    pub pfltVal: *mut FLOAT,
    pub pdblVal: *mut DOUBLE,
    pub pboolVal: *mut VARIANT_BOOL,
    pub pscode: *mut SCODE,
    pub pcyVal: *mut CY,
    pub pdate: *mut DATE,
    pub pbstrVal: *mut BSTR,
    pub ppunkVal: *mut *mut IUnknown,
    pub ppdispVal: *mut *mut IDispatch,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: PVOID,
    pub cVal: CHAR,
    pub uiVal: USHORT,
    pub ulVal: ULONG,
    pub ullVal: ULONGLONG,
    pub intVal: INT,
    pub uintVal: UINT,
    pub pdecVal: *mut DECIMAL,
    pub pcVal: *mut CHAR,
    pub puiVal: *mut USHORT,
    pub pulVal: *mut ULONG,
    pub pullVal: *mut ULONGLONG,
    pub pintVal: *mut INT,
    pub puintVal: *mut UINT,
    /// Anonymous C member #1: `struct tagVARIANT::$::$::$E09503A454170B491AC1C4312CE36FE6::$0FDBD249F1AECD6A49409B6B82281578`
    pub anonymous_1: _tagVARIANT_________E09503A454170B491AC1C4312CE36FE6___0FDBD249F1AECD6A49409B6B82281578,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tagCY {
    /// Anonymous C member #1: `struct tagCY::$3EA7BC8C29B528C7CA1203FC489E132F`
    pub anonymous_1: _tagCY___3EA7BC8C29B528C7CA1203FC489E132F,
    pub int64: LONGLONG,
}

/// Original IDA name: `tagCY::$3EA7BC8C29B528C7CA1203FC489E132F`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _tagCY___3EA7BC8C29B528C7CA1203FC489E132F {
    pub Lo: ULONG,
    pub Hi: LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IUnknown {
    pub lpVtbl: *mut IUnknownVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IUnknownVtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(*mut IUnknown, *const IID, *mut *mut c_void) -> HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(*mut IUnknown) -> ULONG>,
    pub Release: Option<unsafe extern "system" fn(*mut IUnknown) -> ULONG>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IDispatch {
    pub lpVtbl: *mut IDispatchVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IDispatchVtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(*mut IDispatch, *const IID, *mut *mut c_void) -> HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(*mut IDispatch) -> ULONG>,
    pub Release: Option<unsafe extern "system" fn(*mut IDispatch) -> ULONG>,
    pub GetTypeInfoCount: Option<unsafe extern "system" fn(*mut IDispatch, *mut UINT) -> HRESULT>,
    pub GetTypeInfo: Option<unsafe extern "system" fn(*mut IDispatch, UINT, LCID, *mut *mut ITypeInfo) -> HRESULT>,
    pub GetIDsOfNames: Option<unsafe extern "system" fn(*mut IDispatch, *const IID, *mut LPOLESTR, UINT, LCID, *mut DISPID) -> HRESULT>,
    pub Invoke: Option<unsafe extern "system" fn(*mut IDispatch, DISPID, *const IID, LCID, WORD, *mut DISPPARAMS, *mut VARIANT, *mut EXCEPINFO, *mut UINT) -> HRESULT>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ITypeInfo {
    pub lpVtbl: *mut ITypeInfoVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ITypeInfoVtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(*mut ITypeInfo, *const IID, *mut *mut c_void) -> HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(*mut ITypeInfo) -> ULONG>,
    pub Release: Option<unsafe extern "system" fn(*mut ITypeInfo) -> ULONG>,
    pub GetTypeAttr: Option<unsafe extern "system" fn(*mut ITypeInfo, *mut *mut TYPEATTR) -> HRESULT>,
    pub GetTypeComp: Option<unsafe extern "system" fn(*mut ITypeInfo, *mut *mut ITypeComp) -> HRESULT>,
    pub GetFuncDesc: Option<unsafe extern "system" fn(*mut ITypeInfo, UINT, *mut *mut FUNCDESC) -> HRESULT>,
    pub GetVarDesc: Option<unsafe extern "system" fn(*mut ITypeInfo, UINT, *mut *mut VARDESC) -> HRESULT>,
    pub GetNames: Option<unsafe extern "system" fn(*mut ITypeInfo, MEMBERID, *mut BSTR, UINT, *mut UINT) -> HRESULT>,
    pub GetRefTypeOfImplType: Option<unsafe extern "system" fn(*mut ITypeInfo, UINT, *mut HREFTYPE) -> HRESULT>,
    pub GetImplTypeFlags: Option<unsafe extern "system" fn(*mut ITypeInfo, UINT, *mut INT) -> HRESULT>,
    pub GetIDsOfNames: Option<unsafe extern "system" fn(*mut ITypeInfo, *mut LPOLESTR, UINT, *mut MEMBERID) -> HRESULT>,
    pub Invoke: Option<unsafe extern "system" fn(*mut ITypeInfo, PVOID, MEMBERID, WORD, *mut DISPPARAMS, *mut VARIANT, *mut EXCEPINFO, *mut UINT) -> HRESULT>,
    pub GetDocumentation: Option<unsafe extern "system" fn(*mut ITypeInfo, MEMBERID, *mut BSTR, *mut BSTR, *mut DWORD, *mut BSTR) -> HRESULT>,
    pub GetDllEntry: Option<unsafe extern "system" fn(*mut ITypeInfo, MEMBERID, INVOKEKIND, *mut BSTR, *mut BSTR, *mut WORD) -> HRESULT>,
    pub GetRefTypeInfo: Option<unsafe extern "system" fn(*mut ITypeInfo, HREFTYPE, *mut *mut ITypeInfo) -> HRESULT>,
    pub AddressOfMember: Option<unsafe extern "system" fn(*mut ITypeInfo, MEMBERID, INVOKEKIND, *mut PVOID) -> HRESULT>,
    pub CreateInstance: Option<unsafe extern "system" fn(*mut ITypeInfo, *mut IUnknown, *const IID, *mut PVOID) -> HRESULT>,
    pub GetMops: Option<unsafe extern "system" fn(*mut ITypeInfo, MEMBERID, *mut BSTR) -> HRESULT>,
    pub GetContainingTypeLib: Option<unsafe extern "system" fn(*mut ITypeInfo, *mut *mut ITypeLib, *mut UINT) -> HRESULT>,
    pub ReleaseTypeAttr: Option<unsafe extern "system" fn(*mut ITypeInfo, *mut TYPEATTR) -> ()>,
    pub ReleaseFuncDesc: Option<unsafe extern "system" fn(*mut ITypeInfo, *mut FUNCDESC) -> ()>,
    pub ReleaseVarDesc: Option<unsafe extern "system" fn(*mut ITypeInfo, *mut VARDESC) -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagTYPEATTR {
    pub guid: GUID,
    pub lcid: LCID,
    pub dwReserved: DWORD,
    pub memidConstructor: MEMBERID,
    pub memidDestructor: MEMBERID,
    pub lpstrSchema: LPOLESTR,
    pub cbSizeInstance: ULONG,
    pub typekind: TYPEKIND,
    pub cFuncs: WORD,
    pub cVars: WORD,
    pub cImplTypes: WORD,
    pub cbSizeVft: WORD,
    pub cbAlignment: WORD,
    pub wTypeFlags: WORD,
    pub wMajorVerNum: WORD,
    pub wMinorVerNum: WORD,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tagTYPEKIND {
    TKIND_ENUM = 0x0,
    TKIND_RECORD = 0x1,
    TKIND_MODULE = 0x2,
    TKIND_INTERFACE = 0x3,
    TKIND_DISPATCH = 0x4,
    TKIND_COCLASS = 0x5,
    TKIND_ALIAS = 0x6,
    TKIND_UNION = 0x7,
    TKIND_MAX = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagTYPEDESC {
    /// Anonymous C member #1: `union tagTYPEDESC::$AC700B6542D8071E244CADABF8A32897`
    pub anonymous_1: _tagTYPEDESC___AC700B6542D8071E244CADABF8A32897,
    pub vt: VARTYPE,
}

/// Original IDA name: `tagTYPEDESC::$AC700B6542D8071E244CADABF8A32897`
#[repr(C)]
#[derive(Copy, Clone)]
pub union _tagTYPEDESC___AC700B6542D8071E244CADABF8A32897 {
    pub lptdesc: *mut tagTYPEDESC,
    pub lpadesc: *mut tagARRAYDESC,
    pub hreftype: HREFTYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagARRAYDESC {
    pub tdescElem: TYPEDESC,
    pub cDims: USHORT,
    pub rgbounds: [SAFEARRAYBOUND; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagSAFEARRAYBOUND {
    pub cElements: ULONG,
    pub lLbound: LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagIDLDESC {
    pub dwReserved: ULONG_PTR,
    pub wIDLFlags: USHORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ITypeComp {
    pub lpVtbl: *mut ITypeCompVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ITypeCompVtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(*mut ITypeComp, *const IID, *mut *mut c_void) -> HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(*mut ITypeComp) -> ULONG>,
    pub Release: Option<unsafe extern "system" fn(*mut ITypeComp) -> ULONG>,
    pub Bind: Option<unsafe extern "system" fn(*mut ITypeComp, LPOLESTR, ULONG, WORD, *mut *mut ITypeInfo, *mut DESCKIND, *mut BINDPTR) -> HRESULT>,
    pub BindType: Option<unsafe extern "system" fn(*mut ITypeComp, LPOLESTR, ULONG, *mut *mut ITypeInfo, *mut *mut ITypeComp) -> HRESULT>,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tagDESCKIND {
    DESCKIND_NONE = 0x0,
    DESCKIND_FUNCDESC = 0x1,
    DESCKIND_VARDESC = 0x2,
    DESCKIND_TYPECOMP = 0x3,
    DESCKIND_IMPLICITAPPOBJ = 0x4,
    DESCKIND_MAX = 0x5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tagBINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: *mut ITypeComp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagFUNCDESC {
    pub memid: MEMBERID,
    pub lprgscode: *mut SCODE,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: SHORT,
    pub cParamsOpt: SHORT,
    pub oVft: SHORT,
    pub cScodes: SHORT,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: WORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagELEMDESC {
    pub tdesc: TYPEDESC,
    /// Anonymous C member #1: `union tagELEMDESC::$7C8F4CED1424251743D09680A1A0B07D`
    pub anonymous_1: _tagELEMDESC___7C8F4CED1424251743D09680A1A0B07D,
}

/// Original IDA name: `tagELEMDESC::$7C8F4CED1424251743D09680A1A0B07D`
#[repr(C)]
#[derive(Copy, Clone)]
pub union _tagELEMDESC___7C8F4CED1424251743D09680A1A0B07D {
    pub idldesc: IDLDESC,
    pub paramdesc: PARAMDESC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagPARAMDESC {
    pub pparamdescex: LPPARAMDESCEX,
    pub wParamFlags: USHORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagPARAMDESCEX {
    pub cBytes: ULONG,
    pub varDefaultValue: VARIANTARG,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tagFUNCKIND {
    FUNC_VIRTUAL = 0x0,
    FUNC_PUREVIRTUAL = 0x1,
    FUNC_NONVIRTUAL = 0x2,
    FUNC_STATIC = 0x3,
    FUNC_DISPATCH = 0x4,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tagINVOKEKIND {
    INVOKE_FUNC = 0x1,
    INVOKE_PROPERTYGET = 0x2,
    INVOKE_PROPERTYPUT = 0x4,
    INVOKE_PROPERTYPUTREF = 0x8,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tagCALLCONV {
    CC_FASTCALL = 0x0,
    CC_CDECL = 0x1,
    CC_MSCPASCAL = 0x2,
    CC_MACPASCAL = 0x3,
    CC_coreCALL = 0x4,
    CC_FPFASTCALL = 0x5,
    CC_SYSCALL = 0x6,
    CC_MPWCDECL = 0x7,
    CC_MPWPASCAL = 0x8,
    CC_MAX = 0x9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagVARDESC {
    pub memid: MEMBERID,
    pub lpstrSchema: LPOLESTR,
    /// Anonymous C member #1: `union tagVARDESC::$E6274BD6A7149C9CC2413444FF769F0B`
    pub anonymous_1: _tagVARDESC___E6274BD6A7149C9CC2413444FF769F0B,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: WORD,
    pub varkind: VARKIND,
}

/// Original IDA name: `tagVARDESC::$E6274BD6A7149C9CC2413444FF769F0B`
#[repr(C)]
#[derive(Copy, Clone)]
pub union _tagVARDESC___E6274BD6A7149C9CC2413444FF769F0B {
    pub oInst: ULONG,
    pub lpvarValue: *mut VARIANT,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tagVARKIND {
    VAR_PERINSTANCE = 0x0,
    VAR_STATIC = 0x1,
    VAR_CONST = 0x2,
    VAR_DISPATCH = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagDISPPARAMS {
    pub rgvarg: *mut VARIANTARG,
    pub rgdispidNamedArgs: *mut DISPID,
    pub cArgs: UINT,
    pub cNamedArgs: UINT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagEXCEPINFO {
    pub wCode: WORD,
    pub wReserved: WORD,
    pub bstrSource: BSTR,
    pub bstrDescription: BSTR,
    pub bstrHelpFile: BSTR,
    pub dwHelpContext: DWORD,
    pub pvReserved: PVOID,
    pub pfnDeferredFillIn: Option<unsafe extern "system" fn(tag: tagEXCEPINFO) -> HRESULT>,
    pub scode: SCODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ITypeLib {
    pub lpVtbl: *mut ITypeLibVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ITypeLibVtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(*mut ITypeLib, *const IID, *mut *mut c_void) -> HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(*mut ITypeLib) -> ULONG>,
    pub Release: Option<unsafe extern "system" fn(*mut ITypeLib) -> ULONG>,
    pub GetTypeInfoCount: Option<unsafe extern "system" fn(*mut ITypeLib) -> UINT>,
    pub GetTypeInfo: Option<unsafe extern "system" fn(*mut ITypeLib, UINT, *mut *mut ITypeInfo) -> HRESULT>,
    pub GetTypeInfoType: Option<unsafe extern "system" fn(*mut ITypeLib, UINT, *mut TYPEKIND) -> HRESULT>,
    pub GetTypeInfoOfGuid: Option<unsafe extern "system" fn(*mut ITypeLib, *const GUID, *mut *mut ITypeInfo) -> HRESULT>,
    pub GetLibAttr: Option<unsafe extern "system" fn(*mut ITypeLib, *mut *mut TLIBATTR) -> HRESULT>,
    pub GetTypeComp: Option<unsafe extern "system" fn(*mut ITypeLib, *mut *mut ITypeComp) -> HRESULT>,
    pub GetDocumentation: Option<unsafe extern "system" fn(*mut ITypeLib, INT, *mut BSTR, *mut BSTR, *mut DWORD, *mut BSTR) -> HRESULT>,
    pub IsName: Option<unsafe extern "system" fn(*mut ITypeLib, LPOLESTR, ULONG, *mut BOOL) -> HRESULT>,
    pub FindName: Option<unsafe extern "system" fn(*mut ITypeLib, LPOLESTR, ULONG, *mut *mut ITypeInfo, *mut MEMBERID, *mut USHORT) -> HRESULT>,
    pub ReleaseTLibAttr: Option<unsafe extern "system" fn(*mut ITypeLib, *mut TLIBATTR) -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagTLIBATTR {
    pub guid: GUID,
    pub lcid: LCID,
    pub syskind: SYSKIND,
    pub wMajorVerNum: WORD,
    pub wMinorVerNum: WORD,
    pub wLibFlags: WORD,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tagSYSKIND {
    SYS_WIN16 = 0x0,
    SYS_WIN32 = 0x1,
    SYS_MAC = 0x2,
    SYS_WIN64 = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagSAFEARRAY {
    pub cDims: USHORT,
    pub fFeatures: USHORT,
    pub cbElements: ULONG,
    pub cLocks: ULONG,
    pub pvData: PVOID,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagDEC {
    pub wReserved: USHORT,
    /// Anonymous C member #1: `union tagDEC::$64EC678C49E7BE49873AFBFB7A849D34`
    pub anonymous_1: _tagDEC___64EC678C49E7BE49873AFBFB7A849D34,
    pub Hi32: ULONG,
    /// Anonymous C member #2: `union tagDEC::$D28E26DEC3EC762C06C2AA9D0F7AC301`
    pub anonymous_2: _tagDEC___D28E26DEC3EC762C06C2AA9D0F7AC301,
}

/// Original IDA name: `tagDEC::$64EC678C49E7BE49873AFBFB7A849D34`
#[repr(C)]
#[derive(Copy, Clone)]
pub union _tagDEC___64EC678C49E7BE49873AFBFB7A849D34 {
    /// Anonymous C member #1: `struct tagDEC::$64EC678C49E7BE49873AFBFB7A849D34::$7F8459940C2B08BD5D82B0F27239141B`
    pub anonymous_1: _tagDEC___64EC678C49E7BE49873AFBFB7A849D34___7F8459940C2B08BD5D82B0F27239141B,
    pub signscale: USHORT,
}

/// Original IDA name: `tagDEC::$64EC678C49E7BE49873AFBFB7A849D34::$7F8459940C2B08BD5D82B0F27239141B`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _tagDEC___64EC678C49E7BE49873AFBFB7A849D34___7F8459940C2B08BD5D82B0F27239141B {
    pub scale: BYTE,
    pub sign: BYTE,
}

/// Original IDA name: `tagDEC::$D28E26DEC3EC762C06C2AA9D0F7AC301`
#[repr(C)]
#[derive(Copy, Clone)]
pub union _tagDEC___D28E26DEC3EC762C06C2AA9D0F7AC301 {
    /// Anonymous C member #1: `struct tagDEC::$D28E26DEC3EC762C06C2AA9D0F7AC301::$674876891A86A76F12C10005982BCA56`
    pub anonymous_1: _tagDEC___D28E26DEC3EC762C06C2AA9D0F7AC301___674876891A86A76F12C10005982BCA56,
    pub Lo64: ULONGLONG,
}

/// Original IDA name: `tagDEC::$D28E26DEC3EC762C06C2AA9D0F7AC301::$674876891A86A76F12C10005982BCA56`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _tagDEC___D28E26DEC3EC762C06C2AA9D0F7AC301___674876891A86A76F12C10005982BCA56 {
    pub Lo32: ULONG,
    pub Mid32: ULONG,
}

/// Original IDA name: `tagVARIANT::$::$::$E09503A454170B491AC1C4312CE36FE6::$0FDBD249F1AECD6A49409B6B82281578`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _tagVARIANT_________E09503A454170B491AC1C4312CE36FE6___0FDBD249F1AECD6A49409B6B82281578 {
    pub pvRecord: PVOID,
    pub pRecInfo: *mut IRecordInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRecordInfo {
    pub lpVtbl: *mut IRecordInfoVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRecordInfoVtbl {
    pub QueryInterface: Option<unsafe extern "system" fn(*mut IRecordInfo, *const IID, *mut *mut c_void) -> HRESULT>,
    pub AddRef: Option<unsafe extern "system" fn(*mut IRecordInfo) -> ULONG>,
    pub Release: Option<unsafe extern "system" fn(*mut IRecordInfo) -> ULONG>,
    pub RecordInit: Option<unsafe extern "system" fn(*mut IRecordInfo, PVOID) -> HRESULT>,
    pub RecordClear: Option<unsafe extern "system" fn(*mut IRecordInfo, PVOID) -> HRESULT>,
    pub RecordCopy: Option<unsafe extern "system" fn(*mut IRecordInfo, PVOID, PVOID) -> HRESULT>,
    pub GetGuid: Option<unsafe extern "system" fn(*mut IRecordInfo, *mut GUID) -> HRESULT>,
    pub GetName: Option<unsafe extern "system" fn(*mut IRecordInfo, *mut BSTR) -> HRESULT>,
    pub GetSize: Option<unsafe extern "system" fn(*mut IRecordInfo, *mut ULONG) -> HRESULT>,
    pub GetTypeInfo: Option<unsafe extern "system" fn(*mut IRecordInfo, *mut *mut ITypeInfo) -> HRESULT>,
    pub GetField: Option<unsafe extern "system" fn(*mut IRecordInfo, PVOID, LPCOLESTR, *mut VARIANT) -> HRESULT>,
    pub GetFieldNoCopy: Option<unsafe extern "system" fn(*mut IRecordInfo, PVOID, LPCOLESTR, *mut VARIANT, *mut PVOID) -> HRESULT>,
    pub PutField: Option<unsafe extern "system" fn(*mut IRecordInfo, ULONG, PVOID, LPCOLESTR, *mut VARIANT) -> HRESULT>,
    pub PutFieldNoCopy: Option<unsafe extern "system" fn(*mut IRecordInfo, ULONG, PVOID, LPCOLESTR, *mut VARIANT) -> HRESULT>,
    pub GetFieldNames: Option<unsafe extern "system" fn(*mut IRecordInfo, *mut ULONG, *mut BSTR) -> HRESULT>,
    pub IsMatchingType: Option<unsafe extern "system" fn(*mut IRecordInfo, *mut IRecordInfo) -> BOOL>,
    pub RecordCreate: Option<unsafe extern "system" fn(*mut IRecordInfo) -> PVOID>,
    pub RecordCreateCopy: Option<unsafe extern "system" fn(*mut IRecordInfo, PVOID, *mut PVOID) -> HRESULT>,
    pub RecordDestroy: Option<unsafe extern "system" fn(*mut IRecordInfo, PVOID) -> HRESULT>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _LSA_OBJECT_ATTRIBUTES {
    pub Length: ULONG,
    pub RootDirectory: HANDLE,
    pub ObjectName: PLSA_UNICODE_STRING,
    pub Attributes: ULONG,
    pub SecurityDescriptor: PVOID,
    pub SecurityQualityOfService: PVOID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _LSA_UNICODE_STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: PWSTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SERVICE_NOTIFY_2W {
    pub dwVersion: DWORD,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: PVOID,
    pub dwNotificationStatus: DWORD,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: DWORD,
    pub pszServiceNames: LPWSTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SERVICE_STATUS_PROCESS {
    pub dwServiceType: DWORD,
    pub dwCurrentState: DWORD,
    pub dwControlsAccepted: DWORD,
    pub dwWin32ExitCode: DWORD,
    pub dwServiceSpecificExitCode: DWORD,
    pub dwCheckPoint: DWORD,
    pub dwWaitHint: DWORD,
    pub dwProcessId: DWORD,
    pub dwServiceFlags: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _MEMORY_BASIC_INFORMATION {
    pub BaseAddress: PVOID,
    pub AllocationBase: PVOID,
    pub AllocationProtect: DWORD,
    pub RegionSize: SIZE_T,
    pub State: DWORD,
    pub Protect: DWORD,
    pub Type: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C_SCOPE_TABLE {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EXCEPTION_POINTERS {
    pub ExceptionRecord: PEXCEPTION_RECORD,
    pub ContextRecord: PCONTEXT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _EXCEPTION_RECORD {
    pub ExceptionCode: DWORD,
    pub ExceptionFlags: DWORD,
    pub ExceptionRecord: *mut _EXCEPTION_RECORD,
    pub ExceptionAddress: PVOID,
    pub NumberParameters: DWORD,
    pub ExceptionInformation: [ULONG_PTR; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SharedContextStateSlot {
    pub version: i32,
    pub isActive: i8,
    pub padding0: i8,
    pub channelType: i16,
    pub subStatus: i8,
    pub padding1: i8,
    pub capacity: i64,
    pub reservedPool: [i64; 4],
    pub trailerId: i16,
    pub cleanupByte: i8,
    pub padding2: [i8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SharedContextBlock {
    pub lanes: [LaneFrameTracker; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrorStatusTuple {
    pub hresult: i32,
    pub ntStatus: i32,
    pub flags: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ItemTracker {
    pub state: i32,
    pub padding: i32,
    pub target_status_ptr: *mut i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ItemManager {
    pub is_active: i32,
    pub srw_lock: RTL_SRWLOCK,
    pub version: i32,
    pub items_start: *mut ItemTracker,
    pub items_end: *mut ItemTracker,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MontgomeryContext {
    pub padding: i32,
    pub base_count: i32,
    pub mont_inverse: u64,
    pub modulus: [u64; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CRYPTO_CONTEXT {
    pub flags: u32,
    pub num_blocks: u32,
    pub padding_08: [u8; 16],
    pub n_prime: u64,
    pub padding_20: [u8; 96],
    pub modulus: [u64; 8],
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub union __m128i {
    pub m128i_i8: [i8; 16],
    pub m128i_i16: [i16; 8],
    pub m128i_i32: [i32; 4],
    pub m128i_i64: [i64; 2],
    pub m128i_u8: [u8; 16],
    pub m128i_u16: [u16; 8],
    pub m128i_u32: [u32; 4],
    pub m128i_u64: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _IMAGE_DOS_HEADER {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CUSTOM_TRACE_CONTEXT {
    pub IsEnabledLevel: ULONG,
    pub Reserved: ULONG,
    pub MatchAnyKeyword: ULONGLONG,
    pub MatchAllKeyword: ULONGLONG,
    pub Reserved2: PVOID,
    pub UserCallback: PENABLECALLBACK,
    pub UserContext: PVOID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _TelemetryEvent {
    pub padding1: [u8; 16],
    pub pDataDescriptors: *mut _EVENT_DATA_DESCRIPTOR,
    pub pNextEvent: *mut _TelemetryEvent,
    pub padding2: [u8; 8],
    pub pCustomSubsystemLink: u64,
    pub descriptorCount: u8,
    pub customArgumentCount: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _TelemetryManager {
    pub totalPending: u32,
    pub currentWheelIndex: u32,
    pub bucketLock: SRWLOCK,
    pub stats_lifetimeFlushed: u64,
    pub stats_flushCycles: u64,
    pub padding3: [u8; 4],
    pub stats_maxBatchSize: u32,
    pub stats_minBatchSize: u32,
    pub padding4: [u8; 24],
    pub pSubsystemContext: *mut SubsystemContext,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBindingDescriptor {
    pub BindSlot: DWORD,
    pub ResourceCount: WORD,
    pub VisibilityFlags: WORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WnfMetricEntry {
    pub MetricId: DWORD,
    pub ComponentFlags: WORD,
    pub Padding: WORD,
    pub CounterValue: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VectorTracker {
    pub pStart: *mut WnfMetricEntry,
    pub pEnd: *mut WnfMetricEntry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VectorLayout {
    pub pBegin: *mut BYTE,
    pub pEnd: *mut BYTE,
    pub pCapacityEnd: *mut BYTE,
    pub pAllocation: *mut BYTE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderManager {
    pub status_flag: u32,
    pub padding_0: u32,
    pub lock: RTL_SRWLOCK,
    pub timer: PTP_TIMER,
    pub unk_config_flags: u64,
    pub vector_begin: *mut DirtyResourceEntry,
    pub vector_end: *mut DirtyResourceEntry,
    pub vector_capacity: *mut DirtyResourceEntry,
    pub heap_buffer_1: *mut c_void,
    pub pad_64_87: [u8; 24],
    pub heap_buffer_2: *mut c_void,
    pub sub_object_1: *mut c_void,
    pub sub_object_2: *mut c_void,
    pub final_pad: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DirtyResourceEntry {
    pub BindSlot: u32,
    pub padding: u32,
    pub StatePtr: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LaneFrameTracker {
    pub dynamic_payload_bytes: [u8; 56],
    pub is_dirty_or_active: u8,
    pub alignment_padding: [u8; 7],
    pub reservedPool: ()
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderContextBlock {
    pub reference_count: u32,
    pub padding_0: u32,
    pub mutex_handle: HANDLE,
    pub sync_events: ContextSyncHandles,
    pub pipeline_manager: PipelineCoordinator,
    pub trailing_state: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ContextSyncHandles {
    pub completion_event_0: HANDLE,
    pub completion_event_1: HANDLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCoordinator {
    pub unk_flags_or_id: u64,
    pub lane_array: [LaneSnapshotData;3],
    pub thread_buckets: [ThreadContextNode; 10],
    pub lane_lock: CRITICAL_SECTION,
    pub pad_240_263: [u8; 24],
    pub dynamic_state_array: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExecutionCoordinator {
    pub is_active_flag: u8,
    pub pad_1_15: [u8; 15],
    pub render_context: *mut RenderContextBlock,
    pub pad_24_47: [u8; 24],
    pub update_timer_0: PTP_TIMER,
    pub update_timer_1: PTP_TIMER,
    pub pad_64_71: [u8; 8],
    pub queue_lock: CRITICAL_SECTION,
    pub pad_112_135: [u8; 24],
    pub heap_buffer_0: *mut c_void,
    pub unk_state_trigger: u64,
    pub state_lock: CRITICAL_SECTION,
    pub pad_192_215: [u8; 24],
    pub heap_buffer_1: *mut c_void,
    pub telemetry_flag: u64,
    pub pad_232_255: [u8; 24],
    pub heap_buffer_2: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ManagedAllocationObject48Byte {
    pub lpVtbl: *mut IManagedObjectVtbl,
    pub reference_count: u32,
    pub padding: u32,
    pub internal_state_0: u64,
    pub internal_state_1: u64,
    pub internal_state_2: u64,
    pub trailing_handle: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ManagedAllocationObject40Byte {
    pub lpVtbl: *mut IManagedObjectVtbl,
    pub reference_count: u32,
    pub padding: u32,
    pub internal_state_0: u64,
    pub internal_state_1: u64,
    pub component_handle: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ManagedAllocationObject48Byte_Variant2 {
    pub lpVtbl: *mut IManagedObjectVtbl,
    pub reference_count: u32,
    pub padding: u32,
    pub internal_state_0: u64,
    pub internal_state_1: u64,
    pub explicit_null_ptr: *mut c_void,
    pub internal_state_3: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ObjectBaseRef {
    pub reference_count: i32,
    pub padding_0: u32,
    pub subsystem_base_lock: *mut c_void,
    pub pad_16_111: u8,
    pub sync_array_descriptor: *mut c_void,
    pub sync_array_buffer: *mut *mut c_void,
    pub variant_b_descriptor: *mut c_void,
    pub variant_b_array: *mut *mut ManagedAllocationObject48Byte_Variant2,
    pub size28_descriptor: *mut c_void,
    pub size28_array: *mut *mut ManagedAllocationObject40Byte,
    pub size30_descriptor: *mut c_void,
    pub size30_array: *mut *mut ManagedAllocationObject48Byte,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProductKeyConfigSchemaElement {
    pub lpVtbl: ProductKeyConfigSchemaElement_Vtbl,
    pub m_cRef: LONG,
    pub m_dwPadding: DWORD,
    pub m_reserved: [BYTE; 16],
    pub m_pPropertyContainer: *mut c_void,
    pub m_pParentSchema: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProductKeyConfigSchemaElement_Vtbl {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProductKeyConfigGuidElement {
    pub lpVtbl: ProductKeyConfigGuidElement_Vtbl,
    pub m_cRef: LONG,
    pub m_dwPadding: DWORD,
    pub m_reserved: [BYTE; 16],
    pub m_guidValue: GUID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProductKeyConfigGuidElement_Vtbl {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SymmetricCryptoContext {
    pub dwUnusedOffset0: DWORD,
    pub dwUnusedOffset4: DWORD,
    pub dwUnusedOffset8: DWORD,
    pub dwAlgorithmVariant: DWORD,
    pub pKeyIdString: *mut u16,
    pub dwKeyIdLength: DWORD,
    pub dwPadding: DWORD,
    pub pInitializationVector: *mut c_void,
    pub dwIvBufferLength: DWORD,
    pub dwPadding2: DWORD,
    pub hCryptKey: HCRYPTKEY,
    pub hCryptProv: HCRYPTPROV,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigPathDataNode {
    pub node_tracking_id: u32,
    pub padding: u32,
    pub node_element_name: *const i8,
    pub unk_metric_0: u64,
    pub unk_metric_1: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigHierarchyNode {
    pub unk_state_flags: u64,
    pub unk_sibling_ptr: u64,
    pub next_traverse_node: *mut ConfigHierarchyNode,
    pub padding: u64,
    pub path_data: *mut ConfigPathDataNode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ThreadContextNode {
    pub thread_id: u32,
    pub padding: u32,
    pub next_node: *mut ThreadContextNode,
    pub thread_payload_ptr: *mut c_void,
    pub coordinator_back_ptr: *mut c_void,
}

impl ThreadContextNode {
    pub(crate) fn is_null(&self) -> bool {
        todo!()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigArrayElement {
    pub internal_state_bytes: [u8; 64],
    pub sub_allocated_buffer: *mut c_void,
    pub unk_tracking_value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigVectorHeader {
    pub pad_0_23: [u8; 24],
    pub array_base_pointer: *mut ConfigArrayElement,
    pub element_count: u16,
    pub capacity_or_padding: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigHashNode {
    pub node_key_or_id: u64,
    pub next_node: *mut ConfigHashNode,
    pub element_vector: ConfigVectorHeader,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigHashMapContainer {
    pub buckets: *mut ConfigHashNode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigManager {
    pub reference_count: i32,
    pub padding_0: u32,
    pub session_mutex: HANDLE,
    pub completion_event_0: HANDLE,
    pub completion_event_1: HANDLE,
    pub padding_or_flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigHashNodeSimple {
    pub node_key_or_id: u64,
    pub next_node: *mut ConfigHashNodeSimple,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConfigHashMapSimpleContainer {
    pub hash_buckets: *mut ConfigHashNodeSimple,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RelocDescriptorEntry {
    pub padding: u64, // C bitfield: 3 bits
    pub block_rva: u64, // C bitfield: 28 bits
    pub block_length: u64, // C bitfield: 28 bits
    pub custom_flags: u64, // C bitfield: 5 bits
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RelocTableHeader {
    pub entry_count_marker: u32,
    pub alignment_padding: u32,
    pub entries: [RelocDescriptorEntry; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IStreamAccumulatorVtbl {
    pub unk_method_0: *mut c_void,
    pub PushStreamByte: Option<unsafe fn(*mut *mut IStreamAccumulatorVtbl, *mut i64, u64) -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryMappedViewEntry {
    pub pMappedBaseAddress: *mut c_void,
    pub hFileMappingObject: HANDLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MappedViewVectorHeader {
    pub allocated_capacity: i32,
    pub current_element_count: i32,
    pub pArrayBuffer: *mut MemoryMappedViewEntry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProtectedSubsystemContext {
    pub allocated_capacity: i32,
    pub current_element_count: i32,
    pub pMappedViewsArray: *mut MemoryMappedViewEntry,
    pub obfuscated_state_flags: u64,
}

impl ProtectedSubsystemContext {
    pub(crate) fn is_null(&self) -> bool {
        todo!()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecureCallbackNode {
    pub blink: *mut SecureCallbackNode,
    pub pEncodedInitFunc: PVOID,
    pub pEncodedShutdownFunc: PVOID,
    pub registration_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataNode {
    pub ID: LONG,
    pub NameComponent: *mut i8,
    pub Metadata: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TreeNode {
    pub Unknown0: *mut c_void,
    pub Unknown8: *mut c_void,
    pub NextNode: *mut TreeNode,
    pub Unknown24: *mut c_void,
    pub Data: *mut DataNode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ContextState {
    pub Padding: [i8; 80],
    pub CachedData_OWORD1: i128,
    pub CachedData_QWORD1: i64,
    pub CachedData_OWORD2: i128,
    pub CachedData_QWORD2: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SPP_STRING_BUFFER {
    pub LengthInCharacters: u32,
    pub Buffer: [u16; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ISppNamespaceVtbl {
    pub QueryInterface: *mut c_void,
    pub AddRef: *mut c_void,
    pub Release: *mut c_void,
    pub OpenNamespace: Option<unsafe fn(*mut ISppNamespace, i64, *const u16, *const u16, *mut HLOCAL) -> i64>,
    pub GetNamespaceProvider: Option<unsafe fn(*mut ISppNamespace, HLOCAL, *mut HLOCAL) -> i64>,
    pub DeleteProperty: Option<unsafe fn(*mut ISppNamespace, i32, *const u16, *const u16) -> i64>,
    pub CommitProperty: Option<unsafe fn(*mut ISppNamespace, i32, *const u16, *const u16, *mut c_void) -> i64>,
    pub PropertyExists: Option<unsafe fn(*mut ISppNamespace, i32, PCWSTR, i32) -> HRESULT>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ISppCollectionVtbl {
    pub QueryInterface: *mut c_void,
    pub AddRef: *mut c_void,
    pub Release: *mut c_void,
    pub GetCount: Option<unsafe fn(*mut c_void, *mut u32) -> i64>,
    pub GetItemAt: Option<unsafe fn(*mut c_void, u32, *mut c_void) -> i64>,
    pub SkipOrVerifyLayout: Option<unsafe fn(*mut c_void, *mut c_void, i32) -> i64>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppNamespace {
    pub lpVtbl: *mut ISppNamespaceVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppCollection {
    pub lpVtbl: *mut ISppCollectionVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppPluginParams {
    pub lpVtbl: *mut ISppCollectionVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SPP_VECTOR {
    pub capacity: u32,
    pub count: u32,
    pub p_elements_array: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PointerVector {
    pub capacity: i32,
    pub size: i32,
    pub elements: *mut *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppPropertyBag {
    pub lpVtbl: *mut ISppPropertyBagVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppPropertyBagVtbl {
    pub QueryInterface: *mut c_void,
    pub AddRef: *mut c_void,
    pub Release: Option<unsafe fn(*mut ISppPropertyBag) -> i64>,
    pub UnknownMethod3: Option<unsafe fn(*mut ISppPropertyBag) -> i64>,
    pub UnknownMethod4: Option<unsafe fn(*mut ISppPropertyBag) -> i64>,
    pub UnknownMethod5: Option<unsafe fn(*mut ISppPropertyBag) -> i64>,
    pub GetProperty: Option<unsafe fn(*mut ISppPropertyBag, *const u16, *mut c_void) -> i64>,
    pub SetProperty: Option<unsafe fn(*mut ISppPropertyBag, *const u16, u128) -> i64>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppHost {
    pub lpVtbl: *mut ISppHostVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppHostVtbl {
    pub QueryInterface: *mut c_void,
    pub AddRef: *mut c_void,
    pub Release: *mut c_void,
    pub UnknownMethod3: *mut c_void,
    pub UnknownMethod4: *mut c_void,
    pub RegisterElement: Option<unsafe fn(*mut ISppHost, *mut i64, *mut c_void) -> i64>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppStringBuilder {
    pub buffer: *mut u16,
    pub currentLength: i32,
    pub totalCapacity: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppLicenseComponent {
    pub lpVtbl: *mut ISppLicenseComponentVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppLicenseComponentVtbl {
    pub QueryInterface: *mut c_void,
    pub AddRef: *mut c_void,
    pub Release: Option<unsafe fn(*mut ISppLicenseComponent) -> ()>,
    pub GetPropertyValue: Option<unsafe fn(*mut ISppLicenseComponent, *const u16, *mut HLOCAL) -> i64>,
    pub ValidateState: Option<unsafe fn(*mut ISppLicenseComponent) -> i64>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppBindingContext {
    pub appId: i64,
    pub skuId: i64,
    pub pkeyId: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppThreadContextBody {
    pub unknownPtr: *mut c_void,
    pub globalContext: *mut c_void,
    pub lock: SppCustomLock,
    pub alignmentCushion: i64,
    pub threadIdMap: ThreadMap,
    pub threadContextArray: *mut *mut c_void,
    pub trailingPadding: [i8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppThreadContextManager {
    pub padding: [i8; 72],
    pub body: *mut SppThreadContextBody,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ThreadSlot {
    pub threadId: DWORD,
    pub slotRecursionCount: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppCustomLock {
    pub padding: [i8; 8],
    pub internalLock: CRITICAL_SECTION,
    pub hEvent: HANDLE,
    pub activeWritersCount: i32,
    pub queuedWaitersCount: i32,
    pub isWriterMode: i32,
    pub unknownFlag: i32,
    pub owningThreadId: DWORD,
    pub recursionCount: i32,
    pub pThreadSlotsArray: *mut ThreadSlot,
    pub maxSlotsCount: i32,
    pub inlineSlot: *mut ThreadSlot,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ThreadEntry {
    pub threadId: DWORD,
    pub padding: i32,
    pub context: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ThreadMap {
    pub capacity: i32,
    pub elementCount: i32,
    pub array: *mut ThreadEntry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppLicenseAttributeQuery {
    pub lpVtbl: *mut ISppLicenseAttributeQueryVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppLicenseAttributeQueryVtbl {
    pub QueryInterface: *mut c_void,
    pub AddRef: *mut c_void,
    pub Release: *mut c_void,
    pub GetLicenseAttribute: Option<unsafe fn(*mut ISppLicenseAttributeQuery, i64, *const u16, *mut HLOCAL) -> i64>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppTimerCallbackContext {
    pub lpVtbl: *mut ISppTimerCallbackContextVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppTimerCallbackContextVtbl {
    pub QueryInterface: Option<unsafe fn(*mut ISppTimerCallbackContext, *const GUID, *mut *mut c_void) -> i64>,
    pub AddRef: Option<unsafe fn(*mut ISppTimerCallbackContext) -> u32>,
    pub Release: Option<unsafe fn(*mut ISppTimerCallbackContext) -> u32>,
    pub Initialize: Option<unsafe fn(*mut ISppTimerCallbackContext, *mut c_void) -> i64>,
    pub EvaluateState: Option<unsafe fn(*mut ISppTimerCallbackContext, i64, *mut c_void) -> i64>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppTimerCallbackBase {
    pub lpVtbl: *mut SppTimerCallbackBaseVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppTimerCallbackBaseVtbl {
    pub Serialize: Option<unsafe fn(*mut SppTimerCallbackBase) -> u32>,
    pub Stop: Option<unsafe fn(*mut SppTimerCallbackBase) -> u32>,
    pub Initialize: Option<unsafe fn(*mut SppTimerCallbackBase) -> u32>,
    pub EvaluateState: Option<unsafe fn(*mut SppTimerCallbackBase) -> u32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppTimerCallback {
    pub lpVtbl: *mut SppTimerCallbackVtbl,
    pub padding: i32,
    pub timerId: u32,
    pub lastStatus: HRESULT,
    pub isActive: i32,
    pub initializationState: i32,
    pub padding2: i32,
    pub timeoutDuration: u32,
    pub remainingTime: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppTimerCallbackVtbl {
    pub Serialize: Option<unsafe fn(*mut SppTimerCallback, *mut SppStringBuilder) -> i64>,
    pub Stop: Option<unsafe fn(*mut SppTimerCallback, i64) -> HRESULT>,
    pub Initialize: Option<unsafe fn(*mut SppTimerCallback, *mut i64) -> HRESULT>,
    pub EvaluateState: Option<unsafe fn(*mut SppTimerCallback, i64, *mut c_void) -> HRESULT>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppTimerRegistry {
    pub lpVtbl: *mut ISppTimerRegistryVtbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ISppTimerRegistryVtbl {
    pub QueryInterface: *mut c_void,
    pub AddRef: *mut c_void,
    pub Release: *mut c_void,
    pub UnknownMethod3: *mut c_void,
    pub UnknownMethod4: *mut c_void,
    pub UnknownMethod5: *mut c_void,
    pub UnknownMethod6: *mut c_void,
    pub UnknownMethod7: *mut c_void,
    pub RegisterTimerHook: Option<unsafe fn(*mut ISppTimerRegistry, *mut c_void) -> HRESULT>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppLicenseComponentRecord {
    pub padding: [i8; 20],
    pub isActive: i32,
    pub padding2: [i8; 4],
    pub isTimerEnabled: i32,
    pub pTimerContext: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppRearmContext {
    pub pwszAppId: *mut u16,
    pub pwszSkuId: *mut u16,
    pub padding: [i8; 44],
    pub isNotificationRequired: i32,
    pub hLicenseContext: i64,
    pub padding2: [i8; 4],
    pub componentsCount: i32,
    pub pComponentsArray: *mut SppLicenseComponentRecord,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SppPolicyType {
    SppPolicyType_DWORD = 0x1,
    SppPolicyType_String = 0x2,
    SppPolicyType_Binary = 0x4,
}

/// Original IDA name: `SppPolicyVariant::$E4770D199A2960AAB27276250816D5C7`
#[repr(C)]
#[derive(Copy, Clone)]
pub union _SppPolicyVariant___E4770D199A2960AAB27276250816D5C7 {
    pub dwordValue: u32,
    pub pwszValue: *const u16,
    pub pBuffer: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppPolicyVariant {
    pub lpVtbl: *mut c_void,
    pub policyType: i32,
    pub padding: i32,
    pub data: _SppPolicyVariant___E4770D199A2960AAB27276250816D5C7,
    pub binaryBufferSize: i32,
    pub trailingPadding: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppCryptoContext {
    pub isHashActive: i32,
    pub padding: i32,
    pub hAlgProvider: BCRYPT_ALG_HANDLE,
    pub hHashInstance: BCRYPT_HASH_HANDLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppTelemetryContext {
    pub initializationState: i8,
    pub padding: [i8; 7],
    pub hEtwRegistration: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppStateTransitionResult {
    pub isFirstInitialization: i32,
    pub previousCounter: i32,
    pub previousState: i32,
    pub validationError: i32,
    pub isTransitionValid: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VectorLayout24 {
    pub elements: *mut c_void,
    pub capacity: *mut c_void,
    pub currentCursor: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppTelemetryQueueManager {
    pub status_flag: i32,
    pub padding: i32,
    pub lock: SRWLOCK,
    pub hThreadpoolTimer: PTP_TIMER,
    pub isTimerActive: bool,
    pub reserved: [i8; 7],
    pub telemetryQueue: VectorLayout24,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppStringTokenizer {
    pub pCurrentTokenEnd: *mut u16,
    pub pSourceStringBase: *mut u16,
    pub wchDelimiter: u16,
    pub padding: [i8; 2],
    pub cchTotalCharacters: u32,
    pub pCurrentTokenStart: *mut u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VectorLayoutBuffered {
    pub capacity: i32,
    pub count: i32,
    pub ppBuffer: *mut *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SppCustomLockV2 {
    pub padding: i8,
    pub internalLock: CRITICAL_SECTION,
    pub hEvent: HANDLE,
    pub padding2: i32,
    pub activeWritersCount: i32,
    pub isWriterMode: i32,
    pub unknownFlag: i32,
    pub queuedWaitersCount: i32,
    pub targetStateMarker: i32,
    pub owningThreadId: DWORD,
    pub recursionCount: i32,
    pub pThreadSlotsArray: *mut ThreadSlot,
    pub maxSlotsCount: i32,
}
pub type NTSTATUS = i32;
pub type RtlNtStatusToDosErrorFn = unsafe extern "system" fn(NTSTATUS) -> ULONG;

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

pub type HANDLE_FLAGS = u32;

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
    pub Status: LSTATUS,
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

pub type LSTATUS = HRESULT;

pub struct PropertyStruct {
    pub(crate) key_name: *const u16,
    pub(crate) property_type: i32,
    pub(crate) padding: i32,
    pub(crate) value: PCWSTR,
    pub(crate) unused: i32
}

pub type PROCESS_CREATION_FLAGS = u32;


pub type SppVector = _SPP_VECTOR;

#[repr(C)]
pub struct SID_IDENTIFIER_AUTHORITY {
    pub Value: [u8; 6],
}

pub type PSID_IDENTIFIER_AUTHORITY = *mut SID_IDENTIFIER_AUTHORITY;

struct QueryContextParams {
    system_info_type: i32,
    target_ptr: *mut c_void,
    global_ref: *const usize,
    sku_guid: u64,
    out_status: HRESULT,
    output_buf: *mut u8,
    app_guid_ptr: *mut *mut c_void,
    reserved1: i32,
    reserved2: i32,
    reserved3: i32
}

type C_VOID = c_void;

#[repr(C)]
pub struct ContextEntry {
    pub ptr: PVOID,
}