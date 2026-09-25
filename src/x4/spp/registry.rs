use core::ffi::c_void;
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{GetProcessHeap, HeapFree, LocalFree};
use crate::x4::globals::E_INVALIDARG;
use crate::x4::misc::SwapVectorBuffersAndFreeOrphans;
use crate::x4::spp::string::SppDuplicateStringLocal;
use crate::x4::spp::tokenizer::{SppTokenizerGetTokenString, SppTokenizerInitialize};
use crate::x4::spp::vector::{SppVectorPushBack, SppVectorReallocateHeap};
use crate::x4::types::{VectorLayoutBuffered, PWSTR, HLOCAL, PCWSTR, VectorLayout, HRESULT, SppStringTokenizer};

pub unsafe fn SppParseRegistryContainersList(
    hRegistryQueryOutput: i64,
    _unusedParam: i64,
    pOutVectorLayout: *mut VectorLayoutBuffered,
) -> HRESULT {
    let mut v15: SppStringTokenizer = core::mem::zeroed();
    let mut layout = VectorLayoutBuffered {
        capacity: 0,
        count: 0,
        ppBuffer: core::ptr::null_mut(),
    };
    let mut pwsz_source: PCWSTR = core::ptr::null();
    let mut h_mem: HLOCAL = core::ptr::null_mut();

    let mut v5: HRESULT;

    if hRegistryQueryOutput != 0 && !pOutVectorLayout.is_null() {
        v5 = SppTokenizerInitialize(&mut v15, hRegistryQueryOutput as PCWSTR, 0x3B);

        loop {
            if v5 < 0 {
                HandleSubsystemError(v5);
                break;
            }

            v5 = SppTokenizerGetTokenString(&mut v15, &mut pwsz_source);
            if v5 < 0 {
                HandleSubsystemError(v5);
                break;
            }

            let v7 = pwsz_source;
            if v7.is_null() {
                let mut v13 = VectorLayoutBuffered {
                    capacity: 0,
                    count: 0,
                    ppBuffer: core::ptr::null_mut(),
                };

                SwapVectorBuffersAndFreeOrphans(&mut v13.capacity, &mut layout.capacity);
                SwapVectorBuffersAndFreeOrphans(
                    &mut v13.capacity,
                    &mut (*pOutVectorLayout).capacity,
                );

                SppVectorReallocateHeap(&mut v13, 0);

                let pp_buffer = v13.ppBuffer;
                if !pp_buffer.is_null() {
                    let process_heap = GetProcessHeap();
                    HeapFree(process_heap, 0, pp_buffer as *mut c_void);
                }

                break;
            }

            if !h_mem.is_null() {
                LocalFree(h_mem);
                h_mem = core::ptr::null_mut();
            }

            v5 = SppDuplicateStringLocal(v7, &mut h_mem as *mut _ as *mut PWSTR);
            if v5 < 0 {
                HandleSubsystemError(v5);
                break;
            }

            v5 = SppVectorPushBack(&mut layout, &h_mem as *mut *mut c_void);
        }
    } else {
        v5 = E_INVALIDARG;
        HandleSubsystemError(E_INVALIDARG);
    }

    LogTraceEvent(v5);

    if !h_mem.is_null() {
        LocalFree(h_mem);
    }

    SppVectorReallocateHeap(&mut layout, 0);

    let v10 = layout.ppBuffer;
    if !layout.ppBuffer.is_null() {
        let v11 = GetProcessHeap();
        HeapFree(v11, 0, v10 as *mut c_void);
    }

    if !v15.pSourceStringBase.is_null() {
        LocalFree(v15.pSourceStringBase as HLOCAL);
    }

    v5
}