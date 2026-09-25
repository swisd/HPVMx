use alloc::vec::Vec;
use core::ffi::c_void;
use core::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{x4_getprocessheap, x4_heapalloc, x4_heapfree, x4_ntcurrentteb};
use crate::x4::globals::{qword_14045F780, GlobalSchema_Ext_Metadata1, qword_14045F788, GlobalSchema_Ext_Metadata2,
                         dword_14045F7A8, GlobalSchema_Ext_FactoryFn, GlobalSchema_Ext_EndBoundary_Metadata2,
                         GlobalSchema_Ext_EndBoundary_Metadata1, GlobalSchema_Ext_Value_ParserFn,
                         GlobalSchema_Ext_Value_Metadata, GlobalSchema_Ext_Name_ParserFn, GlobalSchema_Ext_Name_Metadata,
                         GlobalSchema_Ext_ItemFactoryFn, GlobalSchema_Ext_Item_Metadata, GlobalSchema_Ext_OtherInfoNode,
                         GlobalNullSchemaMetadata, GlobalSchema_Crypto_PublicKeyValue_FactoryFn,
                         GlobalSchema_Ext_ItemChildNode, GlobalSchema_Ext_DupItemChildNode, _guard_check_icall_fptr,
                         GlobalSchema_RootMetadata, GlobalSchema_RootFactoryFn, GlobalSchema_RootReservedField,
                         GlobalSchema_ActConfigId_Flags, dword_14045F310, GlobalSchema_RootElementName, dword_14045F328,
                         GlobalSchema_RootContainerName, GlobalSchema_ConfigurationsNodeName, dword_14045F32C,
                         GlobalSchema_ConfigurationChildName, GlobalSchema_ActConfigId_ParentNodeName, GlobalSchema_KeyRangesListNodeName,
                         GlobalSchema_ActConfigId_AttributeName, GlobalSchema_ActConfigId_DupString,
                         GlobalSchema_RefGroupId_AttributeName, GlobalSchema_RefGroupId_DupString,
                         GlobalSchema_EditionId_AttributeName, GlobalSchema_EditionId_DupString,
                         GlobalSchema_ProductDesc_AttributeName, GlobalSchema_ProductDesc_DupString,
                         GlobalSchema_KeyType_AttributeName, GlobalSchema_KeyType_DupString, GlobalSchema_IsRandomized_AttributeName,
                         GlobalSchema_IsRandomized_DupString, dword_14045F348, qword_14045F350, qword_14045F358,
                         GlobalSchema_ActConfigId_Metadata, dword_14045F380, GlobalSchema_ActConfigId_ParserFn,
                         GlobalSchema_ActConfigId_Reserved1, GlobalSchema_ActConfigId_Reserved2, GlobalSchema_RefGroupId_Metadata,
                         dword_14045F3B8, GlobalSchema_RefGroupId_ParserFn, GlobalSchema_RefGroupId_Reserved1,
                         GlobalSchema_RefGroupId_Reserved2, GlobalSchema_EditionId_Metadata, dword_14045F3F0,
                         GlobalSchema_EditionId_ParserFn, GlobalSchema_EditionId_Reserved1, GlobalSchema_EditionId_Reserved2,
                         GlobalSchema_ProductDesc_Metadata, dword_14045F428, GlobalSchema_ProductDesc_ParserFn,
                         GlobalSchema_ProductDesc_Reserved1, GlobalSchema_ProductDesc_Reserved2, GlobalSchema_KeyType_Metadata,
                         dword_14045F460, GlobalSchema_KeyType_ParserFn, GlobalSchema_KeyType_Reserved1,
                         GlobalSchema_KeyType_Reserved2, GlobalSchema_IsRandomized_Metadata, dword_14045F498,
                         GlobalSchema_IsRandomized_FactoryFn, GlobalSchema_IsRandomized_Reserved1,
                         GlobalSchema_IsRandomized_Reserved2, GlobalSchema_KeyRanges_Metadata1, GlobalSchema_KeyRanges_Metadata2,
                         dword_14045F888, dword_14045F86C, qword_14045F860, dword_14045F868, dword_14045F850,
                         dword_14045F834, dword_14045F830, qword_14045F828, dword_14045F818, qword_14045F7F8,
                         qword_14045F7F0, dword_14045F7E0, dword_14045F7C4, dword_14045F7C0, qword_14045F7B8,
                         dword_14045F770, GlobalSchema_Crypto_PublicKeyValue_Metadata, qword_14045F750, qword_14045F748,
                         GlobalSchema_Crypto_AlgorithmId_ParserFn, dword_14045F738, qword_14045F728, qword_14045F718,
                         qword_14045F710, GlobalSchema_Crypto_GroupId_ParserFn, dword_14045F700, GlobalSchema_Crypto_GroupId_Metadata,
                         GlobalSchema_Ext_DupValueField, GlobalSchema_Ext_ValueField, GlobalSchema_Crypto_PublicKeyValue_DupString,
                         GlobalSchema_Crypto_PublicKeyValue_Name, GlobalSchema_Ext_DupNameField, GlobalSchema_Ext_NameField,
                         GlobalSchema_Crypto_OtherInfoFactoryFn, GlobalSchema_Crypto_OtherInfoNodeName, GlobalSchema_Crypto_AlgorithmId_DupString,
                         GlobalSchema_Crypto_AlgorithmId_Name, GlobalSchema_Crypto_GroupId_DupString, GlobalSchema_Crypto_GroupId_Name,
                         qword_14045F6E0, qword_14045F6D8, dword_14045F6C8, dword_14045F6AC, dword_14045F6A8,
                         GlobalSchema_Crypto_PublicKeysNode, qword_14045F6A0, GlobalSchema_Crypto_FactoryFn, dword_14045F690,
                         GlobalSchema_Crypto_Metadata2, GlobalSchema_Crypto_Metadata1, qword_14045F670, qword_14045F668,
                         GlobalSchema_KeyRange_End_ParserFn, dword_14045F658, GlobalSchema_KeyRange_End_Metadata, dword_14045F4D0,
                         GlobalSchema_KeyRanges_FactoryFn, GlobalSchema_KeyRanges_Reserved1, GlobalSchema_KeyRanges_NodeName,
                         GlobalSchema_KeyRanges_ChildName, GlobalSchema_KeyRange_DupChildName, GlobalSchema_KeyRange_RefActConfigId_ParserFn,
                         GlobalSchema_KeyRange_RefActConfigId_Name, GlobalSchema_KeyRange_RefActConfigId_DupString, GlobalSchema_KeyRange_FactoryFn_Variant,
                         GlobalSchema_KeyRange_EulaType_Name, GlobalSchema_KeyRange_EulaType_DupString, GlobalSchema_KeyRange_IsValid_Name,
                         GlobalSchema_KeyRange_IsValid_DupString, GlobalSchema_KeyRange_Start_Name, GlobalSchema_KeyRange_Start_DupString,
                         GlobalSchema_KeyRange_End_Name, GlobalSchema_KeyRange_End_DupString, GlobalSchema_KeyRange_IsValid_FactoryFn,
                         GlobalSchema_Crypto_PublicKeyChildNode, GlobalSchema_Crypto_DupPublicKeyChildNode, dword_14045F4E8,
                         dword_14045F4EC, GlobalSchema_KeyRange_PublicKeysContainerName, dword_14045F508, GlobalSchema_KeyRange_FactoryFn,
                         qword_14045F518, qword_14045F520, GlobalSchema_KeyRange_RefActConfigId_Metadata, dword_14045F540, qword_14045F550,
                         qword_14045F558, GlobalSchema_KeyRange_PartNumber_Metadata, GlobalSchema_KeyRange_PartNumber_Name, dword_14045F578,
                         qword_14045F588, qword_14045F590, GlobalSchema_KeyRange_PartNumber_DupString, GlobalSchema_KeyRange_EulaType_Metadata,
                         dword_14045F5B0, GlobalSchema_KeyRange_EulaType_ParserFn, qword_14045F5C0, dword_14045F5C8, dword_14045F5CC,
                         GlobalSchema_KeyRange_IsValid_Metadata, dword_14045F5E8, qword_14045F5F8, qword_14045F600,
                         GlobalSchema_KeyRange_Start_Metadata, dword_14045F620, GlobalSchema_KeyRange_Start_ParserFn,
                         qword_14045F630, qword_14045F638};
