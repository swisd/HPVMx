use core::ffi::{c_void, VaList};
use crate::vdebug_autoprefix;
use crate::x4::rawasm::FARPROC;
use crate::x4::types::{__int64, DWORD, HANDLE, LPVOID, PSRWLOCK, PVOID, PCWSTR, PWSTR, SIZE_T, LPCRITICAL_SECTION, BOOL, CRITICAL_SECTION, HKEY, LPCWSTR, REGSAM, PHKEY, LPDWORD, LPBYTE, HLOCAL, UINT, LSTATUS, WCHAR, HRESULT, PSID, HMODULE, PCSTR, SECURITY_ATTRIBUTES, PTP_TIMER, FILETIME, SID_IDENTIFIER_AUTHORITY, BCRYPT_HASH_HANDLE, NTSTATUS, BCRYPT_ALG_HANDLE, GUID, SRWLOCK};

unsafe extern "C" {
    pub fn AcquireSRWLockExclusive(SRWLock: PSRWLOCK);
    pub fn ReleaseSRWLockExclusive(SRWLock: PSRWLOCK);
    pub fn GetProcessHeap() -> HANDLE;
    pub fn HeapFree(hHeap: HANDLE, dwFlags: DWORD, lpMem: LPVOID);
    pub fn EtwWriteTransfer(a1: __int64, a2: __int64, a3: __int64) -> i32;
    pub fn GetCurrentProcessId() -> DWORD;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
    pub fn memcpy_s(dest: *mut c_void,destsz: usize,src: *const c_void,count: usize) -> i32;
    pub fn towlower(c: u32) -> u32;
    pub fn _vsnwprintf(buffer: PWSTR,count: usize,format: PCWSTR,argptr: VaList) -> i32;
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32;
    pub fn EncodePointer(Ptr: PVOID) -> PVOID;
    pub fn HeapAlloc(hHeap: HANDLE, dwFlags: DWORD, dwBytes: SIZE_T) -> LPVOID;
    pub fn GetLastError() -> DWORD;
    pub fn SetLastError(dwErrorCode: DWORD);
    pub fn InitializeCriticalSectionAndSpinCount(lpCriticalSection: LPCRITICAL_SECTION, dwSpinCount: DWORD) -> BOOL;
    pub fn ReleaseSemaphore(p0: i32, p1: i32, p2: *mut c_void) -> i32;
    pub fn SetEvent(p0: HANDLE);
    pub fn LeaveCriticalSection(p0: LPCRITICAL_SECTION);
    pub fn RaiseException(p0: i32, p1: i32, p2: i32, p3: *const c_void);
    pub fn GetCurrentThreadId() -> DWORD;
    pub fn WaitForSingleObject(hHandle: HANDLE, dwMiliseconds: DWORD) -> u32;
    pub fn RegOpenKeyExW(hKey: HKEY, lpSubKey: LPCWSTR, ulOptions: DWORD, samDesired: REGSAM, phkResult: PHKEY) -> LSTATUS;
    pub fn RegCloseKey(hKey: HKEY);
    pub fn RegQueryValueExW(hKey: HKEY, lpValueName: LPCWSTR, lpReserved: LPDWORD, lpType: LPDWORD, lpData: LPBYTE, lpcbData: LPDWORD) -> LSTATUS;
    pub fn LocalAlloc(uFlags: UINT, uBytes: SIZE_T) -> HLOCAL;
    pub fn LocalFree(hMem: HLOCAL);
    pub fn wcschr(ws: PCWSTR, wchar: WCHAR) -> *mut WCHAR;
    pub fn wcsncmp(string1: PCWSTR, string2: PCWSTR, count: SIZE_T) -> i32;
    pub fn RpcRevertToSelfEx(BindingHandle: RPC_BINDING_HANDLE ) -> RPC_STATUS;
    pub fn I_RpcMapWin32Status(Status: RPC_STATUS) -> HRESULT;
    pub fn RpcImpersonateClient(BindingHandle: RPC_BINDING_HANDLE) -> RPC_STATUS;
    pub fn ConvertStringSidToSidW(stringsid: *const u16, sid: *mut *mut c_void) -> i32;
    pub fn CheckTokenMembership(tokenhandle: HANDLE,sidtocheck: PSID,isadmin: *mut BOOL) -> BOOL;
    pub fn FreeSid(psid: PSID) -> *mut c_void;
    pub fn CloseHandle(hobject: HANDLE) -> BOOL;
    pub fn WaitForSingleObjectEx(hhandle: HANDLE,dwmilliseconds: u32,balertable: BOOL,) -> u32;
    pub fn ReleaseMutex(hmutex: HANDLE,) -> BOOL;
    pub fn DeleteCriticalSection(lpcriticalsection: *mut CRITICAL_SECTION);
    pub fn GetModuleHandleW(lpmodulename: *const u16) -> HMODULE;
    pub fn GetProcAddress(hmodule: HMODULE, lpprocname: PCSTR,) -> FARPROC;
    pub fn _dllonexit(
        func: OnExitCallback,
        pbegin: *mut *mut PVFV,
        pend: *mut *mut PVFV,
    ) -> *mut core::ffi::c_void;
    pub fn onexit(
        function: OnExitCallback,
    ) -> OnExitT;
    pub fn CreateSemaphoreExW(
        lpsemaphoreattributes: *const SECURITY_ATTRIBUTES,
        linitialcount: i32,
        lmaximumcount: i32,
        lpname: *const u16,
        dwflags: u32,
        dwdesiredaccess: u32,
    ) -> HANDLE;
    pub fn OpenSemaphoreW(
        dwdesiredaccess: u32,
        binherithandle: BOOL,
        lpname: *const u16,
    ) -> HANDLE;
    pub fn InitializeCriticalSectionEx(
        lpcriticalsection: *mut CRITICAL_SECTION,
        dwspincount: u32,
        flags: u32,
    ) -> BOOL;
    pub fn SetThreadpoolTimer(
        ptmi: PTP_TIMER,
        pftduetime: *const FILETIME,
        msperiod: u32,
        mswindowlength: u32,
    );
    pub fn WaitForThreadpoolTimerCallbacks(
        ptmi: PTP_TIMER,
        fcancelpendingcallbacks: BOOL,
    );
    pub fn CloseThreadpoolTimer(
        ptmi: PTP_TIMER,
    );
    pub fn CreateMutexExW(
        lpmutexattributes: *const SECURITY_ATTRIBUTES,
        lpname: *const u16,
        dwflags: u32,
        dwdesiredaccess: u32,
    ) -> HANDLE;
    pub fn EnterCriticalSection(
        lpcriticalsection: *mut CRITICAL_SECTION,
    );
    pub fn AllocateAndInitializeSid(
        pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY,
        nsubauthoritycount: u8,
        nsubauthority0: u32,
        nsubauthority1: u32,
        nsubauthority2: u32,
        nsubauthority3: u32,
        nsubauthority4: u32,
        nsubauthority5: u32,
        nsubauthority6: u32,
        nsubauthority7: u32,
        psid: *mut PSID,
    ) -> BOOL;
    pub fn LCMapStringW(
        locale: u32,
        dwmapflags: u32,
        lpsrcstr: *const u16,
        ccntsrc: i32,
        lpdeststr: *mut u16,
        ccntdest: i32,
    ) -> i32;
    pub fn BCryptFinishHash(
        hhash: BCRYPT_HASH_HANDLE,
        pboutput: *mut u8,
        cboutput: u32,
        dwflags: u32,
    ) -> NTSTATUS;
    pub fn BCryptDestroyHash(
        hhash: BCRYPT_HASH_HANDLE,
    ) -> NTSTATUS;
    pub fn BCryptCloseAlgorithmProvider(
        halgorithm: BCRYPT_ALG_HANDLE,
        dwflags: u32,
    ) -> NTSTATUS;
    pub fn BCryptHashData(
    hhash: BCRYPT_HASH_HANDLE,
    pbinput: *mut u8,
    cbinput: u32,
    dwflags: u32,
) -> NTSTATUS;
    pub fn BCryptOpenAlgorithmProvider(
        phalgorithm: *mut BCRYPT_ALG_HANDLE,
        pszalgid: *const u16,
        pszimplementation: *const u16,
        dwflags: u32,
    ) -> NTSTATUS;
    pub fn BCryptCreateHash(
        halgorithm: BCRYPT_ALG_HANDLE,
        phhash: *mut BCRYPT_HASH_HANDLE,
        pbhashobject: *mut u8,
        cbhashobject: u32,
        pbsecret: *mut u8,
        cbsecret: u32,
        dwflags: u32,
    ) -> NTSTATUS;
    pub fn OpenThreadToken(
        threadhandle: HANDLE,
        desiredaccess: u32,
        openaself: BOOL,
        tokenhandle: *mut HANDLE,
    ) -> BOOL;
    pub fn GetCurrentThread() -> HANDLE;
    pub fn RtlQueryPackageClaims(
        token_handle: HANDLE,
        package_full_name: *mut u16,
        package_size: *mut usize,
        app_id: *mut u16,
        app_id_size: *mut usize,
        dynamic_id: *mut GUID,
        pkg_claim: *mut PS_PKG_CLAIM,
        attributes_present: *mut u64,
    ) -> NTSTATUS;
    pub fn _wcsicmp(
        string1: *const u16,
        string2: *const u16,
    ) -> i32;
    pub fn NtCurrentTeb() -> *mut TEB;
    pub fn NtQuerySystemInformation(
        system_information_class: u32,
        system_information: *mut c_void,
        system_information_length: ULONG,
        return_length: *mut ULONG,
    ) -> NTSTATUS;
    pub fn UnmapViewOfFile(
        lpbaseaddress: *const c_void,
    ) -> BOOL;
    pub fn GetTickCount() -> u32;
    pub fn CreateThreadpoolTimer(
        pfnti: PTP_TIMER_CALLBACK,
        pv: *mut c_void,
        pcbe: *const TP_CALLBACK_ENVIRON,
    ) -> PTP_TIMER;
    pub fn GetModuleHandleExW(
        dwflags: u32,
        lpmodulename: *const u16,
        phmodule: *mut HMODULE,
    ) -> BOOL;
    pub fn ReleaseSRWLockShared(
        srwlock: *mut SRWLOCK,
    );
    pub fn AcquireSRWLockShared(
        srwlock: *mut SRWLOCK,
    );
    pub fn QueryPerformanceCounter(
        lpperformancecount: *mut i64,
    ) -> BOOL;
}

pub fn Sleep(p0: i32) {
    vdebug_autoprefix!("sleep call: {p0:X}")
}
