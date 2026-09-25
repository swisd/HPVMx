use core::ffi::{c_void, VaList};
use crate::vdebug_autoprefix;
use crate::x4::rawasm::FARPROC;
use crate::x4::types::{__int64, DWORD, HANDLE, LPVOID, PSRWLOCK, PVOID, PCWSTR, PWSTR, SIZE_T, LPCRITICAL_SECTION, BOOL, CRITICAL_SECTION, HKEY, LPCWSTR, REGSAM, PHKEY, LPDWORD, LPBYTE, HLOCAL, UINT, LSTATUS, WCHAR, HRESULT, PSID, HMODULE, PCSTR, SECURITY_ATTRIBUTES, PTP_TIMER, FILETIME, SID_IDENTIFIER_AUTHORITY, BCRYPT_HASH_HANDLE, NTSTATUS, BCRYPT_ALG_HANDLE, GUID, SRWLOCK};

/// Host services used by x4. Install a static implementation before calling x4 entry points.
/// Methods have zero/no-op defaults so a new kernel can implement services incrementally;
/// an unimplemented service must be treated as unavailable by its caller.
pub trait X4ExternalApi {
    unsafe fn x4_acquiresrwlockexclusive(&self, SRWLock: PSRWLOCK) {  }
    unsafe fn x4_releasesrwlockexclusive(&self, SRWLock: PSRWLOCK) {  }
    unsafe fn x4_getprocessheap(&self) -> HANDLE { core::mem::zeroed() }
    unsafe fn x4_heapfree(&self, hHeap: HANDLE, dwFlags: DWORD, lpMem: LPVOID) {  }
    unsafe fn x4_etwwritetransfer(&self, a1: __int64, a2: __int64, a3: __int64) -> i32 { core::mem::zeroed() }
    unsafe fn x4_getcurrentprocessid(&self) -> DWORD { core::mem::zeroed() }
    unsafe fn x4__vsnwprintf(&self, buffer: PWSTR, count: usize, format: PCWSTR, argptr: VaList) -> i32 { core::mem::zeroed() }
    unsafe fn x4_encodepointer(&self, Ptr: PVOID) -> PVOID { core::mem::zeroed() }
    unsafe fn x4_heapalloc(&self, hHeap: HANDLE, dwFlags: DWORD, dwBytes: SIZE_T) -> LPVOID { core::mem::zeroed() }
    unsafe fn x4_getlasterror(&self) -> DWORD { core::mem::zeroed() }
    unsafe fn x4_setlasterror(&self, dwErrorCode: DWORD) {  }
    unsafe fn x4_initializecriticalsectionandspincount(&self, lpCriticalSection: LPCRITICAL_SECTION, dwSpinCount: DWORD) -> BOOL { core::mem::zeroed() }
    unsafe fn x4_releasesemaphore(&self, p0: i32, p1: i32, p2: *mut c_void) -> i32 { core::mem::zeroed() }
    unsafe fn x4_setevent(&self, p0: HANDLE) {  }
    unsafe fn x4_leavecriticalsection(&self, p0: LPCRITICAL_SECTION) {  }
    unsafe fn x4_raiseexception(&self, p0: i32, p1: i32, p2: i32, p3: *const c_void) {  }
    unsafe fn x4_getcurrentthreadid(&self) -> DWORD { core::mem::zeroed() }
    unsafe fn x4_waitforsingleobject(&self, hHandle: HANDLE, dwMiliseconds: DWORD) -> u32 { core::mem::zeroed() }
    unsafe fn x4_regopenkeyexw(&self, hKey: HKEY, lpSubKey: LPCWSTR, ulOptions: DWORD, samDesired: REGSAM, phkResult: PHKEY) -> LSTATUS { core::mem::zeroed() }
    unsafe fn x4_regclosekey(&self, hKey: HKEY) {  }
    unsafe fn x4_regqueryvalueexw(&self, hKey: HKEY, lpValueName: LPCWSTR, lpReserved: LPDWORD, lpType: LPDWORD, lpData: LPBYTE, lpcbData: LPDWORD) -> LSTATUS { core::mem::zeroed() }
    unsafe fn x4_localalloc(&self, uFlags: UINT, uBytes: SIZE_T) -> HLOCAL { core::mem::zeroed() }
    unsafe fn x4_localfree(&self, hMem: HLOCAL) {  }
    unsafe fn x4_rpcreverttoselfex(&self, BindingHandle: RPC_BINDING_HANDLE) -> RPC_STATUS { core::mem::zeroed() }
    unsafe fn x4_i_rpcmapwin32status(&self, Status: RPC_STATUS) -> HRESULT { core::mem::zeroed() }
    unsafe fn x4_rpcimpersonateclient(&self, BindingHandle: RPC_BINDING_HANDLE) -> RPC_STATUS { core::mem::zeroed() }
    unsafe fn x4_convertstringsidtosidw(&self, stringsid: *const u16, sid: *mut *mut c_void) -> i32 { core::mem::zeroed() }
    unsafe fn x4_checktokenmembership(&self, tokenhandle: HANDLE, sidtocheck: PSID, isadmin: *mut BOOL) -> BOOL { core::mem::zeroed() }
    unsafe fn x4_freesid(&self, psid: PSID) -> *mut c_void { core::mem::zeroed() }
    unsafe fn x4_closehandle(&self, hobject: HANDLE) -> BOOL { core::mem::zeroed() }
    unsafe fn x4_waitforsingleobjectex(&self, hhandle: HANDLE, dwmilliseconds: u32, balertable: BOOL) -> u32 { core::mem::zeroed() }
    unsafe fn x4_releasemutex(&self, hmutex: HANDLE) -> BOOL { core::mem::zeroed() }
    unsafe fn x4_deletecriticalsection(&self, lpcriticalsection: *mut CRITICAL_SECTION) {  }
    unsafe fn x4_getmodulehandlew(&self, lpmodulename: *const u16) -> HMODULE { core::mem::zeroed() }
    unsafe fn x4_getprocaddress(&self, hmodule: HMODULE, lpprocname: PCSTR) -> FARPROC { core::mem::zeroed() }
    unsafe fn x4__dllonexit(&self, func: OnExitCallback, pbegin: *mut *mut PVFV, pend: *mut *mut PVFV) -> *mut core::ffi::c_void { core::mem::zeroed() }
    unsafe fn x4_onexit(&self, function: OnExitCallback) -> OnExitT { core::mem::zeroed() }
    unsafe fn x4_createsemaphoreexw(&self, lpsemaphoreattributes: *const SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: *const u16, dwflags: u32, dwdesiredaccess: u32) -> HANDLE { core::mem::zeroed() }
    unsafe fn x4_opensemaphorew(&self, dwdesiredaccess: u32, binherithandle: BOOL, lpname: *const u16) -> HANDLE { core::mem::zeroed() }
    unsafe fn x4_initializecriticalsectionex(&self, lpcriticalsection: *mut CRITICAL_SECTION, dwspincount: u32, flags: u32) -> BOOL { core::mem::zeroed() }
    unsafe fn x4_setthreadpooltimer(&self, ptmi: PTP_TIMER, pftduetime: *const FILETIME, msperiod: u32, mswindowlength: u32) {  }
    unsafe fn x4_waitforthreadpooltimercallbacks(&self, ptmi: PTP_TIMER, fcancelpendingcallbacks: BOOL) {  }
    unsafe fn x4_closethreadpooltimer(&self, ptmi: PTP_TIMER) {  }
    unsafe fn x4_createmutexexw(&self, lpmutexattributes: *const SECURITY_ATTRIBUTES, lpname: *const u16, dwflags: u32, dwdesiredaccess: u32) -> HANDLE { core::mem::zeroed() }
    unsafe fn x4_entercriticalsection(&self, lpcriticalsection: *mut CRITICAL_SECTION) {  }
    unsafe fn x4_allocateandinitializesid(&self, pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8, nsubauthority0: u32, nsubauthority1: u32, nsubauthority2: u32, nsubauthority3: u32, nsubauthority4: u32, nsubauthority5: u32, nsubauthority6: u32, nsubauthority7: u32, psid: *mut PSID) -> BOOL { core::mem::zeroed() }
    unsafe fn x4_lcmapstringw(&self, locale: u32, dwmapflags: u32, lpsrcstr: *const u16, ccntsrc: i32, lpdeststr: *mut u16, ccntdest: i32) -> i32 { core::mem::zeroed() }
    unsafe fn x4_bcryptfinishhash(&self, hhash: BCRYPT_HASH_HANDLE, pboutput: *mut u8, cboutput: u32, dwflags: u32) -> NTSTATUS { core::mem::zeroed() }
    unsafe fn x4_bcryptdestroyhash(&self, hhash: BCRYPT_HASH_HANDLE) -> NTSTATUS { core::mem::zeroed() }
    unsafe fn x4_bcryptclosealgorithmprovider(&self, halgorithm: BCRYPT_ALG_HANDLE, dwflags: u32) -> NTSTATUS { core::mem::zeroed() }
    unsafe fn x4_bcrypthashdata(&self, hhash: BCRYPT_HASH_HANDLE, pbinput: *mut u8, cbinput: u32, dwflags: u32) -> NTSTATUS { core::mem::zeroed() }
    unsafe fn x4_bcryptopenalgorithmprovider(&self, phalgorithm: *mut BCRYPT_ALG_HANDLE, pszalgid: *const u16, pszimplementation: *const u16, dwflags: u32) -> NTSTATUS { core::mem::zeroed() }
    unsafe fn x4_bcryptcreatehash(&self, halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut BCRYPT_HASH_HANDLE, pbhashobject: *mut u8, cbhashobject: u32, pbsecret: *mut u8, cbsecret: u32, dwflags: u32) -> NTSTATUS { core::mem::zeroed() }
    unsafe fn x4_openthreadtoken(&self, threadhandle: HANDLE, desiredaccess: u32, openaself: BOOL, tokenhandle: *mut HANDLE) -> BOOL { core::mem::zeroed() }
    unsafe fn x4_getcurrentthread(&self) -> HANDLE { core::mem::zeroed() }
    unsafe fn x4_rtlquerypackageclaims(&self, token_handle: HANDLE, package_full_name: *mut u16, package_size: *mut usize, app_id: *mut u16, app_id_size: *mut usize, dynamic_id: *mut GUID, pkg_claim: *mut PS_PKG_CLAIM, attributes_present: *mut u64) -> NTSTATUS { core::mem::zeroed() }
    unsafe fn x4__wcsicmp(&self, string1: *const u16, string2: *const u16) -> i32 { core::mem::zeroed() }
    unsafe fn x4_ntcurrentteb(&self) -> *mut TEB { core::mem::zeroed() }
    unsafe fn x4_ntquerysysteminformation(&self, system_information_class: u32, system_information: *mut c_void, system_information_length: ULONG, return_length: *mut ULONG) -> NTSTATUS { core::mem::zeroed() }
    unsafe fn x4_unmapviewoffile(&self, lpbaseaddress: *const c_void) -> BOOL { core::mem::zeroed() }
    unsafe fn x4_gettickcount(&self) -> u32 { core::mem::zeroed() }
    unsafe fn x4_createthreadpooltimer(&self, pfnti: PTP_TIMER_CALLBACK, pv: *mut c_void, pcbe: *const TP_CALLBACK_ENVIRON) -> PTP_TIMER { core::mem::zeroed() }
    unsafe fn x4_getmodulehandleexw(&self, dwflags: u32, lpmodulename: *const u16, phmodule: *mut HMODULE) -> BOOL { core::mem::zeroed() }
    unsafe fn x4_releasesrwlockshared(&self, srwlock: *mut SRWLOCK) {  }
    unsafe fn x4_acquiresrwlockshared(&self, srwlock: *mut SRWLOCK) {  }
    unsafe fn x4_queryperformancecounter(&self, lpperformancecount: *mut i64) -> BOOL { core::mem::zeroed() }
}

