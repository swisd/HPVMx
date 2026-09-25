use core::ffi::c_void;
use crate::{u16_str, vdebug_autoprefix};
use crate::x4::spp::timer::{SppTimerCallbackBaseDefaultPlaceholder, SppTimerCallbackBaseSerialize, SppTimerCallbackDestructor, SppTimerCallbackEvaluateState, SppTimerCallbackInitialize, SppTimerCallbackSerialize, SppTimerCallbackStop};
use crate::x4::types::{ExecutionCoordinator, ShaderManager, LPCVOID, HRESULT, DWORD, BOOL, HANDLE_FLAGS, PCWSTR, ProtectedSubsystemContext, SecureCallbackNode, __int64, PROCESS_CREATION_FLAGS, ISppNamespace, SppThreadContextManager, SppTelemetryContext, SppThreadContextBody, SppCustomLock, _RTL_CRITICAL_SECTION, ThreadMap, ThreadEntry, HANDLE, PRTL_CRITICAL_SECTION_DEBUG, ThreadSlot, MemoryMappedViewEntry, SppTimerCallback, ISppTimerRegistry, SppStringBuilder, SppTimerCallbackBase, SppPacketControlFlags, HKEY};

pub static mut GlobalDXGISwapChainPresentWrapper: i64 = 0;
pub static mut pQueueMgr: ShaderManager = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
pub static mut GlobalExecutionCoordinator: ExecutionCoordinator = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

pub static mut g_DisableBypassCheck: bool = false;
pub static mut g_pfnConditionCheck: u8 = 0;

pub fn _guard_check_icall_fptr() { vdebug_autoprefix!("_guard_check_icall_fptr") }

pub static mut lpSource: LPCVOID = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

pub static mut GlobalPfnPresentPrimary: u64 = 0x000000014046B330; // ADDR UNK
pub static mut GlobalPfnPresentFallback: u64 = 0x000000014046B3A0; // ADDR UNK


pub const E_INVALIDARG: HRESULT = -2147024809; // 0x80070057
pub const INTSAFE_E_ARITHMETIC_OVERFLOW: HRESULT = -2147024362; // 0x80070216
pub const S_OK: HRESULT = 0;
pub const COR_E_OVERFLOW: HRESULT = -2147024362; // 0x80070216 (INTSAFE_E_ARITHMETIC_OVERFLOW)
pub const E_OUTOFMEMORY: HRESULT = -2147024882;  // 0x8007000E

pub const SERVER_PROPS_NAMESPACE: PCWSTR = u16_str!("lsrv:/8/Validation/ServerProps");

pub const STRSAFE_E_INSUFFICIENT_BUFFER: HRESULT = -2147024774; // 0x8007007A

pub const LMEM_ZEROINIT: DWORD = 0x0040;
pub const STRSAFE_MAX_CCH: usize = 2147483646;
pub const MAX_STRING_CCH: usize = 2147483646;
pub const E_NOT_VALID_STATE: HRESULT = -2147024883; // 0x8007000D (ERROR_INVALID_DATA)

pub const MAX_BUILDER_CAPACITY: u32 = 0x2000000; // 32 MB / WCHARs

// L"%08lx-%04x-%04x-%02x%02x-%02x%02x%02x%02x%02x%02x"
pub const GUID_FORMAT_STR: &[u16] = &[
    0x0025, 0x0030, 0x0038, 0x006C, 0x0078, 0x002D, // %08lx-
    0x0025, 0x0030, 0x0034, 0x0078, 0x002D,         // %04x-
    0x0025, 0x0030, 0x0034, 0x0078, 0x002D,         // %04x-
    0x0025, 0x0030, 0x0032, 0x0078,                 // %02x
    0x0025, 0x0030, 0x0032, 0x0078, 0x002D,         // %02x-
    0x0025, 0x0030, 0x0032, 0x0078,                 // %02x
    0x0025, 0x0030, 0x0032, 0x0078,                 // %02x
    0x0025, 0x0030, 0x0032, 0x0078,                 // %02x
    0x0025, 0x0030, 0x0032, 0x0078,                 // %02x
    0x0025, 0x0030, 0x0032, 0x0078,                 // %02x
    0x0025, 0x0030, 0x0032, 0x0078, 0x0000,         // %02x\0
];

pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;

pub const ERROR_HANDLE_EOF: HRESULT = -2147024785; // 0x80070026

pub static mut BreakCode: i32 = 0; //-1 = enabled/all, 0=none, any code = code

pub const SPP_E_ACCESS_DENIED: i32 = -1073418203;

pub const HKEY_LOCAL_MACHINE: HKEY = HKEY { unused: 0 };

unsafe extern "C" {
    pub static mut GlobalNullSchemaMetadata: c_void;
    pub static mut GlobalSchema_ActConfigId_Flags: i32;
    pub static mut dword_14045F310: i32;
    pub static mut GlobalSchema_RootMetadata: *mut c_void;
    pub static mut GlobalSchema_RootFactoryFn: *mut c_void;
    pub static mut GlobalSchema_RootReservedField: i32;
    pub static mut GlobalSchema_RootElementName: *const u16;
    pub static mut dword_14045F328: i32;
    pub static mut GlobalSchema_RootContainerName: *const u16;
    pub static mut GlobalSchema_ConfigurationsNodeName: *const u16;
    pub static mut dword_14045F32C: i32;
    pub static mut GlobalSchema_ConfigurationChildName: *const u16;
    pub static mut GlobalSchema_ActConfigId_ParentNodeName: *const u16;
    pub static mut GlobalSchema_KeyRangesListNodeName: *const u16;
    pub static mut GlobalSchema_ActConfigId_AttributeName: *const u16;
    pub static mut GlobalSchema_ActConfigId_DupString: *const u16;
    pub static mut GlobalSchema_RefGroupId_AttributeName: *const u16;
    pub static mut GlobalSchema_RefGroupId_DupString: *const u16;
    pub static mut GlobalSchema_EditionId_AttributeName: *const u16;
    pub static mut GlobalSchema_EditionId_DupString: *const u16;
    pub static mut GlobalSchema_ProductDesc_AttributeName: *const u16;
    pub static mut GlobalSchema_ProductDesc_DupString: *const u16;
    pub static mut GlobalSchema_KeyType_AttributeName: *const u16;
    pub static mut GlobalSchema_KeyType_DupString: *const u16;
    pub static mut GlobalSchema_IsRandomized_AttributeName: *const u16;
    pub static mut GlobalSchema_IsRandomized_DupString: *const u16;
    pub static mut dword_14045F348: i32;
    pub static mut qword_14045F350: *mut c_void;
    pub static mut qword_14045F358: i64;
    pub static mut GlobalSchema_ActConfigId_Metadata: *mut c_void;
    pub static mut dword_14045F380: i32;
    pub static mut GlobalSchema_ActConfigId_ParserFn: *mut c_void;
    pub static mut GlobalSchema_ActConfigId_Reserved1: i32;
    pub static mut GlobalSchema_ActConfigId_Reserved2: i32;
    pub static mut GlobalSchema_RefGroupId_Metadata: *mut c_void;
    pub static mut dword_14045F3B8: i32;
    pub static mut GlobalSchema_RefGroupId_ParserFn: *mut c_void;
    pub static mut GlobalSchema_RefGroupId_Reserved1: i32;
    pub static mut GlobalSchema_RefGroupId_Reserved2: i32;
    pub static mut GlobalSchema_EditionId_Metadata: *mut c_void;
    pub static mut dword_14045F3F0: i32;
    pub static mut GlobalSchema_EditionId_ParserFn: *mut c_void;
    pub static mut GlobalSchema_EditionId_Reserved1: i32;
    pub static mut GlobalSchema_EditionId_Reserved2: i32;
    pub static mut GlobalSchema_ProductDesc_Metadata: *mut c_void;
    pub static mut dword_14045F428: i32;
    pub static mut GlobalSchema_ProductDesc_ParserFn: *mut c_void;
    pub static mut GlobalSchema_ProductDesc_Reserved1: i32;
    pub static mut GlobalSchema_ProductDesc_Reserved2: i32;
    pub static mut GlobalSchema_KeyType_Metadata: *mut c_void;
    pub static mut dword_14045F460: i32;
    pub static mut GlobalSchema_KeyType_ParserFn: *mut c_void;
    pub static mut GlobalSchema_KeyType_Reserved1: i32;
    pub static mut GlobalSchema_KeyType_Reserved2: i32;
    pub static mut GlobalSchema_IsRandomized_Metadata: *mut c_void;
    pub static mut dword_14045F498: i32;
    pub static mut GlobalSchema_IsRandomized_FactoryFn: *mut c_void;
    pub static mut GlobalSchema_IsRandomized_Reserved1: i32;
    pub static mut GlobalSchema_IsRandomized_Reserved2: i32;
    pub static mut GlobalSchema_KeyRanges_Metadata1: *mut c_void;
    pub static mut GlobalSchema_KeyRanges_Metadata2: *mut c_void;
    pub static mut dword_14045F4D0: i32;
    pub static mut GlobalSchema_KeyRanges_FactoryFn: *mut c_void;
    pub static mut GlobalSchema_KeyRanges_Reserved1: i32;
    pub static mut GlobalSchema_KeyRanges_NodeName: *const u16;
    pub static mut GlobalSchema_KeyRanges_ChildName: *const u16;
    pub static mut GlobalSchema_KeyRange_DupChildName: *const u16;
    pub static mut GlobalSchema_KeyRange_RefActConfigId_ParserFn: *mut c_void;
    pub static mut GlobalSchema_KeyRange_RefActConfigId_Name: *const u16;
    pub static mut GlobalSchema_KeyRange_RefActConfigId_DupString: *const u16;
    pub static mut GlobalSchema_KeyRange_FactoryFn_Variant: *mut c_void;
    pub static mut GlobalSchema_KeyRange_EulaType_Name: *const u16;
    pub static mut GlobalSchema_KeyRange_EulaType_DupString: *const u16;
    pub static mut GlobalSchema_KeyRange_IsValid_Name: *const u16;
    pub static mut GlobalSchema_KeyRange_IsValid_DupString: *const u16;
    pub static mut GlobalSchema_KeyRange_Start_Name: *const u16;
    pub static mut GlobalSchema_KeyRange_Start_DupString: *const u16;
    pub static mut GlobalSchema_KeyRange_End_Name: *const u16;
    pub static mut GlobalSchema_KeyRange_End_DupString: *const u16;
    pub static mut GlobalSchema_KeyRange_IsValid_FactoryFn: *mut c_void;
    pub static mut GlobalSchema_Crypto_PublicKeyChildNode: *const u16;
    pub static mut GlobalSchema_Crypto_DupPublicKeyChildNode: *const u16;
    pub static mut dword_14045F4E8: i32;
    pub static mut dword_14045F4EC: i32;
    pub static mut GlobalSchema_KeyRange_PublicKeysContainerName: *const u16;
    pub static mut dword_14045F508: i32;
    pub static mut GlobalSchema_KeyRange_FactoryFn: *mut c_void;
    pub static mut qword_14045F518: i64;
    pub static mut qword_14045F520: i64;
    pub static mut GlobalSchema_KeyRange_RefActConfigId_Metadata: *mut c_void;
    pub static mut dword_14045F540: i32;
    pub static mut qword_14045F550: i64;
    pub static mut qword_14045F558: i64;
    pub static mut GlobalSchema_KeyRange_PartNumber_Metadata: *mut c_void;
    pub static mut GlobalSchema_KeyRange_PartNumber_Name: *const u16;
    pub static mut dword_14045F578: i32;
    pub static mut qword_14045F588: i64;
    pub static mut qword_14045F590: i64;
    pub static mut GlobalSchema_KeyRange_PartNumber_DupString: *const u16;
    pub static mut GlobalSchema_KeyRange_EulaType_Metadata: *mut c_void;
    pub static mut dword_14045F5B0: i32;
    pub static mut GlobalSchema_KeyRange_EulaType_ParserFn: *mut c_void;
    pub static mut qword_14045F5C0: i64;
    pub static mut dword_14045F5C8: i32;
    pub static mut dword_14045F5CC: i32;
    pub static mut GlobalSchema_KeyRange_IsValid_Metadata: *mut c_void;
    pub static mut dword_14045F5E8: i32;
    pub static mut qword_14045F5F8: i64;
    pub static mut qword_14045F600: i64;
    pub static mut GlobalSchema_KeyRange_Start_Metadata: *mut c_void;
    pub static mut dword_14045F620: i32;
    pub static mut GlobalSchema_KeyRange_Start_ParserFn: *mut c_void;
    pub static mut qword_14045F630: i64;
    pub static mut qword_14045F638: i64;
    pub static mut GlobalSchema_KeyRange_End_Metadata: *mut c_void;
    pub static mut dword_14045F658: i32;
    pub static mut GlobalSchema_KeyRange_End_ParserFn: *mut c_void;
    pub static mut qword_14045F668: i64;
    pub static mut qword_14045F670: i64;
    pub static mut GlobalSchema_Crypto_Metadata1: *mut c_void;
    pub static mut GlobalSchema_Crypto_Metadata2: *mut c_void;
    pub static mut dword_14045F690: i32;
    pub static mut GlobalSchema_Crypto_FactoryFn: *mut c_void;
    pub static mut qword_14045F6A0: i64;
    pub static mut dword_14045F6A8: i32;
    pub static mut dword_14045F6AC: i32;
    pub static mut GlobalSchema_Crypto_PublicKeysNode: *const u16;
    pub static mut GlobalSchema_Crypto_OtherInfoNodeName: *const u16;
    pub static mut dword_14045F6C8: i32;
    pub static mut GlobalSchema_Crypto_OtherInfoFactoryFn: *mut c_void;
    pub static mut qword_14045F6D8: i64;
    pub static mut qword_14045F6E0: i64;
    pub static mut GlobalSchema_Crypto_GroupId_Name: *const u16;
    pub static mut GlobalSchema_Crypto_GroupId_DupString: *const u16;
    pub static mut GlobalSchema_Crypto_AlgorithmId_Name: *const u16;
    pub static mut GlobalSchema_Crypto_AlgorithmId_DupString: *const u16;
    pub static mut GlobalSchema_Crypto_PublicKeyValue_FactoryFn: *mut c_void;
    pub static mut GlobalSchema_Ext_ItemChildNode: *const u16;
    pub static mut GlobalSchema_Ext_DupItemChildNode: *const u16;
    pub static mut GlobalSchema_Ext_NameField: *const u16;
    pub static mut GlobalSchema_Ext_DupNameField: *const u16;
    pub static mut GlobalSchema_Crypto_PublicKeyValue_Name: *const u16;
    pub static mut GlobalSchema_Crypto_PublicKeyValue_DupString: *const u16;
    pub static mut GlobalSchema_Ext_ValueField: *const u16;
    pub static mut GlobalSchema_Ext_DupValueField: *const u16;
    pub static mut GlobalSchema_Crypto_GroupId_Metadata: *mut c_void;
    pub static mut dword_14045F700: i32;
    pub static mut GlobalSchema_Crypto_GroupId_ParserFn: *mut c_void;
    pub static mut qword_14045F710: i64;
    pub static mut qword_14045F718: i64;
    pub static mut qword_14045F728: *mut c_void;
    pub static mut dword_14045F738: i32;
    pub static mut GlobalSchema_Crypto_AlgorithmId_ParserFn: *mut c_void;
    pub static mut qword_14045F748: i64;
    pub static mut qword_14045F750: i64;
    pub static mut GlobalSchema_Crypto_PublicKeyValue_Metadata: *mut c_void;
    pub static mut dword_14045F770: i32;
    pub static mut qword_14045F780: i64;
    pub static mut qword_14045F788: i64;
    pub static mut GlobalSchema_Ext_Metadata1: *mut c_void;
    pub static mut GlobalSchema_Ext_Metadata2: *mut c_void;
    pub static mut dword_14045F7A8: i32;
    pub static mut GlobalSchema_Ext_FactoryFn: *mut c_void;
    pub static mut qword_14045F7B8: i64;
    pub static mut dword_14045F7C0: i32;
    pub static mut dword_14045F7C4: i32;
    pub static mut GlobalSchema_Ext_OtherInfoNode: *const u16;
    pub static mut GlobalSchema_Ext_Item_Metadata: *mut c_void;
    pub static mut dword_14045F7E0: i32;
    pub static mut GlobalSchema_Ext_ItemFactoryFn: *mut c_void;
    pub static mut qword_14045F7F0: i64;
    pub static mut qword_14045F7F8: i64;
    pub static mut GlobalSchema_Ext_Name_Metadata: *mut c_void;
    pub static mut dword_14045F818: i32;
    pub static mut GlobalSchema_Ext_Name_ParserFn: *mut c_void;
    pub static mut qword_14045F828: i64;
    pub static mut dword_14045F830: i32;
    pub static mut dword_14045F834: i32;
    pub static mut GlobalSchema_Ext_Value_Metadata: *mut c_void;
    pub static mut dword_14045F850: i32;
    pub static mut GlobalSchema_Ext_Value_ParserFn: *mut c_void;
    pub static mut qword_14045F860: i64;
    pub static mut dword_14045F868: i32;
    pub static mut dword_14045F86C: i32;
    pub static mut GlobalSchema_Ext_EndBoundary_Metadata1: *mut c_void;
    pub static mut GlobalSchema_Ext_EndBoundary_Metadata2: *mut c_void;
    pub static mut dword_14045F888: i32;
}

