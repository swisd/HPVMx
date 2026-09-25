/*uintptr_t __fastcall security_init_cookie()
{
uintptr_t cookie; // rax
uintptr_t new_cookie; // rcx
uintptr_t cookie_complement; // rax
struct _FILETIME cookie_hash; // [rsp+30h] [rbp+10h] BYREF
LARGE_INTEGER perf_count; // [rsp+38h] [rbp+18h] BYREF
struct _FILETIME system_time; // [rsp+40h] [rbp+20h] BYREF

system_time = 0;
perf_count.QuadPart = 0;
cookie = _security_cookie;
if ( _security_cookie == 0x2B992DDFA232LL )
{
GetSystemTimeAsFileTime(&system_time);
cookie_hash = system_time;
cookie_hash = (struct _FILETIME)(GetCurrentProcessId() ^ *(unsigned __int64 *)&cookie_hash);
cookie_hash = (struct _FILETIME)(GetCurrentThreadId() ^ *(unsigned __int64 *)&cookie_hash);
cookie_hash = (struct _FILETIME)(((unsigned __int64)GetTickCount() << 24) ^ *(_QWORD *)&cookie_hash);
cookie_hash = (struct _FILETIME)((unsigned __int64)&cookie_hash ^ *(unsigned __int64 *)&cookie_hash ^ GetTickCount());
QueryPerformanceCounter(&perf_count);
cookie = (*(_QWORD *)&cookie_hash
^ perf_count.QuadPart
^ ((unsigned __int64)perf_count.LowPart << 32))
& 0xFFFFFFFFFFFFLL;
new_cookie = cookie;
if ( cookie == 0x2B992DDFA232LL )
{
cookie = 0x2B992DDFA233LL;                // DEFAULT_SECURITY_COOKIE
new_cookie = 0x2B992DDFA233LL;
}
_security_cookie = new_cookie;
}
cookie_complement = ~cookie;
_security_cookie_complement = cookie_complement;
return cookie_complement;
}*/
use core::arch::x86_64::_rdtsc;

static mut SECURITY_COOKIE: usize = 0x0u64 as usize;
static mut SECURITY_COOKIE_COMPLEMENT: usize = 0x0u64 as usize;

unsafe fn security_init_cookie() -> usize {
    let mut cookie: usize; // rax
    let mut new_cookie: usize; // rcx
    let mut cookie_complement: usize; // rax
    let mut cookie_hash: u64 = 0; // [rsp+30h] [rbp+10h] BYREF
    let mut perf_count: i64 = 0; // [rsp+38h] [rbp+18h] BYREF
    let mut system_time: u64 = 0; // [rsp+40h] [rbp+20h] BYREF

    system_time = 0;
    perf_count = 0;
    cookie = SECURITY_COOKIE;
    if SECURITY_COOKIE == 0x2B992DDFA232_u64 as usize {
        system_time = _rdtsc();
        cookie_hash = system_time;
        cookie_hash ^= (9001u64); // PID
        cookie_hash ^= (0x4734792763457647u64); // Thread ID
        cookie_hash ^= (_rdtsc()) << 24;
        cookie_hash ^= (_rdtsc());
        cookie_hash ^= (&cookie_hash as *const _ as u64);
        cookie = ((cookie_hash ^ perf_count as u64 ^ ((perf_count as u32 as u64) << 32)) & 0xFFFFFFFFFFFF) as usize;
        new_cookie = cookie;
        if cookie == 0x2B992DDFA232_u64 as usize {
            cookie = 0x2B992DDFA233_u64 as usize; // DEFAULT_SECURITY_COOKIE
            new_cookie = 0x2B992DDFA233_u64 as usize;
        }
        SECURITY_COOKIE = new_cookie;
    }
    cookie_complement = !cookie;
    SECURITY_COOKIE_COMPLEMENT = cookie_complement;
    cookie_complement
}