static mut X4_EXTERNAL_API: Option<&'static dyn X4ExternalApi> = None;

/// Set the host implementation used by every x4 external call.
pub unsafe fn install_x4_external_api(api: &'static dyn X4ExternalApi) {
    // SAFETY: installation must happen during single-threaded startup, before x4 calls begin.
    unsafe { X4_EXTERNAL_API = Some(api); }
}

#[inline]
pub unsafe fn x4_acquiresrwlockexclusive(SRWLock: PSRWLOCK) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_acquiresrwlockexclusive(SRWLock), None => () } }
}

#[inline]
pub unsafe fn x4_releasesrwlockexclusive(SRWLock: PSRWLOCK) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_releasesrwlockexclusive(SRWLock), None => () } }
}

#[inline]
pub unsafe fn x4_getprocessheap() -> HANDLE {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_getprocessheap(), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_heapfree(hHeap: HANDLE, dwFlags: DWORD, lpMem: LPVOID) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_heapfree(hHeap, dwFlags, lpMem), None => () } }
}

#[inline]
pub unsafe fn x4_etwwritetransfer(a1: __int64, a2: __int64, a3: __int64) -> i32 {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_etwwritetransfer(a1, a2, a3), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_getcurrentprocessid() -> DWORD {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_getcurrentprocessid(), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4__vsnwprintf(buffer: PWSTR, count: usize, format: PCWSTR, argptr: VaList) -> i32 {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4__vsnwprintf(buffer, count, format, argptr), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_encodepointer(Ptr: PVOID) -> PVOID {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_encodepointer(Ptr), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_heapalloc(hHeap: HANDLE, dwFlags: DWORD, dwBytes: SIZE_T) -> LPVOID {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_heapalloc(hHeap, dwFlags, dwBytes), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_getlasterror() -> DWORD {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_getlasterror(), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_setlasterror(dwErrorCode: DWORD) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_setlasterror(dwErrorCode), None => () } }
}

#[inline]
pub unsafe fn x4_initializecriticalsectionandspincount(lpCriticalSection: LPCRITICAL_SECTION, dwSpinCount: DWORD) -> BOOL {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_initializecriticalsectionandspincount(lpCriticalSection, dwSpinCount), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_releasesemaphore(p0: i32, p1: i32, p2: *mut c_void) -> i32 {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_releasesemaphore(p0, p1, p2), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_setevent(p0: HANDLE) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_setevent(p0), None => () } }
}

#[inline]
pub unsafe fn x4_leavecriticalsection(p0: LPCRITICAL_SECTION) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_leavecriticalsection(p0), None => () } }
}