use crate::x4::heap::{CreateRefCountedObject40byte, CreateRefCountedObject48byte, CreateRefCountedObject48byte_Variant2};
use crate::x4::types::{ProductKeyConfigSchemaElement, HRESULT, ProductKeyConfigSchemaElement_Vtbl, ProductKeyConfigGuidElement};


pub unsafe fn CreateSchemaElementObject48byte(
    pp_out_element: *mut *mut ProductKeyConfigSchemaElement,
) -> HRESULT {
    let mut v2: HRESULT = 0;

    if pp_out_element.is_null() {
        v2 = -2147024809; // E_INVALIDARG
        HandleSubsystemError(v2);
        LogTraceEvent(v2);
        return v2;
    }

    let process_heap = x4_getprocessheap();
    let allocated = x4_heapalloc(process_heap, 0, 0x30);
    let v4 = allocated as *mut ProductKeyConfigSchemaElement;

    if !v4.is_null() {
        // Zero-initialize the 48-byte block (equivalent to setting three OWORD blocks to 0)
        core::ptr::write_bytes(v4 as *mut u8, 0, 0x30);

        (*v4).m_cRef = 1;
        (*v4).lpVtbl = GlobalProductKeyConfigSchemaElement_Vtbl;
        (*v4).m_pPropertyContainer = core::ptr::null_mut();
        (*v4).m_pParentSchema = core::ptr::null_mut();

        *pp_out_element = v4;
    } else {
        v2 = -2147024882; // E_OUTOFMEMORY
        HandleSubsystemError(v2);
    }

    LogTraceEvent(v2) as HRESULT
}