/*void __cdecl _security_check_cookie(uintptr_t StackCookie)
{
__int64 v1; // rcx

if ( StackCookie != _security_cookie )
LABEL_4:
report_gsfailure(StackCookie);
v1 = __ROL8__(StackCookie, 16);
if ( (_WORD)v1 )
{
StackCookie = __ROR8__(v1, 16);
goto LABEL_4;
}
}*/
unsafe fn _security_check_cookie(stack_cookie: usize) {
    let mut v1: i64; // rcx

    if stack_cookie != SECURITY_COOKIE {
        report_gsfailure(stack_cookie);
    }
    v1 = stack_cookie.rotate_left(16) as i64;
    if (v1 as u16) != 0 {
        let stack_cookie = v1.rotate_right(16) as usize;
        report_gsfailure(stack_cookie);
    }
}

/*void __fastcall __noreturn report_gsfailure(DWORD64 corruptedCookie)
{
struct _RUNTIME_FUNCTION *FunctionEntry; // [rsp+40h] [rbp-48h]
DWORD64 ControlPc; // [rsp+48h] [rbp-40h]
unsigned __int64 ImageBase; // [rsp+50h] [rbp-38h] BYREF
unsigned __int64 EstablisherFrame; // [rsp+58h] [rbp-30h] BYREF
PVOID HandlerData[5]; // [rsp+60h] [rbp-28h] BYREF

EstablisherFrame = 0;
HandlerData[0] = nullptr;
ImageBase = 0;
RtlCaptureContext(&ContextRecord);
ControlPc = ContextRecord.Rip;
FunctionEntry = RtlLookupFunctionEntry(ContextRecord.Rip, &ImageBase, nullptr);
if ( FunctionEntry )
{
RtlVirtualUnwind(0, ImageBase, ControlPc, FunctionEntry, &ContextRecord, HandlerData, &EstablisherFrame, nullptr);
}
else
{
ContextRecord.Rip = *(_QWORD *)ContextRecord.Rsp;
ContextRecord.Rsp += 8LL;
}
qword_14046ACC0 = ContextRecord.Rip;
ContextRecord.Rcx = corruptedCookie;
dword_14046ACB0 = -1073740791;
dword_14046ACB4 = 1;
dword_14046ACC8 = 3;
unk_14046ACD0 = 2;
unk_14046ACD8 = _security_cookie;
unk_14046ACE0 = _security_cookie_complement;
HandlerData[2] = (PVOID)_security_cookie_complement;
_raise_securityfailure((struct _EXCEPTION_POINTERS *)&ExceptionInfo);
}*/
// #[noreturn]
unsafe /*extern "fastcall"*/ fn report_gsfailure(corrupted_cookie: usize) {
    // let mut function_entry: *mut _RUNTIME_FUNCTION; // FunctionEntry
    // let mut control_pc: u64; // ControlPc
    // let mut image_base: u64 = 0; // ImageBase
    // let mut establisher_frame: u64 = 0; // EstablisherFrame
    // let mut handler_data: [Option<*mut core::ffi::c_void>; 5] = [None; 5]; // HandlerData
    //
    // RtlCaptureContext(&mut CONTEXT_RECORD);
    // control_pc = CONTEXT_RECORD.Rip;
    // function_entry = RtlLookupFunctionEntry(CONTEXT_RECORD.Rip, &mut image_base as *mut u64, core::ptr::null_mut());
    // if !function_entry.is_null() {
    //     RtlVirtualUnwind(
    //         0,
    //         image_base,
    //         control_pc,
    //         function_entry,
    //         &mut CONTEXT_RECORD,
    //         handler_data.as_mut_ptr(),
    //         &mut establisher_frame,
    //         core::ptr::null_mut(),
    //     );
    // } else {
    //     CONTEXT_RECORD.Rip = *(CONTEXT_RECORD.Rsp as *const u64);
    //     CONTEXT_RECORD.Rsp += 8;
    // }
    // qword_14046ACC0 = CONTEXT_RECORD.Rip;
    // CONTEXT_RECORD.Rcx = corrupted_cookie;
    // dword_14046ACB0 = -1073740791;
    // dword_14046ACB4 = 1;
    // dword_14046ACC8 = 3;
    // unk_14046ACD0 = 2;
    // unk_14046ACD8 = _security_cookie;
    // unk_14046ACE0 = _security_cookie_complement;
    // handler_data[2] = Some(_security_cookie_complement as *mut core::ffi::c_void);
    // _raise_securityfailure(&mut EXCEPTION_INFO as *mut _EXCEPTION_POINTERS);
}