pub const HANDLE_FLAG_INHERIT: HANDLE_FLAGS = 1u32;
pub const HANDLE_FLAG_PROTECT_FROM_CLOSE: HANDLE_FLAGS = 2u32;
pub const HIGH_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 128u32;
pub const IDLE_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 64u32;

pub static mut pContext: ProtectedSubsystemContext = ProtectedSubsystemContext {
    allocated_capacity: 0,
    current_element_count: 0,
    pMappedViewsArray: MemoryMappedViewEntry {
        pMappedBaseAddress: c_void,
        hFileMappingObject: c_void
    } as *mut MemoryMappedViewEntry,
    obfuscated_state_flags: 0,
};
pub static mut GlobalSecureRegistrationNode: SecureCallbackNode = SecureCallbackNode {
    blink: SecureCallbackNode {
        blink: (),
        pEncodedInitFunc: (),
        pEncodedShutdownFunc: (),
        registration_flags: 0,
    },
    pEncodedInitFunc: SecureCallbackNode {
        blink: (),
        pEncodedInitFunc: (),
        pEncodedShutdownFunc: (),
        registration_flags: 0,
    },
    pEncodedShutdownFunc: SecureCallbackNode {
        blink: (),
        pEncodedInitFunc: (),
        pEncodedShutdownFunc: (),
        registration_flags: 0,
    },
    registration_flags: 0,
};
pub static mut GlobalMasterCallbackListHead: __int64 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
pub static mut GlobalSecureRegistrationNode_pEncodedInitFunc: __int64 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
pub static mut GlobalSecureRegistrationNode_registration_flags: i64 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
pub static mut GlobalSecureRegistrationNode_pEncodedShutdownFunc: __int64 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

