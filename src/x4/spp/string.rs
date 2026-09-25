use core::ffi::{c_void, VaList};
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{memcmp, GetProcessHeap, _vsnwprintf, HeapAlloc, LocalAlloc, memcpy, HeapFree, LocalFree, wcschr, towlower};
use crate::x4::globals::{COR_E_OVERFLOW, E_INVALIDARG, E_NOT_VALID_STATE, E_OUTOFMEMORY, FALSE, GUID_FORMAT_STR, INTSAFE_E_ARITHMETIC_OVERFLOW, LMEM_ZEROINIT, MAX_BUILDER_CAPACITY, MAX_STRING_CCH, STRSAFE_E_INSUFFICIENT_BUFFER, STRSAFE_MAX_CCH, TRUE};
use crate::x4::misc::SwapVectorBuffersAndFreeOrphans;
use crate::x4::spp::format::SppFormatString;
use crate::x4::spp::pointer::{PointerVectorPushBack, PointerVectorResize};
use crate::x4::spp::SppDecodeHexByte;
use crate::x4::types::{PointerVector, BOOL, GUID, HRESULT, SppStringBuilder, PCWSTR, SIZE_T, PWSTR, HLOCAL};

pub unsafe fn SppDuplicateString(
    pszSource: PCWSTR,
    cchLength: u32,
    ppszDestination: *mut *mut u16,
) -> HRESULT {
    let mut alloc_size: u32 = 0;
    let mut status: HRESULT = 0;

    if pszSource.is_null() {
        if !ppszDestination.is_null() {
            *ppszDestination = core::ptr::null_mut();
        }
    } else {
        // Step 1: Calculate character count with null terminator (cchLength + 1)
        if let Some(cch_with_null) = cchLength.checked_add(1) {
            alloc_size = cch_with_null;
        } else {
            status = COR_E_OVERFLOW;
            HandleSubsystemError(COR_E_OVERFLOW);
        }

        LogTraceEvent(status);

        if status >= 0 {
            status = 0;
            // Step 2: Convert character count to byte size (alloc_size * 2)
            if alloc_size != 0 {
                if let Some(byte_size) = alloc_size.checked_mul(2) {
                    alloc_size = byte_size;
                } else {
                    status = COR_E_OVERFLOW;
                    HandleSubsystemError(COR_E_OVERFLOW);
                }
            } else {
                alloc_size = 0;
            }

            LogTraceEvent(status);

            if status >= 0 {
                // Step 3: Add 4-byte header prefix for length storage (alloc_size + 4)
                if let Some(total_bytes) = alloc_size.checked_add(4) {
                    alloc_size = total_bytes;
                    status = 0;
                } else {
                    status = COR_E_OVERFLOW;
                    HandleSubsystemError(COR_E_OVERFLOW);
                }

                LogTraceEvent(status);

                if status >= 0 {
                    let heap = GetProcessHeap();
                    let mem_ptr = HeapAlloc(heap, 0, alloc_size as usize as SIZE_T) as *mut u32;

                    if !mem_ptr.is_null() {
                        // Header stores original character length (4 bytes before string data)
                        *mem_ptr = cchLength;

                        // String data begins directly after 4-byte header
                        let dest_str_ptr = mem_ptr.add(1) as *mut u16;

                        // Copy UTF-16 string content and append null terminator
                        core::ptr::copy_nonoverlapping(
                            pszSource,
                            dest_str_ptr,
                            cchLength as usize,
                        );
                        *dest_str_ptr.add(cchLength as usize) = 0;

                        status = 0;
                        if !ppszDestination.is_null() {
                            *ppszDestination = dest_str_ptr;
                        }
                    } else {
                        status = E_OUTOFMEMORY;
                    }
                }
            }
        }

        if status < 0 {
            HandleSubsystemError(status);
        }
    }

    LogTraceEvent(status);

    status
}

pub unsafe fn SppDuplicateStringBounded(
    pwszSource: PCWSTR,
    cchLength: i64,
    ppwszDestination: *mut *mut u16,
) -> HRESULT {
    let mut bounded_cch: u32 = 0;
    let mut status: HRESULT;

    // Check if 64-bit cchLength fits within a 32-bit unsigned integer range without truncation
    if cchLength == (cchLength as u32 as i64) {
        LogTraceEvent(0);
        status = 0;
        bounded_cch = cchLength as u32;
    } else {
        status = COR_E_OVERFLOW;
        HandleSubsystemError(COR_E_OVERFLOW);
    }

    LogTraceEvent(status);

    if status >= 0 {
        status = SppDuplicateString(pwszSource, bounded_cch, ppwszDestination);
        if status < 0 {
            HandleSubsystemError(status);
        }
    } else {
        HandleSubsystemError(status);
    }

    LogTraceEvent(status);

    status
}