#[inline]
pub unsafe fn x4_raiseexception(p0: i32, p1: i32, p2: i32, p3: *const c_void) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_raiseexception(p0, p1, p2, p3), None => () } }
}

#[inline]
pub unsafe fn x4_getcurrentthreadid() -> DWORD {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_getcurrentthreadid(), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_waitforsingleobject(hHandle: HANDLE, dwMiliseconds: DWORD) -> u32 {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_waitforsingleobject(hHandle, dwMiliseconds), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_regopenkeyexw(hKey: HKEY, lpSubKey: LPCWSTR, ulOptions: DWORD, samDesired: REGSAM, phkResult: PHKEY) -> LSTATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_regopenkeyexw(hKey, lpSubKey, ulOptions, samDesired, phkResult), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_regclosekey(hKey: HKEY) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_regclosekey(hKey), None => () } }
}

#[inline]
pub unsafe fn x4_regqueryvalueexw(hKey: HKEY, lpValueName: LPCWSTR, lpReserved: LPDWORD, lpType: LPDWORD, lpData: LPBYTE, lpcbData: LPDWORD) -> LSTATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_regqueryvalueexw(hKey, lpValueName, lpReserved, lpType, lpData, lpcbData), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_localalloc(uFlags: UINT, uBytes: SIZE_T) -> HLOCAL {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_localalloc(uFlags, uBytes), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_localfree(hMem: HLOCAL) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_localfree(hMem), None => () } }
}