pub  static mut GlobalEtwEventRegister: __int64 = 0x0000004060AA0D010;

pub static mut GlobalPtrSppNamespace: ISppNamespace = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
pub static mut pManager: SppThreadContextManager = SppThreadContextManager {
    padding: [0i8;72],
    body: SppThreadContextBody {
        unknownPtr: c_void,
        globalContext: c_void,
        lock: SppCustomLock {
            padding: [0i8;8],
            internalLock: _RTL_CRITICAL_SECTION {
                DebugInfo: PRTL_CRITICAL_SECTION_DEBUG{},
                LockCount: 0,
                RecursionCount: 0,
                OwningThread: HANDLE {},
                LockSemaphore: HANDLE {},
                SpinCount: 0,
            },
            hEvent: HANDLE {},
            activeWritersCount: 0,
            queuedWaitersCount: 0,
            isWriterMode: 0,
            unknownFlag: 0,
            owningThreadId: 0,
            recursionCount: 0,
            pThreadSlotsArray: ThreadSlot {
                threadId: 0,
                slotRecursionCount: 0,
            },
            maxSlotsCount: 0,
            inlineSlot: ThreadSlot {
                threadId: 0,
                slotRecursionCount: 0,
            },
        },
        alignmentCushion: 0,
        threadIdMap: ThreadMap {
            capacity: 0,
            elementCount: 0,
            array: ThreadEntry {
                threadId: 0,
                padding: 0,
                context: c_void,
            },
        },
        threadContextArray: c_void,
        trailingPadding: [0i8;8],
    },
};
pub static mut GlobalPtrTelemetryContext: SppTelemetryContext = SppTelemetryContext {
    initializationState: 0,
    padding: [0i8;7],
    hEtwRegistration: 0,
};