pub unsafe fn CreateSchemaElementObject48byte_Variant2(
    pp_out_guid_element: *mut *mut ProductKeyConfigGuidElement,
) -> HRESULT {
    let mut v2: HRESULT = 0;

    if pp_out_guid_element.is_null() {
        v2 = -2147024809; // E_INVALIDARG
        HandleSubsystemError(v2);
        LogTraceEvent(v2);
        return v2;
    }

    let process_heap = x4_getprocessheap();
    let allocated = x4_heapalloc(process_heap, 0, 0x30);
    let v4 = allocated as *mut ProductKeyConfigGuidElement;

    if !v4.is_null() {
        // Zero-initialize the 48-byte block (equivalent to clearing the OWORDS via pZeroCursor)
        core::ptr::write_bytes(v4 as *mut u8, 0, 0x30);

        (*v4).m_cRef = 1;
        (*v4).lpVtbl = GlobalProductKeyConfigGuidElement_Vtbl;

        *pp_out_guid_element = v4;
    } else {
        v2 = -2147024882; // E_OUTOFMEMORY
        HandleSubsystemError(v2);
    }

    LogTraceEvent(v2) as HRESULT
}

pub unsafe fn InitializeProductKeyConfigSchema(
    p_out_schema_table: *mut *const c_void,
    p_out_element_count: *mut u32,
) -> *const c_void {
    let teb = x4_ntcurrentteb();
    let tls_pointer = *(teb as *const *const u8);
    let tls_slot_val = *(tls_pointer.offset(4) as *const i32);

    if GlobalProductKeyConfigSchemaInitGuard > tls_slot_val {
        InitThreadNotifyFooter(&mut GlobalProductKeyConfigSchemaInitGuard);
        if GlobalProductKeyConfigSchemaInitGuard == -1 {
            GlobalSchema_ActConfigId_Flags = 2;
            dword_14045F310 = 0;
            GlobalSchema_RootMetadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            GlobalSchema_RootFactoryFn = CreateSchemaElementObject48byte as *mut c_void;
            GlobalSchema_RootReservedField = 0;
            GlobalSchema_RootElementName = wide_string("ProductKeyConfiguration\0").as_ptr();
            dword_14045F328 = 1;
            GlobalSchema_RootContainerName = wide_string("Configurations\0").as_ptr();
            GlobalSchema_ConfigurationsNodeName = wide_string("Configurations\0").as_ptr();
            dword_14045F32C = 1;
            GlobalSchema_ConfigurationChildName = wide_string("Configuration\0").as_ptr();
            GlobalSchema_ActConfigId_ParentNodeName = wide_string("Configuration\0").as_ptr();
            GlobalSchema_KeyRangesListNodeName = wide_string("KeyRanges\0").as_ptr();
            GlobalSchema_ActConfigId_AttributeName = wide_string("ActConfigId\0").as_ptr();
            GlobalSchema_ActConfigId_DupString = wide_string("ActConfigId\0").as_ptr();
            GlobalSchema_RefGroupId_AttributeName = wide_string("RefGroupId\0").as_ptr();
            GlobalSchema_RefGroupId_DupString = wide_string("RefGroupId\0").as_ptr();
            GlobalSchema_EditionId_AttributeName = wide_string("EditionId\0").as_ptr();
            GlobalSchema_EditionId_DupString = wide_string("EditionId\0").as_ptr();
            GlobalSchema_ProductDesc_AttributeName = wide_string("ProductDescription\0").as_ptr();
            GlobalSchema_ProductDesc_DupString = wide_string("ProductDescription\0").as_ptr();
            GlobalSchema_KeyType_AttributeName = wide_string("ProductKeyType\0").as_ptr();
            GlobalSchema_KeyType_DupString = wide_string("ProductKeyType\0").as_ptr();
            GlobalSchema_IsRandomized_AttributeName = wide_string("IsRandomized\0").as_ptr();
            GlobalSchema_IsRandomized_DupString = wide_string("IsRandomized\0").as_ptr();
            dword_14045F348 = 0;
            qword_14045F350 = CreateSchemaElementObject48byte as *mut c_void;
            qword_14045F358 = 0;
            GlobalSchema_ActConfigId_Metadata = &raw mut GlobalNullSchemaMetadata;
            dword_14045F380 = 0;
            GlobalSchema_ActConfigId_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            GlobalSchema_ActConfigId_Reserved1 = 0;
            GlobalSchema_ActConfigId_Reserved2 = 0;
            GlobalSchema_RefGroupId_Metadata = &raw mut GlobalNullSchemaMetadata;
            dword_14045F3B8 = 0;
            GlobalSchema_RefGroupId_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            GlobalSchema_RefGroupId_Reserved1 = 0;
            GlobalSchema_RefGroupId_Reserved2 = 0;
            GlobalSchema_EditionId_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F3F0 = 0;
            GlobalSchema_EditionId_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            GlobalSchema_EditionId_Reserved1 = 0;
            GlobalSchema_EditionId_Reserved2 = 0;
            GlobalSchema_ProductDesc_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F428 = 0;
            GlobalSchema_ProductDesc_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            GlobalSchema_ProductDesc_Reserved1 = 0;
            GlobalSchema_ProductDesc_Reserved2 = 0;
            GlobalSchema_KeyType_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F460 = 0;
            GlobalSchema_KeyType_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            GlobalSchema_KeyType_Reserved1 = 0;
            GlobalSchema_KeyType_Reserved2 = 0;
            GlobalSchema_IsRandomized_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F498 = 0;
            GlobalSchema_IsRandomized_FactoryFn = CreateRefCountedObject40byte as *mut c_void;
            GlobalSchema_IsRandomized_Reserved1 = 0;
            GlobalSchema_IsRandomized_Reserved2 = 0;
            GlobalSchema_KeyRanges_Metadata1 = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            GlobalSchema_KeyRanges_Metadata2 = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F4D0 = 0;
            GlobalSchema_KeyRanges_FactoryFn = CreateSchemaElementObject48byte as *mut c_void;
            GlobalSchema_KeyRanges_Reserved1 = 0;
            GlobalSchema_KeyRanges_NodeName = wide_string("KeyRanges\0").as_ptr();
            GlobalSchema_KeyRanges_ChildName = wide_string("KeyRange\0").as_ptr();
            GlobalSchema_KeyRange_DupChildName = wide_string("KeyRange\0").as_ptr();
            GlobalSchema_KeyRange_RefActConfigId_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            GlobalSchema_KeyRange_RefActConfigId_Name = wide_string("RefActConfigId\0").as_ptr();
            GlobalSchema_KeyRange_RefActConfigId_DupString = wide_string("RefActConfigId\0").as_ptr();
            GlobalSchema_KeyRange_FactoryFn_Variant = CreateRefCountedObject48byte_Variant2 as *mut c_void;
            GlobalSchema_KeyRange_EulaType_Name = wide_string("EulaType\0").as_ptr();
            GlobalSchema_KeyRange_EulaType_DupString = wide_string("EulaType\0").as_ptr();
            GlobalSchema_KeyRange_IsValid_Name = wide_string("IsValid\0").as_ptr();
            GlobalSchema_KeyRange_IsValid_DupString = wide_string("IsValid\0").as_ptr();
            GlobalSchema_KeyRange_Start_Name = wide_string("Start\0").as_ptr();
            GlobalSchema_KeyRange_Start_DupString = wide_string("Start\0").as_ptr();
            GlobalSchema_KeyRange_End_Name = wide_string("End\0").as_ptr();
            GlobalSchema_KeyRange_End_DupString = wide_string("End\0").as_ptr();
            GlobalSchema_KeyRange_IsValid_FactoryFn = CreateRefCountedObject40byte as *mut c_void;
            GlobalSchema_Crypto_PublicKeyChildNode = wide_string("PublicKey\0").as_ptr();
            GlobalSchema_Crypto_DupPublicKeyChildNode = wide_string("PublicKey\0").as_ptr();
            dword_14045F4E8 = 1;
            dword_14045F4EC = 1;
            GlobalSchema_KeyRange_PublicKeysContainerName = wide_string("PublicKeys\0").as_ptr();
            dword_14045F508 = 0;
            GlobalSchema_KeyRange_FactoryFn = CreateSchemaElementObject48byte as *mut c_void;
            qword_14045F518 = 0;
            qword_14045F520 = 2;
            GlobalSchema_KeyRange_RefActConfigId_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F540 = 0;
            qword_14045F550 = 0;
            qword_14045F558 = 0;
            GlobalSchema_KeyRange_PartNumber_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            GlobalSchema_KeyRange_PartNumber_Name = wide_string("PartNumber\0").as_ptr();
            dword_14045F578 = 0;
            qword_14045F588 = 0;
            qword_14045F590 = 0;
            GlobalSchema_KeyRange_PartNumber_DupString = wide_string("PartNumber\0").as_ptr();
            GlobalSchema_KeyRange_EulaType_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F5B0 = 0;
            GlobalSchema_KeyRange_EulaType_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            qword_14045F5C0 = 0;
            dword_14045F5C8 = 0;
            dword_14045F5CC = 1;
            GlobalSchema_KeyRange_IsValid_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F5E8 = 1;
            qword_14045F5F8 = 0;
            qword_14045F600 = 0;
            GlobalSchema_KeyRange_Start_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F620 = 0;
            GlobalSchema_KeyRange_Start_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            qword_14045F630 = 0;
            qword_14045F638 = 0;
            GlobalSchema_KeyRange_End_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F658 = 0;
            GlobalSchema_KeyRange_End_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            qword_14045F668 = 0;
            qword_14045F670 = 0;
            GlobalSchema_Crypto_Metadata1 = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            GlobalSchema_Crypto_Metadata2 = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F690 = 0;
            GlobalSchema_Crypto_FactoryFn = CreateSchemaElementObject48byte as *mut c_void;
            qword_14045F6A0 = 0;
            dword_14045F6A8 = 1;
            dword_14045F6AC = 1;
            GlobalSchema_Crypto_PublicKeysNode = wide_string("PublicKeys\0").as_ptr();
            GlobalSchema_Crypto_OtherInfoNodeName = wide_string("OtherInfo\0").as_ptr();
            dword_14045F6C8 = 0;
            GlobalSchema_Crypto_OtherInfoFactoryFn = CreateSchemaElementObject48byte as *mut c_void;
            qword_14045F6D8 = 0;
            qword_14045F6E0 = 2;
            GlobalSchema_Crypto_GroupId_Name = wide_string("GroupId\0").as_ptr();
            GlobalSchema_Crypto_GroupId_DupString = wide_string("GroupId\0").as_ptr();
            GlobalSchema_Crypto_AlgorithmId_Name = wide_string("AlgorithmId\0").as_ptr();
            GlobalSchema_Crypto_AlgorithmId_DupString = wide_string("AlgorithmId\0").as_ptr();
            GlobalSchema_Crypto_PublicKeyValue_FactoryFn = CreateRefCountedObject48byte as *mut c_void;
            GlobalSchema_Ext_ItemChildNode = wide_string("Item\0").as_ptr();
            GlobalSchema_Ext_DupItemChildNode = wide_string("Item\0").as_ptr();
            GlobalSchema_Ext_NameField = wide_string("Name\0").as_ptr();
            GlobalSchema_Ext_DupNameField = wide_string("Name\0").as_ptr();
            GlobalSchema_Crypto_PublicKeyValue_Name = wide_string("PublicKeyValue\0").as_ptr();
            GlobalSchema_Crypto_PublicKeyValue_DupString = wide_string("PublicKeyValue\0").as_ptr();
            GlobalSchema_Ext_ValueField = wide_string("Value\0").as_ptr();
            GlobalSchema_Ext_DupValueField = wide_string("Value\0").as_ptr();
            GlobalSchema_Crypto_GroupId_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F700 = 0;
            GlobalSchema_Crypto_GroupId_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            qword_14045F710 = 0;
            qword_14045F718 = 0;
            qword_14045F728 = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F738 = 0;
            GlobalSchema_Crypto_AlgorithmId_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            qword_14045F748 = 0;
            qword_14045F750 = 0;
            GlobalSchema_Crypto_PublicKeyValue_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F770 = 0;
            qword_14045F780 = 0;
            qword_14045F788 = 0;
            GlobalSchema_Ext_Metadata1 = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            GlobalSchema_Ext_Metadata2 = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F7A8 = 0;
            GlobalSchema_Ext_FactoryFn = CreateSchemaElementObject48byte as *mut c_void;
            qword_14045F7B8 = 0;
            dword_14045F7C0 = 1;
            dword_14045F7C4 = 1;
            GlobalSchema_Ext_OtherInfoNode = wide_string("OtherInfo\0").as_ptr();
            GlobalSchema_Ext_Item_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F7E0 = 1;
            GlobalSchema_Ext_ItemFactoryFn = CreateSchemaElementObject48byte as *mut c_void;
            qword_14045F7F0 = 0;
            qword_14045F7F8 = 2;
            GlobalSchema_Ext_Name_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F818 = 0;
            GlobalSchema_Ext_Name_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            qword_14045F828 = 0;
            dword_14045F830 = 0;
            dword_14045F834 = 1;
            GlobalSchema_Ext_Value_Metadata = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F850 = 0;
            GlobalSchema_Ext_Value_ParserFn = CreateSchemaElementObject48byte_Variant2 as *mut c_void;
            qword_14045F860 = 0;
            dword_14045F868 = 0;
            dword_14045F86C = 1;
            GlobalSchema_Ext_EndBoundary_Metadata1 = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            GlobalSchema_Ext_EndBoundary_Metadata2 = &raw mut GlobalNullSchemaMetadata as *mut c_void;
            dword_14045F888 = 0;

            // InitThreadNotify(&mut GlobalProductKeyConfigSchemaInitGuard);
        }
    }

    if !p_out_schema_table.is_null() {
        *p_out_schema_table = &raw const off_14045F2E0;
    }
    if !p_out_element_count.is_null() {
        *p_out_element_count = 26;
    }

    &raw const off_14045F2E0
}