#[inline]
pub unsafe fn x4_rpcreverttoselfex(BindingHandle: RPC_BINDING_HANDLE) -> RPC_STATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_rpcreverttoselfex(BindingHandle), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_i_rpcmapwin32status(Status: RPC_STATUS) -> HRESULT {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_i_rpcmapwin32status(Status), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_rpcimpersonateclient(BindingHandle: RPC_BINDING_HANDLE) -> RPC_STATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_rpcimpersonateclient(BindingHandle), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_convertstringsidtosidw(stringsid: *const u16, sid: *mut *mut c_void) -> i32 {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_convertstringsidtosidw(stringsid, sid), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_checktokenmembership(tokenhandle: HANDLE, sidtocheck: PSID, isadmin: *mut BOOL) -> BOOL {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_checktokenmembership(tokenhandle, sidtocheck, isadmin), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_freesid(psid: PSID) -> *mut c_void {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_freesid(psid), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_closehandle(hobject: HANDLE) -> BOOL {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_closehandle(hobject), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_waitforsingleobjectex(hhandle: HANDLE, dwmilliseconds: u32, balertable: BOOL) -> u32 {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_waitforsingleobjectex(hhandle, dwmilliseconds, balertable), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_releasemutex(hmutex: HANDLE) -> BOOL {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_releasemutex(hmutex), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_deletecriticalsection(lpcriticalsection: *mut CRITICAL_SECTION) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_deletecriticalsection(lpcriticalsection), None => () } }
}

#[inline]
pub unsafe fn x4_getmodulehandlew(lpmodulename: *const u16) -> HMODULE {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_getmodulehandlew(lpmodulename), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_getprocaddress(hmodule: HMODULE, lpprocname: PCSTR) -> FARPROC {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_getprocaddress(hmodule, lpprocname), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4__dllonexit(func: OnExitCallback, pbegin: *mut *mut PVFV, pend: *mut *mut PVFV) -> *mut core::ffi::c_void {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4__dllonexit(func, pbegin, pend), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_onexit(function: OnExitCallback) -> OnExitT {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_onexit(function), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_createsemaphoreexw(lpsemaphoreattributes: *const SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: *const u16, dwflags: u32, dwdesiredaccess: u32) -> HANDLE {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_createsemaphoreexw(lpsemaphoreattributes, linitialcount, lmaximumcount, lpname, dwflags, dwdesiredaccess), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_opensemaphorew(dwdesiredaccess: u32, binherithandle: BOOL, lpname: *const u16) -> HANDLE {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_opensemaphorew(dwdesiredaccess, binherithandle, lpname), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_initializecriticalsectionex(lpcriticalsection: *mut CRITICAL_SECTION, dwspincount: u32, flags: u32) -> BOOL {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_initializecriticalsectionex(lpcriticalsection, dwspincount, flags), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_setthreadpooltimer(ptmi: PTP_TIMER, pftduetime: *const FILETIME, msperiod: u32, mswindowlength: u32) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_setthreadpooltimer(ptmi, pftduetime, msperiod, mswindowlength), None => () } }
}

#[inline]
pub unsafe fn x4_waitforthreadpooltimercallbacks(ptmi: PTP_TIMER, fcancelpendingcallbacks: BOOL) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_waitforthreadpooltimercallbacks(ptmi, fcancelpendingcallbacks), None => () } }
}

#[inline]
pub unsafe fn x4_closethreadpooltimer(ptmi: PTP_TIMER) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_closethreadpooltimer(ptmi), None => () } }
}

#[inline]
pub unsafe fn x4_createmutexexw(lpmutexattributes: *const SECURITY_ATTRIBUTES, lpname: *const u16, dwflags: u32, dwdesiredaccess: u32) -> HANDLE {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_createmutexexw(lpmutexattributes, lpname, dwflags, dwdesiredaccess), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_entercriticalsection(lpcriticalsection: *mut CRITICAL_SECTION) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_entercriticalsection(lpcriticalsection), None => () } }
}