type SppTimerCallbackDestructorFn = unsafe fn(*mut c_void, u8) -> *mut c_void;
type SppTimerCallbackSerializeFn = unsafe fn(*const u32, *mut SppStringBuilder) -> u64;
type SppTimerCallbackStopFn = unsafe fn(usize, usize) -> u64;
type SppTimerCallbackInitializeFn = unsafe fn(*mut SppTimerCallback, *mut ISppTimerRegistry) -> HRESULT;
type SppTimerCallbackEvaluateStateFn = unsafe fn(*mut SppTimerCallback, *mut c_void) -> HRESULT;
type SppTimerCallbackBaseSerializeFn = unsafe fn(*mut SppTimerCallbackBase, *mut SppStringBuilder) -> HRESULT;
type SppTimerCallbackBaseDefaultPlaceholderFn = unsafe fn() -> HRESULT;

// 2. Define the Vtbl structures matching the exact layout layout (dq = 64-bit pointers)
#[repr(C)]
pub struct GlobalSppTimerCallbackVtbl {
    pub destructor: SppTimerCallbackDestructorFn,
    pub serialize: SppTimerCallbackSerializeFn,
    pub stop: SppTimerCallbackStopFn,
    pub initialize: SppTimerCallbackInitializeFn,
    pub evaluate_state: SppTimerCallbackEvaluateStateFn,
}