pub unsafe fn SppDuplicateStringLocal(
    pwszSource: PCWSTR,
    ppwszDestination: *mut *mut u16,
) -> HRESULT {
    let mut byte_len: u32 = 0;
    let mut cleanup_ptr: *mut u16 = core::ptr::null_mut();

    let mut status = SppGetStringByteLengthSafe(pwszSource, &mut byte_len, 0);

    if status < 0 {
        HandleSubsystemError(status);
    } else {
        let total_bytes = byte_len as usize;
        let alloc_ptr = LocalAlloc(LMEM_ZEROINIT, total_bytes as SIZE_T) as *mut u16;

        if alloc_ptr.is_null() {
            status = E_OUTOFMEMORY;
            HandleSubsystemError(status);
        } else {
            let mut remaining_cch = total_bytes >> 1; // Convert byte length to wchar_t count
            cleanup_ptr = alloc_ptr;

            if remaining_cch == 0 {
                status = E_INVALIDARG;
                HandleSubsystemError(status);
            } else {
                let max_limit = STRSAFE_MAX_CCH - remaining_cch;
                let mut dest_ptr = alloc_ptr;
                let mut src_ptr = pwszSource;

                // String copy loop bounded by buffer length and StringCchCopyW semantics
                loop {
                    if (max_limit + remaining_cch) == 0 {
                        break;
                    }

                    let ch = *src_ptr;
                    if ch == 0 {
                        break;
                    }

                    *dest_ptr = ch;
                    dest_ptr = dest_ptr.add(1);
                    src_ptr = src_ptr.add(1);

                    remaining_cch -= 1;
                    if remaining_cch == 0 {
                        break;
                    }
                }

                let term_ptr = if remaining_cch == 0 {
                    dest_ptr.offset(-1)
                } else {
                    dest_ptr
                };

                status = if remaining_cch == 0 {
                    STRSAFE_E_INSUFFICIENT_BUFFER
                } else {
                    0
                };

                *term_ptr = 0;

                if status < 0 {
                    HandleSubsystemError(status);
                } else {
                    cleanup_ptr = core::ptr::null_mut();
                    if !ppwszDestination.is_null() {
                        *ppwszDestination = alloc_ptr;
                    }
                }
            }
        }
    }

    LogTraceEvent(status);

    if !cleanup_ptr.is_null() {
        LocalFree(cleanup_ptr as HLOCAL);
    }

    status
}

pub unsafe fn SppGetAndDuplicateString(
    pszSourceString: PCWSTR,
    ppszDestinationString: *mut *mut u16,
) -> HRESULT {
    let mut cch_length: u32 = 0;
    let mut status: HRESULT = 0;

    if !pszSourceString.is_null() {
        let mut length_out: usize = 0;
        status = SppStringCchLengthW(
            pszSourceString,
            &mut cch_length as *mut u32 as usize,
            &mut length_out as *mut usize,
        );

        if status < 0 {
            HandleSubsystemError(status);
        } else {
            cch_length = length_out as u32;
        }
    }

    if status >= 0 {
        status = SppDuplicateString(pszSourceString, cch_length, ppszDestinationString);
        if status < 0 {
            HandleSubsystemError(status);
        }
    }

    LogTraceEvent(status);

    status
}

pub unsafe fn SppGetStringByteLength(
    pwszSource: PCWSTR,
    pcbOutBytes: *mut u32,
) -> HRESULT {
    let mut cb_total_bytes: u32 = 0;
    let mut status: HRESULT;

    if pwszSource.is_null() {
        status = E_INVALIDARG;
        HandleSubsystemError(E_INVALIDARG);
    } else {
        let mut curr_ptr = pwszSource;
        let mut cch_remaining_budget = MAX_STRING_CCH;

        // Traverse string up to 0x3FFFFFFF wide characters looking for null terminator
        while cch_remaining_budget != 0 {
            if *curr_ptr == 0 {
                break;
            }
            curr_ptr = curr_ptr.add(1);
            cch_remaining_budget -= 1;
        }

        if cch_remaining_budget == 0 {
            status = E_INVALIDARG;
            HandleSubsystemError(E_INVALIDARG);
        } else {
            let cch_length = MAX_STRING_CCH - cch_remaining_budget;
            let cb_calculated_bytes = 2 * cch_length;
            let is_non_zero = cch_remaining_budget != 0;

            let cb_validated_bytes = if is_non_zero { cb_calculated_bytes } else { 0 };

            // Check if character count to byte count conversion fits within unsigned 32-bit integer
            if cb_validated_bytes == (cb_validated_bytes as u32 as i64) as usize {
                LogTraceEvent(0);
                status = 0;
                cb_total_bytes = cb_validated_bytes as u32;
            } else {
                status = COR_E_OVERFLOW;
                HandleSubsystemError(COR_E_OVERFLOW);
            }

            LogTraceEvent(status);

            if status >= 0 {
                if !pcbOutBytes.is_null() {
                    *pcbOutBytes = cb_total_bytes;
                }
            } else {
                HandleSubsystemError(status);
            }
        }
    }

    LogTraceEvent(status);

    status
}