#[inline]
pub unsafe fn x4_allocateandinitializesid(pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8, nsubauthority0: u32, nsubauthority1: u32, nsubauthority2: u32, nsubauthority3: u32, nsubauthority4: u32, nsubauthority5: u32, nsubauthority6: u32, nsubauthority7: u32, psid: *mut PSID) -> BOOL {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_allocateandinitializesid(pidentifierauthority, nsubauthoritycount, nsubauthority0, nsubauthority1, nsubauthority2, nsubauthority3, nsubauthority4, nsubauthority5, nsubauthority6, nsubauthority7, psid), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_lcmapstringw(locale: u32, dwmapflags: u32, lpsrcstr: *const u16, ccntsrc: i32, lpdeststr: *mut u16, ccntdest: i32) -> i32 {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_lcmapstringw(locale, dwmapflags, lpsrcstr, ccntsrc, lpdeststr, ccntdest), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_bcryptfinishhash(hhash: BCRYPT_HASH_HANDLE, pboutput: *mut u8, cboutput: u32, dwflags: u32) -> NTSTATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_bcryptfinishhash(hhash, pboutput, cboutput, dwflags), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_bcryptdestroyhash(hhash: BCRYPT_HASH_HANDLE) -> NTSTATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_bcryptdestroyhash(hhash), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_bcryptclosealgorithmprovider(halgorithm: BCRYPT_ALG_HANDLE, dwflags: u32) -> NTSTATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_bcryptclosealgorithmprovider(halgorithm, dwflags), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_bcrypthashdata(hhash: BCRYPT_HASH_HANDLE, pbinput: *mut u8, cbinput: u32, dwflags: u32) -> NTSTATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_bcrypthashdata(hhash, pbinput, cbinput, dwflags), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_bcryptopenalgorithmprovider(phalgorithm: *mut BCRYPT_ALG_HANDLE, pszalgid: *const u16, pszimplementation: *const u16, dwflags: u32) -> NTSTATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_bcryptopenalgorithmprovider(phalgorithm, pszalgid, pszimplementation, dwflags), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_bcryptcreatehash(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut BCRYPT_HASH_HANDLE, pbhashobject: *mut u8, cbhashobject: u32, pbsecret: *mut u8, cbsecret: u32, dwflags: u32) -> NTSTATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_bcryptcreatehash(halgorithm, phhash, pbhashobject, cbhashobject, pbsecret, cbsecret, dwflags), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_openthreadtoken(threadhandle: HANDLE, desiredaccess: u32, openaself: BOOL, tokenhandle: *mut HANDLE) -> BOOL {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_openthreadtoken(threadhandle, desiredaccess, openaself, tokenhandle), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_getcurrentthread() -> HANDLE {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_getcurrentthread(), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_rtlquerypackageclaims(token_handle: HANDLE, package_full_name: *mut u16, package_size: *mut usize, app_id: *mut u16, app_id_size: *mut usize, dynamic_id: *mut GUID, pkg_claim: *mut PS_PKG_CLAIM, attributes_present: *mut u64) -> NTSTATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_rtlquerypackageclaims(token_handle, package_full_name, package_size, app_id, app_id_size, dynamic_id, pkg_claim, attributes_present), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4__wcsicmp(string1: *const u16, string2: *const u16) -> i32 {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4__wcsicmp(string1, string2), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_ntcurrentteb() -> *mut TEB {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_ntcurrentteb(), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_ntquerysysteminformation(system_information_class: u32, system_information: *mut c_void, system_information_length: ULONG, return_length: *mut ULONG) -> NTSTATUS {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_ntquerysysteminformation(system_information_class, system_information, system_information_length, return_length), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_unmapviewoffile(lpbaseaddress: *const c_void) -> BOOL {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_unmapviewoffile(lpbaseaddress), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_gettickcount() -> u32 {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_gettickcount(), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_createthreadpooltimer(pfnti: PTP_TIMER_CALLBACK, pv: *mut c_void, pcbe: *const TP_CALLBACK_ENVIRON) -> PTP_TIMER {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_createthreadpooltimer(pfnti, pv, pcbe), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_getmodulehandleexw(dwflags: u32, lpmodulename: *const u16, phmodule: *mut HMODULE) -> BOOL {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_getmodulehandleexw(dwflags, lpmodulename, phmodule), None => core::mem::zeroed() } }
}

#[inline]
pub unsafe fn x4_releasesrwlockshared(srwlock: *mut SRWLOCK) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_releasesrwlockshared(srwlock), None => () } }
}

#[inline]
pub unsafe fn x4_acquiresrwlockshared(srwlock: *mut SRWLOCK) {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_acquiresrwlockshared(srwlock), None => () } }
}

#[inline]
pub unsafe fn x4_queryperformancecounter(lpperformancecount: *mut i64) -> BOOL {
    // SAFETY: callers uphold the platform function's raw-pointer and handle contracts.
    unsafe { match X4_EXTERNAL_API { Some(api) => api.x4_queryperformancecounter(lpperformancecount), None => core::mem::zeroed() } }
}

pub fn x4_sleep(p0: i32) {
    vdebug_autoprefix!("sleep call: {p0:X}")
}

/// Copy bytes without depending on a hosted C runtime.
pub unsafe fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void {
    let dst = dest.cast::<u8>(); let src = src.cast::<u8>();
    for i in 0..count { unsafe { dst.add(i).write(src.add(i).read()); } }
    dest
}

/// Bounds-checked byte copy; returns zero on success and a nonzero status on invalid bounds.
pub unsafe fn memcpy_s(dest: *mut c_void, destsz: usize, src: *const c_void, count: usize) -> i32 {
    if count > destsz || (count != 0 && (dest.is_null() || src.is_null())) { return 22; }
    unsafe { memcpy(dest, src, count); }
    0
}

pub unsafe fn memcmp(a: *const c_void, b: *const c_void, len: usize) -> i32 {
    let (a,b)=(a.cast::<u8>(),b.cast::<u8>());
    for i in 0..len { let (x,y)=unsafe {(a.add(i).read(),b.add(i).read())}; if x!=y { return x as i32-y as i32; } }
    0
}

pub fn towlower(c: u32) -> u32 { if (b'A' as u32..=b'Z' as u32).contains(&c) { c + 32 } else { c } }

pub unsafe fn wcschr(ws: PCWSTR, wchar: WCHAR) -> *mut WCHAR {
    if ws.is_null() { return core::ptr::null_mut(); }
    let mut i=0; loop { let p=unsafe {ws.add(i)}; let c=unsafe {p.read()}; if c==wchar {return p.cast_mut();} if c==0 {return core::ptr::null_mut();} i+=1; }
}

pub unsafe fn wcsncmp(a: PCWSTR, b: PCWSTR, count: SIZE_T) -> i32 {
    for i in 0..count {
        let (x,y)=unsafe {(a.add(i).read(),b.add(i).read())};
        if x!=y {
            return x as i32-y as i32;
        } if x==0 {
            break;
        }
    }
    0
}