#[repr(C)]
pub struct GlobalSppTimerCallbackBaseVtbl {
    pub destructor: SppTimerCallbackDestructorFn,
    pub base_serialize: SppTimerCallbackBaseSerializeFn,
    pub placeholder1: SppTimerCallbackBaseDefaultPlaceholderFn,
    pub placeholder2: SppTimerCallbackBaseDefaultPlaceholderFn,
    pub placeholder3: SppTimerCallbackBaseDefaultPlaceholderFn,
}


pub static GlobalSppTimerCallbackVtbl: GlobalSppTimerCallbackVtbl = GlobalSppTimerCallbackVtbl {
    destructor: SppTimerCallbackDestructor,
    serialize: SppTimerCallbackSerialize,
    stop: SppTimerCallbackStop,
    initialize: SppTimerCallbackInitialize,
    evaluate_state: SppTimerCallbackEvaluateState,
};


pub static GlobalSppTimerCallbackBaseVtbl: GlobalSppTimerCallbackBaseVtbl = GlobalSppTimerCallbackBaseVtbl {
    destructor: SppTimerCallbackDestructor, // Reuses the same destructor pointer as seen in disassembly
    base_serialize: SppTimerCallbackBaseSerialize,
    placeholder1: SppTimerCallbackBaseDefaultPlaceholder,
    placeholder2: SppTimerCallbackBaseDefaultPlaceholder,
    placeholder3: SppTimerCallbackBaseDefaultPlaceholder,
};

pub static GlobalSppPacketControlFlags: SppPacketControlFlags = [0,0];