pub unsafe fn SppGetStringByteLengthSafe(
    pwszSource: PCWSTR,
    pcbOutBytes: *mut u32,
    cbMaxLimit: u32,
) -> HRESULT {
    let mut out_bytes: u32 = 0;
    let mut status: HRESULT;

    if pwszSource.is_null() {
        status = E_INVALIDARG;
        HandleSubsystemError(E_INVALIDARG);
    } else {
        let max_cch = (cbMaxLimit >> 1) as usize;
        let mut remaining_cch = max_cch;
        let mut curr_ptr = pwszSource;

        // Traverse source string up to max_cch characters looking for null terminator
        while remaining_cch != 0 {
            if *curr_ptr == 0 {
                break;
            }
            curr_ptr = curr_ptr.add(1);
            remaining_cch -= 1;
        }

        if remaining_cch == 0 {
            status = E_INVALIDARG;
            HandleSubsystemError(E_INVALIDARG);
        } else {
            let cch_length = max_cch - remaining_cch;
            let string_bytes = cch_length * 2;

            // Include 2 extra bytes for the null terminator
            let mut total_bytes: usize = 0;
            let overflow = match string_bytes.checked_add(2) {
                Some(val) => {
                    total_bytes = val;
                    false
                }
                None => true,
            };

            if overflow {
                status = COR_E_OVERFLOW;
                HandleSubsystemError(COR_E_OVERFLOW);
            } else {
                status = 0;
            }

            LogTraceEvent(status);

            if status >= 0 {
                // Check if byte length fits within a 32-bit unsigned integer
                if total_bytes == (total_bytes as u32 as usize) {
                    LogTraceEvent(0);
                    status = 0;
                    out_bytes = total_bytes as u32;
                } else {
                    status = COR_E_OVERFLOW;
                    HandleSubsystemError(COR_E_OVERFLOW);
                }

                LogTraceEvent(status);

                if status >= 0 {
                    if !pcbOutBytes.is_null() {
                        *pcbOutBytes = out_bytes;
                    }
                } else {
                    HandleSubsystemError(status);
                }
            }
        }
    }

    LogTraceEvent(status);

    status
}

pub unsafe fn SppGetStringCharacterCount(
    pwszSource: PCWSTR,
    pcchOutCharacters: *mut u32,
) -> HRESULT {
    let mut out_cch: u32 = 0;
    let mut status: HRESULT;

    if pwszSource.is_null() {
        status = E_INVALIDARG;
        HandleSubsystemError(E_INVALIDARG);
    } else {
        let mut curr_ptr = pwszSource;
        let mut remaining_budget = MAX_STRING_CCH;

        // Traverse source string up to 0x7FFFFFFF wide characters looking for null terminator
        while remaining_budget != 0 {
            if *curr_ptr == 0 {
                break;
            }
            curr_ptr = curr_ptr.add(1);
            remaining_budget -= 1;
        }

        if remaining_budget == 0 {
            status = E_INVALIDARG;
            HandleSubsystemError(E_INVALIDARG);
        } else {
            let calculated_cch = MAX_STRING_CCH - remaining_budget;

            // Check if character count fits within a 32-bit unsigned integer
            if calculated_cch == (calculated_cch as u32 as i64) as usize {
                LogTraceEvent(0);
                status = 0;
                out_cch = calculated_cch as u32;
            } else {
                status = COR_E_OVERFLOW;
                HandleSubsystemError(COR_E_OVERFLOW);
            }

            LogTraceEvent(status);

            if status >= 0 {
                if !pcchOutCharacters.is_null() {
                    *pcchOutCharacters = out_cch;
                }
            } else {
                HandleSubsystemError(status);
            }
        }
    }

    LogTraceEvent(status);

    status
}

fn hex_char_to_val(ch: u16) -> Option<u8> {
    match ch {
        48..=57 => Some((ch - 48) as u8),   // '0'-'9'
        65..=70 => Some((ch - 55) as u8),   // 'A'-'F'
        97..=102 => Some((ch - 87) as u8),  // 'a'-'f'
        _ => None,
    }
}

pub unsafe fn SppGuidFromString(
    pwszGuidString: PCWSTR,
    pOutGuid: *mut GUID,
) -> HRESULT {
    let mut status: HRESULT;
    let mut out_guid = GUID::default();

    if pwszGuidString.is_null() || pOutGuid.is_null() {
        HandleSubsystemError(E_INVALIDARG);
        LogTraceEvent(E_INVALIDARG);
        return E_INVALIDARG;
    }

    // Verify hyphen separators at expected positions in standard GUID format (8-4-4-4-12)
    if *pwszGuidString.add(8) != 45
        || *pwszGuidString.add(13) != 45
        || *pwszGuidString.add(18) != 45
        || *pwszGuidString.add(23) != 45
    {
        HandleSubsystemError(E_INVALIDARG);
        LogTraceEvent(E_INVALIDARG);
        return E_INVALIDARG;
    }

    // 1. Parse Data1 (8 hex digits -> 32-bit uint)
    let mut data1_val: u32 = 0;
    let mut curr_ptr = pwszGuidString;
    status = 0;

    for _ in 0..8 {
        if let Some(val) = hex_char_to_val(*curr_ptr) {
            data1_val = (data1_val * 16) + val as u32;
            curr_ptr = curr_ptr.add(1);
        } else {
            status = E_INVALIDARG;
            HandleSubsystemError(E_INVALIDARG);
            break;
        }
    }

    LogTraceEvent(status);

    if status < 0 {
        HandleSubsystemError(status);
        LogTraceEvent(status);
        return status;
    }

    out_guid.Data1 = data1_val;

    // 2. Parse Data2 (4 hex digits -> 16-bit uint)
    let mut data2_val: u16 = 0;
    curr_ptr = pwszGuidString.add(9);
    status = 0;

    for _ in 0..4 {
        if let Some(val) = hex_char_to_val(*curr_ptr) {
            data2_val = (data2_val * 16) + val as u16;
            curr_ptr = curr_ptr.add(1);
        } else {
            status = E_INVALIDARG;
            HandleSubsystemError(E_INVALIDARG);
            break;
        }
    }

    LogTraceEvent(status);

    if status < 0 {
        HandleSubsystemError(status);
        LogTraceEvent(status);
        return status;
    }

    out_guid.Data2 = data2_val;

    // 3. Parse Data3 (4 hex digits -> 16-bit uint)
    let mut data3_val: u16 = 0;
    curr_ptr = pwszGuidString.add(14);
    status = 0;

    for _ in 0..4 {
        if let Some(val) = hex_char_to_val(*curr_ptr) {
            data3_val = (data3_val * 16) + val as u16;
            curr_ptr = curr_ptr.add(1);
        } else {
            status = E_INVALIDARG;
            HandleSubsystemError(E_INVALIDARG);
            break;
        }
    }

    LogTraceEvent(status);

    if status < 0 {
        HandleSubsystemError(status);
        LogTraceEvent(status);
        return status;
    }

    out_guid.Data3 = data3_val;

    // 4. Parse Data4 (8 hex byte pairs)
    let offsets: [usize; 8] = [19, 21, 24, 26, 28, 30, 32, 34];

    for i in 0..8 {
        status = SppDecodeHexByte(pwszGuidString.add(offsets[i]), &mut out_guid.Data4[i]);
        if status < 0 {
            HandleSubsystemError(status);
            LogTraceEvent(status);
            return status;
        }
    }

    *pOutGuid = out_guid;
    LogTraceEvent(status);

    status
}