#[inline(always)]
fn wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

pub unsafe fn ProductKeyConfig__AddRef(a1: *mut c_void) -> i64 {
    // The reference count is typically stored at offset +8 in these COM-like objects
    let ref_count_ptr = (a1 as *mut u8).add(8) as *mut AtomicI32;

    // Atomically increment the reference count
    (*ref_count_ptr).fetch_add(1, Ordering::Relaxed);

    // Returns 1 (or the new ref count depending on convention; decompilation shows explicit return 1)
    1
}

pub unsafe fn ProductKeyConfig__NoOpParser() -> i64 {
    LogTraceEvent(0);
    0
}

pub unsafe fn ProductKeyConfig__QueryInterface(
    a1: *mut c_void,
    a2: *const u64,
    a3: *mut *mut c_void,
) -> HRESULT {
    let mut v3: HRESULT = 0;

    if !a3.is_null() {
        *a3 = core::ptr::null_mut();
    }

    if a2.is_null() || a3.is_null() {
        v3 = -2147467262; // E_NOINTERFACE
        HandleSubsystemError(v3);
        LogTraceEvent(v3);
        return v3;
    }

    let val0 = *a2;
    let val1 = *a2.offset(1);

    // Check for IID_IUnknown ({00000000-0000-0000-C000-000000000046})
    // or the specific custom interface GUID (0x40A5EF24439B1462, 0xC0C03F74FE0129BB)
    let is_valid_iid = (val0 == 0 && val1 == 0x46000000000000C0)
        || (val0 == 0x40A5EF24439B1462 && val1 == 0xC0C03F74FE0129BB);

    if is_valid_iid && !a1.is_null() {
        // Retrieve the vtable pointer, then fetch function pointer at offset 8 (AddRef)
        let vtable = *(a1 as *const *const u8);
        let v6_ptr = vtable.offset(8) as *const unsafe fn(*mut c_void);

        _guard_check_icall_fptr();

        let v6 = *v6_ptr;
        v6(a1);

        *a3 = a1;
    } else {
        v3 = -2147467262; // E_NOINTERFACE
        HandleSubsystemError(v3);
    }

    LogTraceEvent(v3);
    v3
}

pub unsafe fn ProductKeyConfig__Release(p_element: *mut c_void) -> i64 {
    if p_element.is_null() {
        return 1;
    }

    // m_cRef is located at offset +8 (right after the 8-byte vtbl pointer)
    let ref_count_ptr = (p_element as *mut u8).add(8) as *mut AtomicU32;

    // _InterlockedExchangeAdd(&pElement->m_cRef, 0xFFFFFFFF) subtracts 1 and returns the previous value
    let old_ref = (*ref_count_ptr).fetch_sub(1, Ordering::Release);

    if old_ref == 1 {
        core::sync::atomic::fence(Ordering::Acquire);
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, p_element);
    }

    1
}