pub unsafe fn SppMarshalRegistryStringToContext(
    pwszRawRegistryString: PCWSTR,
    cbDataLenBytes: u32,
    ppwszOutContextString: *mut *mut u16,
    pbTakeBufferOwnership: *mut BOOL,
) -> HRESULT {
    let mut cleanup_ptr: *mut u16 = core::ptr::null_mut();
    let mut destination_string: *mut u16 = core::ptr::null_mut();
    let mut status: HRESULT;

    // Validate buffer parameters: length must be non-zero, even byte count, and null-terminated
    let char_len = (cbDataLenBytes >> 1) as usize;
    let is_valid = cbDataLenBytes != 0
        && (cbDataLenBytes & 1) == 0
        && !pwszRawRegistryString.is_null()
        && *pwszRawRegistryString.add(char_len - 1) == 0;

    if !is_valid {
        status = E_NOT_VALID_STATE;
        HandleSubsystemError(E_NOT_VALID_STATE);
    } else {
        status = SppGetAndDuplicateString(pwszRawRegistryString, &mut destination_string);

        if status >= 0 {
            if !ppwszOutContextString.is_null() {
                *ppwszOutContextString = destination_string;
            }
            if !pbTakeBufferOwnership.is_null() {
                *pbTakeBufferOwnership = 0;
            }
        } else {
            HandleSubsystemError(status);
            cleanup_ptr = destination_string;
        }
    }

    LogTraceEvent(status);

    if !cleanup_ptr.is_null() {
        let process_heap = GetProcessHeap();
        // Free heap allocation offset by 2 wchar_t characters (header/prefix bytes)
        HeapFree(process_heap, 0, cleanup_ptr.offset(-2) as *mut c_void);
        LogTraceEvent(0);
    }

    status
}

pub unsafe fn SppMatchWildcardString(
    pwszInputString: PCWSTR,
    pwszPattern: PCWSTR,
) -> BOOL {
    if pwszPattern.is_null() {
        return 1;
    }
    if pwszInputString.is_null() {
        return 0;
    }

    // Find end of input string (i)
    let mut end_input = pwszInputString;
    while *end_input != 0 {
        end_input = end_input.add(1);
    }

    let mut pattern_ptr = pwszPattern;
    let mut input_ptr = pwszInputString;

    let mut backtrack_pattern: PCWSTR = core::ptr::null();
    let mut backtrack_input: PCWSTR = core::ptr::null();
    let mut saved_backtrack_input: PCWSTR = core::ptr::null();

    loop {
        let mut is_wildcard_active = 0;
        saved_backtrack_input = backtrack_input;

        loop {
            let mut ch = *pattern_ptr;

            // Handle end of pattern string segment
            while ch == 0 {
                if is_wildcard_active != 0 || end_input <= input_ptr {
                    return 1;
                }
                if backtrack_pattern.is_null() {
                    return 0;
                }
                if backtrack_input.is_null() {
                    return 0;
                }
                pattern_ptr = backtrack_pattern;
                input_ptr = backtrack_input;
                is_wildcard_active = 1;
                ch = *pattern_ptr;
            }

            if ch != 42 { // '*'
                break;
            }

            let next_pat = pattern_ptr.add(1);
            if is_wildcard_active != 0 {
                is_wildcard_active = 0;
                pattern_ptr = next_pat;
            } else {
                is_wildcard_active = 1;
                pattern_ptr = next_pat;
            }
        }

        let mut segment_end = pattern_ptr;
        let pattern_checkpoint = if is_wildcard_active == 0 {
            backtrack_pattern
        } else {
            pattern_ptr
        };

        let mut current_ch = *pattern_ptr;
        while current_ch != 0 && current_ch != 42 {
            segment_end = segment_end.add(1);
            current_ch = *segment_end;
        }

        let input_remaining = (end_input as usize - input_ptr as usize) / core::mem::size_of::<u16>();
        let segment_len = (segment_end as usize - pattern_ptr as usize) / core::mem::size_of::<u16>();

        if segment_len > input_remaining {
            return 0;
        }

        let mut remaining_len = input_remaining;

        loop {
            let mut matched = true;
            let mut curr_str = input_ptr;
            let mut curr_pat_offset = (pattern_ptr as isize - end_input as isize) / core::mem::size_of::<u16>() as isize;

            while *curr_str != 0 {
                let pat_char_ptr = curr_str.offset(curr_pat_offset + remaining_len as isize);
                if pat_char_ptr >= segment_end {
                    break;
                }

                let c1 = towlower(*curr_str as u32);
                let c2 = towlower(*pat_char_ptr as u32);

                if c1 != c2 {
                    matched = false;
                    break;
                }
                curr_str = curr_str.add(1);
            }

            if matched {
                backtrack_pattern = pattern_checkpoint;
                backtrack_input = curr_str;
                input_ptr = curr_str;
                if is_wildcard_active == 0 {
                    backtrack_input = saved_backtrack_input;
                }
                pattern_ptr = segment_end;
                break; // Continue to outer loop (LABEL_30)
            }

            if is_wildcard_active == 0 {
                backtrack_pattern = pattern_checkpoint;
                if pattern_checkpoint.is_null() {
                    return 0;
                }
                backtrack_input = saved_backtrack_input;

                if backtrack_input.is_null() {
                    return 0;
                }
                pattern_ptr = backtrack_pattern;
                input_ptr = backtrack_input;
                is_wildcard_active = 1;
                break;
            }

            if remaining_len < 1 {
                return 0;
            }

            remaining_len -= 1;
            input_ptr = input_ptr.add(1);

            if remaining_len < segment_len {
                return 0;
            }
        }
    }
}

pub unsafe fn SppSelectAndDuplicateString(
    pwszDefaultValue: PCWSTR,
    pwszOverrideValue: PCWSTR,
    pwszMatchCriterion: PCWSTR,
    ppwszOutString: *mut *mut u16,
) -> HRESULT {
    let mut selected_source = pwszDefaultValue;
    let mut allocated_mem: *mut u16 = core::ptr::null_mut();
    let mut cleanup_mem: HLOCAL = core::ptr::null_mut();

    if !pwszMatchCriterion.is_null() {
        // Compare pwszMatchCriterion against static string L"EditionId"
        let mut match_found = true;
        let mut criterion_ptr = pwszMatchCriterion;
        let mut target_ptr = EDITION_ID_STR.as_ptr();

        loop {
            let target_ch = *target_ptr;
            let criterion_ch = *criterion_ptr;

            let diff = (criterion_ch as i32) - (target_ch as i32);
            if diff != 0 {
                match_found = false;
                break;
            }

            if target_ch == 0 {
                break;
            }

            criterion_ptr = criterion_ptr.add(1);
            target_ptr = target_ptr.add(1);
        }

        if match_found {
            selected_source = pwszOverrideValue;
        }
    }

    let status = SppDuplicateStringLocal(selected_source, &mut allocated_mem);

    if status >= 0 {
        if !ppwszOutString.is_null() {
            *ppwszOutString = allocated_mem;
        }
    } else {
        HandleSubsystemError(status);
        cleanup_mem = allocated_mem as HLOCAL;
    }

    LogTraceEvent(status);

    if !cleanup_mem.is_null() {
        LocalFree(cleanup_mem);
    }

    status
}

pub unsafe fn SppSplitString(
    pwszSourceString: PCWSTR,
    wchDelimiter: u16,
    pOutVector: *mut PointerVector,
) -> HRESULT {
    let mut current_token = pwszSourceString;
    let mut duped_token: *mut u16 = core::ptr::null_mut();
    let mut status: HRESULT = 0;
    let mut dest_string: *mut u16 = core::ptr::null_mut();

    let mut temp_vec = PointerVector {
        capacity: 0,
        size: 0,
        elements: core::ptr::null_mut(),
    };

    if pwszSourceString.is_null() || *pwszSourceString == 0 || pOutVector.is_null() {
        status = E_INVALIDARG;
        HandleSubsystemError(E_INVALIDARG);
    } else {
        let mut curr_ptr = pwszSourceString;

        loop {
            if *curr_ptr == 0 {
                if !current_token.is_null() && *current_token != 0 {
                    status = SppGetAndDuplicateString(current_token, &mut dest_string);
                    if status < 0 {
                        HandleSubsystemError(status);
                        duped_token = dest_string;
                        break;
                    }

                    let push_status = PointerVectorPushBack(
                        &mut temp_vec,
                        &mut dest_string as *mut *mut u16 as *mut *mut c_void,
                    );
                    status = push_status;
                    if push_status < 0 {
                        HandleSubsystemError(push_status);
                        duped_token = dest_string;
                        break;
                    }

                    duped_token = dest_string;
                }
                break;
            }

            if !duped_token.is_null() {
                let heap = GetProcessHeap();
                HeapFree(heap, 0, duped_token.offset(-2) as *mut c_void);
                LogTraceEvent(0);
                duped_token = core::ptr::null_mut();
                dest_string = core::ptr::null_mut();
            }

            let delim_ptr = wcschr(current_token, wchDelimiter);
            if delim_ptr.is_null() {
                if !current_token.is_null() && *current_token != 0 {
                    status = SppGetAndDuplicateString(current_token, &mut dest_string);
                    if status < 0 {
                        HandleSubsystemError(status);
                        duped_token = dest_string;
                        break;
                    }

                    let push_status = PointerVectorPushBack(
                        &mut temp_vec,
                        &mut dest_string as *mut *mut u16 as *mut *mut c_void,
                    );
                    status = push_status;
                    if push_status < 0 {
                        HandleSubsystemError(push_status);
                        duped_token = dest_string;
                        break;
                    }

                    duped_token = dest_string;
                }
                break;
            }

            curr_ptr = delim_ptr.add(1);

            // Skip consecutive delimiters
            while *curr_ptr != 0 && *curr_ptr == wchDelimiter {
                curr_ptr = curr_ptr.add(1);
            }

            if current_token != delim_ptr {
                let token_len = (delim_ptr as usize - current_token as usize) / core::mem::size_of::<u16>();
                status = SppDuplicateStringBounded(
                    current_token,
                    token_len as i64,
                    &mut dest_string,
                );

                if status < 0 {
                    HandleSubsystemError(status);
                    break;
                }

                let push_status = PointerVectorPushBack(
                    &mut temp_vec,
                    &mut dest_string as *mut *mut u16 as *mut *mut c_void,
                );
                status = push_status;
                if push_status < 0 {
                    HandleSubsystemError(push_status);
                    break;
                }

                duped_token = dest_string;
            }

            current_token = curr_ptr;
            if curr_ptr.is_null() {
                break;
            }
        }

        if status >= 0 {
            let mut result_vec = PointerVector {
                capacity: 0,
                size: 0,
                elements: core::ptr::null_mut(),
            };

            SwapVectorBuffersAndFreeOrphans(
                &mut result_vec.capacity,
                &mut temp_vec.capacity,
            );
            SwapVectorBuffersAndFreeOrphans(
                &mut result_vec.capacity,
                &mut (*pOutVector).capacity,
            );

            PointerVectorResize(&mut result_vec, 0);

            if !result_vec.elements.is_null() {
                let heap = GetProcessHeap();
                HeapFree(heap, 0, result_vec.elements as *mut c_void);
            }
        }
    }

    LogTraceEvent(status);

    if !duped_token.is_null() {
        let heap = GetProcessHeap();
        HeapFree(heap, 0, duped_token.offset(-2) as *mut c_void);
        LogTraceEvent(0);
    }

    PointerVectorResize(&mut temp_vec, 0);

    if !temp_vec.elements.is_null() {
        let heap = GetProcessHeap();
        HeapFree(heap, 0, temp_vec.elements as *mut c_void);
    }

    status
}

pub unsafe fn SppStringBuilderAppend(
    builder: *mut SppStringBuilder,
    pwszSource: PCWSTR,
) -> HRESULT {
    let mut status: HRESULT = 0;
    let mut cch_length: usize = 0;

    if !pwszSource.is_null() {
        let hr = SppStringCchLengthW(pwszSource, MAX_STRING_CCH, &mut cch_length);
        status = hr;

        if hr >= 0 {
            let append_hr = SppStringBuilderAppendBounded(
                builder,
                pwszSource as *const c_void,
                cch_length as u32,
            );
            status = append_hr;
            if append_hr < 0 {
                HandleSubsystemError(append_hr);
            }
        } else {
            HandleSubsystemError(hr);
        }
    }

    LogTraceEvent(status);

    status
}

pub unsafe fn SppStringBuilderAppendBounded(
    builder: *mut SppStringBuilder,
    pRawBuffer: *const c_void,
    cchLength: u32,
) -> HRESULT {
    let cch_copy_count = cchLength as usize;

    let status = SppStringBuilderResize(builder, cchLength, 0);
    let hr = status;

    if status >= 0 && !builder.is_null() {
        let builder_ref = &mut *builder;

        // Copy source UTF-16 characters to the end of the current buffer
        memcpy(
            builder_ref.buffer.add(builder_ref.currentLength as usize) as *mut c_void,
            pRawBuffer,
            2 * cch_copy_count,
        );

        let new_length = builder_ref.currentLength + cch_copy_count as i32;
        builder_ref.currentLength = new_length;

        // Ensure null-termination
        *builder_ref.buffer.add(new_length as usize) = 0;
    } else {
        HandleSubsystemError(status);
    }

    LogTraceEvent(hr);

    hr
}

pub unsafe fn SppStringBuilderAppendFormat(
    builder: *mut SppStringBuilder,
    pwszFormat: PCWSTR,
    mut _args:VaList
) -> HRESULT {
    let mut resize_step: u32 = 1024;
    let mut status: HRESULT;

    if builder.is_null() {
        return -2147024809; // E_INVALIDARG
    }

    let builder_ref = &mut *builder;

    if builder_ref.buffer.is_null() {
        builder_ref.totalCapacity = 0;
        builder_ref.currentLength = 0;

        status = SppStringBuilderResize(builder, 1024, 1);
        if status < 0 {
            HandleSubsystemError(status);
        }
        LogTraceEvent(status);
    } else {
        status = 0;
    }

    if status >= 0 {
        loop {
            let dest_ptr = builder_ref.buffer.offset(builder_ref.currentLength as isize);
            let dest_cch = (builder_ref.totalCapacity - builder_ref.currentLength + 1) as usize;

            status = SppStringCchVPrintfW(
                dest_ptr,
                dest_cch,
                pwszFormat,
                _args,
            );

            if status >= 0 {
                break;
            }

            if status != STRSAFE_E_INSUFFICIENT_BUFFER {
                HandleSubsystemError(status);
                LogTraceEvent(status);
                if status < 0 {
                    HandleSubsystemError(status);
                }
                LogTraceEvent(status);
                return status;
            }

            // Reset boundary wchar on buffer overflow before expanding
            *builder_ref.buffer.offset(builder_ref.currentLength as isize) = 0;

            if resize_step >= 0x8000 {
                status = SppStringBuilderResize(builder, 1024, 1);
                if status < 0 {
                    HandleSubsystemError(status);
                    LogTraceEvent(status);
                    if status < 0 {
                        HandleSubsystemError(status);
                    }
                    LogTraceEvent(status);
                    return status;
                }
            } else {
                status = SppStringBuilderResize(builder, resize_step, 1);
                if status < 0 {
                    HandleSubsystemError(status);
                    LogTraceEvent(status);
                    if status < 0 {
                        HandleSubsystemError(status);
                    }
                    LogTraceEvent(status);
                    return status;
                }
                resize_step *= 2;
            }
        }

        let mut calculated_len: usize = 0;
        status = SppStringCchLengthW(builder_ref.buffer, MAX_STRING_CCH, &mut calculated_len);
        if status >= 0 {
            builder_ref.currentLength = calculated_len as i32;
        } else {
            HandleSubsystemError(status);
        }
    }

    LogTraceEvent(status);
    if status < 0 {
        HandleSubsystemError(status);
    }
    LogTraceEvent(status);

    status
}

pub unsafe fn SppStringBuilderResize(
    builder: *mut SppStringBuilder,
    additionalCapacity: u32,
    growthMode: i32,
) -> HRESULT {
    let mut target_capacity: u32 = 0;
    let mut status: HRESULT;

    if builder.is_null() {
        return -2147024809; // E_INVALIDARG
    }

    let builder_ref = &mut *builder;

    if growthMode == 0 {
        // Linear / Additive growth check
        if let Some(req_cap) = builder_ref.currentLength.checked_add(additionalCapacity as i32) {
            target_capacity = req_cap as u32;
            status = 0;
        } else {
            status = INTSAFE_E_ARITHMETIC_OVERFLOW;
            HandleSubsystemError(INTSAFE_E_ARITHMETIC_OVERFLOW);
        }
        LogTraceEvent(status);

        if status < 0 {
            HandleSubsystemError(status);
            LogTraceEvent(status);
            return status;
        }

        // Fast path: existing capacity is sufficient
        if builder_ref.totalCapacity >= target_capacity as i32 && !builder_ref.buffer.is_null() {
            status = 0;
            LogTraceEvent(status);
            return status;
        }

        // Calculate expanded size with a 1024-character padding buffer
        if let Some(req_cap) = builder_ref.currentLength.checked_add(additionalCapacity as i32) {
            target_capacity = req_cap as u32;
            status = 0;
        } else {
            status = INTSAFE_E_ARITHMETIC_OVERFLOW;
            HandleSubsystemError(INTSAFE_E_ARITHMETIC_OVERFLOW);
        }
        LogTraceEvent(status);

        if status < 0 {
            HandleSubsystemError(status);
            LogTraceEvent(status);
            return status;
        }

        if let Some(padded_cap) = target_capacity.checked_add(1024) {
            target_capacity = padded_cap;
            status = 0;
        } else {
            status = INTSAFE_E_ARITHMETIC_OVERFLOW;
            HandleSubsystemError(INTSAFE_E_ARITHMETIC_OVERFLOW);
        }
    } else {
        // Growth mode 1: Accumulate total capacity directly
        if let Some(new_cap) = builder_ref.totalCapacity.checked_add(additionalCapacity as i32) {
            target_capacity = new_cap as u32;
            status = 0;
        } else {
            status = INTSAFE_E_ARITHMETIC_OVERFLOW;
            HandleSubsystemError(INTSAFE_E_ARITHMETIC_OVERFLOW);
        }
    }

    LogTraceEvent(status);
    if status < 0 {
        HandleSubsystemError(status);
        LogTraceEvent(status);
        return status;
    }

    // Capacity upper limit validation
    if target_capacity >= MAX_BUILDER_CAPACITY {
        status = STRSAFE_E_INSUFFICIENT_BUFFER;
        HandleSubsystemError(status);
        LogTraceEvent(status);
        return status;
    }

    // Include +1 for the null terminator
    let element_count: u64 = (target_capacity as u64) + 1;
    if element_count < (target_capacity as u64) {
        status = INTSAFE_E_ARITHMETIC_OVERFLOW;
        HandleSubsystemError(INTSAFE_E_ARITHMETIC_OVERFLOW);
    } else {
        status = 0;
    }
    LogTraceEvent(status);

    if status < 0 {
        HandleSubsystemError(status);
        LogTraceEvent(status);
        return status;
    }

    LogTraceEvent(0);

    let bytes_to_alloc: usize = if element_count == 0 {
        0
    } else {
        2 * (element_count as usize)
    };

    let heap = GetProcessHeap();
    let new_buffer = HeapAlloc(heap, 0, bytes_to_alloc as SIZE_T) as *mut u16;

    if new_buffer.is_null() {
        status = E_OUTOFMEMORY;
        HandleSubsystemError(status);
        LogTraceEvent(status);
        return status;
    }

    // Copy existing string data to the newly allocated memory block
    if !builder_ref.buffer.is_null() && builder_ref.currentLength > 0 {
        memcpy(
            new_buffer as *mut c_void,
            builder_ref.buffer as *const c_void,
            2 * (builder_ref.currentLength as usize),
        );
    }

    // Ensure null termination at current length boundary
    *new_buffer.offset(builder_ref.currentLength as isize) = 0;
    builder_ref.totalCapacity = target_capacity as i32;

    // Release old buffer if allocated
    if !builder_ref.buffer.is_null() {
        let heap = GetProcessHeap();
        HeapFree(heap, 0, builder_ref.buffer as *mut c_void);
    }

    builder_ref.buffer = new_buffer;

    LogTraceEvent(status);
    status
}

pub unsafe fn SppStringCchLengthW(
    pszString: PCWSTR,
    cchMax: usize,
    pcchLength: *mut usize,
) -> HRESULT {
    let mut status: HRESULT;
    let original_cch_max = cchMax;

    if pszString.is_null() || cchMax > STRSAFE_MAX_CCH {
        status = E_INVALIDARG;
        HandleSubsystemError(E_INVALIDARG);
        LogTraceEvent(status);
        return status;
    }

    let mut remaining_cch = cchMax;
    let mut curr_ptr = pszString;

    if remaining_cch > 0 {
        while remaining_cch > 0 {
            if *curr_ptr == 0 {
                break;
            }
            curr_ptr = curr_ptr.add(1);
            remaining_cch -= 1;
        }
    }

    if remaining_cch == 0 {
        status = E_INVALIDARG;
        HandleSubsystemError(E_INVALIDARG);
        LogTraceEvent(status);
        return status;
    }

    let calculated_len = original_cch_max - remaining_cch;

    if calculated_len <= (u32::MAX as usize) {
        LogTraceEvent(0);
        status = 0;

        if !pcchLength.is_null() {
            *pcchLength = calculated_len;
        }
    } else {
        status = INTSAFE_E_ARITHMETIC_OVERFLOW;
        HandleSubsystemError(INTSAFE_E_ARITHMETIC_OVERFLOW);
    }

    LogTraceEvent(status);
    if status < 0 {
        HandleSubsystemError(status);
    }

    LogTraceEvent(status);

    status
}

pub unsafe fn SppStringCchVPrintfW(
    pszDest: PWSTR,
    cchDest: usize,
    pszFormat: PCWSTR,
    argList: VaList,
) -> HRESULT {
    let mut status: HRESULT;

    // Check bounds: cchDest - 1 must be <= 0x7FFFFFFE (i.e., 1 <= cchDest <= 0x7FFFFFFF)
    if cchDest > 0 && cchDest <= STRSAFE_MAX_CCH {
        let max_chars = cchDest - 1;
        status = 0;

        let written = _vsnwprintf(pszDest, max_chars, pszFormat, argList);

        if written < 0 || (written as usize) > max_chars {
            status = STRSAFE_E_INSUFFICIENT_BUFFER;
        } else if (written as usize) != max_chars {
            return status;
        }

        // Null-terminate at the last available wchar position
        *pszDest.add(max_chars) = 0;
        return status;
    }

    status = E_INVALIDARG;

    if cchDest > 0 && !pszDest.is_null() {
        *pszDest = 0;
    }

    status
}

pub unsafe fn SppStringFromGuid(
    pGuid: *const GUID,
    ppwszOutString: *mut PWSTR,
) -> HRESULT {
    let mut ppsz_destination: PWSTR = core::ptr::null_mut();

    if pGuid.is_null() || ppwszOutString.is_null() {
        let status = -2147024809; // E_INVALIDARG
        HandleSubsystemError(status);
        LogTraceEvent(status);
        return status;
    }

    let guid = &*pGuid;

    let hr = SppFormatString(
        &mut ppsz_destination,
        GUID_FORMAT_STR.as_ptr(),
        guid.Data1,
        guid.Data2 as u32,
        guid.Data3 as u32,
        guid.Data4[0] as u32,
        guid.Data4[1] as u32,
        guid.Data4[2] as u32,
        guid.Data4[3] as u32,
        guid.Data4[4] as u32,
        guid.Data4[5] as u32,
        guid.Data4[6] as u32,
        guid.Data4[7] as u32,
    );

    let mut string_to_free: PWSTR = core::ptr::null_mut();

    if hr >= 0 {
        *ppwszOutString = ppsz_destination;
    } else {
        HandleSubsystemError(hr);
        string_to_free = ppsz_destination;
    }

    LogTraceEvent(hr);

    if !string_to_free.is_null() {
        let heap = GetProcessHeap();
        // The allocated memory buffer header offset (-2 wchars)
        HeapFree(heap, 0, string_to_free.offset(-2) as *mut c_void);
        LogTraceEvent(0);
    }

    hr
}

pub unsafe fn SppStringStartsWith(
    pszString: PCWSTR,
    pszPrefix: PCWSTR,
    pIsMatch: *mut BOOL,
) -> HRESULT {
    let mut is_match: BOOL = FALSE;

    // Retrieve length from length header preceding the buffer pointer (-1 u32 offset)
    let len_string: u32 = if !pszString.is_null() {
        *(pszString as *const u32).offset(-1)
    } else {
        0
    };
    LogTraceEvent(0);

    let len_prefix: u32 = if !pszPrefix.is_null() {
        *(pszPrefix as *const u32).offset(-1)
    } else {
        0
    };
    LogTraceEvent(0);

    if len_string >= len_prefix {
        let byte_count = (len_prefix as usize) * 2;
        let cmp_result = memcmp(
            pszString as *const c_void,
            pszPrefix as *const c_void,
            byte_count,
        );
        if cmp_result == 0 {
            is_match = TRUE;
        }
    }

    if !pIsMatch.is_null() {
        *pIsMatch = is_match;
    }

    LogTraceEvent(0);

    0
}