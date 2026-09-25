use core::ops::Shr;
use crate::x4::error::HandleFatalRelocError;
use crate::x4::helpers::binary_search_lookup;
use crate::x4::types::{uint32_t};

pub static mut off_1403C4590: *mut unsafe extern "fastcall" fn() -> i64 = core::ptr::null_mut();
pub static mut off_1403C4580: *mut unsafe extern "fastcall" fn() -> i64 = core::ptr::null_mut();
pub static mut qword_14045FD10: u64 = 0;

#[repr(C)]
pub struct RelocDescriptorEntry {

    padding: u64, // C bitfield: 3 bits
    block_rva: u64, // C bitfield: 28 bits
    block_length: u64, // C bitfield: 28 bits
    custom_flags: u64, // C bitfield: 5 bits
}

#[repr(C)]
pub struct RelocTableHeader {

    entry_count_marker: uint32_t,
    alignment_padding: uint32_t,
    entries: [RelocDescriptorEntry; 1],
}

impl Shr<i32> for RelocDescriptorEntry {
    type Output = ();

    fn shr(self, rhs: i32) -> Self::Output {
        todo!()
    }
}

pub unsafe fn ProcessReloc_Rva0_Len36(
    pRelocBlockRva: *mut RelocTableHeader,
    pTargetState: *mut u32,
) -> i64 {
    let v2 = *pTargetState as u64;
    let entry_count_marker = (*pRelocBlockRva).entry_count_marker as usize;
    let mut v4: i64 = 0;

    // v33 = off_1403C4580;
    let mut v33 = off_1403C4580 as *mut usize;
    let mut v31: u32 = 0;

    // v6 = entry_count_marker + 335544320; (Pointer arithmetic on struct)
    let v6 = (entry_count_marker + 335544320) as *mut RelocTableHeader;
    let mut v5 = (0x140000000u64 + v2) as *mut i64;

    let mut v7: i64 = 0;
    let v42 = v6;
    let mut v32: i64 = 0;
    let mut v8: u32 = 0;
    let mut v9 = *v5;
    let mut v10 = (*v6).entry_count_marker;
    let mut v11 = (*v5 as u64) >> 63;
    let v34 = v11;

    if (v10 & 0x7FFFFFFE) != 0 {
        loop {
            let mut v12: u32 = 0;
            let mut v36: i64 = 0;
            let mut v13: u8 = 0;
            let mut v35: u32 = 0;
            let mut v37: i32 = 0;

            // v14 = *(_DWORD *)&v6->entries[(unsigned int)v8] & 0xFFFFFFF;
            let entry_ptr = core::ptr::addr_of!((*v6).entries[v8 as usize]) as *const u32;
            let v14 = *entry_ptr & 0xFFFFFFF;
            let mut v15 = 0x140000000u64 + v14 as u64;

            // v16 = *(_QWORD *)&v6->entries[(unsigned int)v8] >> 36;
            let v16 = (*core::ptr::addr_of!((*v6).entries[v8 as usize])) >> 36;

            let v40 = v14;
            let v41 = v15;

            // Dynamic storage variables matching stack footprint
            let mut v38: u32 = 0;
            let mut v39: u32 = 0;

            if v11 != 0 {
                // Populate internal variables by reference via custom binary lookup
                // In C++, the layout of stack variable spans fields v35 through v41 continuously
                let mut stack_context_ptr = core::ptr::addr_of_mut!(v35) as usize;
                binary_search_lookup(stack_context_ptr as *mut usize, v14, v16 as i32);

                v38 = *( (stack_context_ptr + 24) as *const u32 );
                v39 = *( (stack_context_ptr + 28) as *const u32 );
                v36 = *( (stack_context_ptr + 4) as *const i64 );

                if v39 < v35 && (*( (v36 + 4 * v39 as i64) as *const u32 ) & 0xFFFFFFF) < v38 {
                    v12 = *( (v36 + 4 * v39 as i64) as *const u32 );
                    v39 += 1;
                    // Write back updated index counter to local context
                    *( (stack_context_ptr + 28) as *mut u32 ) = v39;
                    v13 = 1;
                }
                v11 = v34;
                v15 = v41;
                v8 = v31;
            }

            let mut v17: u32 = 0;
            if v16 != 0 {
                let v18 = v40;
                loop {
                    if v11 != 0 && v13 != 0 && (v17 + v18) == (v12 & 0xFFFFFFF) {
                        let v19 = qword_14045FD10.wrapping_sub(0x140000000);
                        let v20 = v12 >> 28;
                        let mut v21: u32 = 0;
                        let mut v22: u64;
                        let mut v45 = [0u64; 2];

                        if v20 != 0 {
                            if v20 == 3 {
                                v22 = 4;
                                let source_val = *( (v17 as u64 + v15) as *const u32 );
                                let casting_ptr = v45.as_mut_ptr() as *mut u32;
                                *casting_ptr = source_val.wrapping_add(v19 as u32);
                            } else if v20 == 10 {
                                v22 = 8;
                                let source_val = *( (v17 as u64 + v15) as *const u64 );
                                v45[0] = source_val.wrapping_add(v19);
                            } else {
                                v22 = u64::MAX; // -1
                            }

                            let mut v23: u64 = 0;
                            let v33_func_ptr = *(v33.add(1) as *const usize);
                            let stream_hasher_fn = core::mem::transmute::<
                                usize,
                                unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64, u64)
                            >(v33_func_ptr);

                            loop {
                                v8 = *((v45.as_ptr() as usize + v23 as usize) as *const u8) as u32;
                                stream_hasher_fn(
                                    core::ptr::addr_of_mut!(v33),
                                    core::ptr::addr_of_mut!(v32),
                                    v8 as u64,
                                    v15,
                                );
                                v21 += 1;
                                v23 = v21 as u64;
                                if (v21 as u64) >= v22 { break; }
                            }
                        }

                        v17 += v21;
                        if v39 >= v35 {
                            v13 = 0;
                        } else {
                            let current_check_val = *( (v36 + 4 * v39 as i64) as *const u32 );
                            if (current_check_val & 0xFFFFFFF) >= v38 {
                                v13 = 0;
                            } else {
                                v12 = *( (v36 + 4 * v39 as i64) as *const u32 );
                                v39 += 1;
                                v13 = 1;
                            }
                        }
                    } else {
                        let v24_val = *( (v17 as u64 + v15) as *const u8 ) as u64;
                        let v33_func_ptr = *(v33.add(1) as *const usize);
                        let stream_hasher_fn_short = core::mem::transmute::<
                            usize,
                            unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                        >(v33_func_ptr);

                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v33),
                            core::ptr::addr_of_mut!(v32),
                            v24_val,
                        );
                        v17 += 1;
                    }

                    v15 = v41;
                    v11 = v34;
                    if v17 >= (v16 as u32) { break; }
                }
                let v6_current = v42;
                v8 = v31;
            }

            v10 = (*v6).entry_count_marker;
            v11 = v34;
            v8 += 1;
            v31 = v8;

            if (v8 as u32) >= ((v10 >> 1) & 0x3FFFFFFF) { break; }
        }
        v7 = v32;
    }

    // 2. Validate structural checksum parity states
    let v25 = 0x8000000000000000u64;
    if (v7 as u64 | v25) != (v9 as u64 | v25) {
        HandleFatalRelocError();
    }

    // 3. Fallback tracking execution block under active relocation flags
    if v11 != 0 {
        v32 = 0;
        let mut v26: u32 = 0;
        if (v10 & 0x7FFFFFFE) != 0 {
            loop {
                let entry_ptr = core::ptr::addr_of!((*v6).entries[v26 as usize]) as *const u32;
                let mut v27 = (0x140000000u64 + (*entry_ptr & 0xFFFFFFF) as u64) as *mut u8;
                let v28 = (*core::ptr::addr_of!((*v6).entries[v26 as usize])) >> 36;

                if v28 != 0 {
                    let mut v29 = v28 as u32;
                    let v33_func_ptr = *(v33.add(1) as *const usize);
                    let stream_hasher_fn_short = core::mem::transmute::<
                        usize,
                        unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                    >(v33_func_ptr);

                    loop {
                        let v25_val = *v27 as u64;
                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v33),
                            core::ptr::addr_of_mut!(v32),
                            v25_val,
                        );
                        v27 = v27.add(1);
                        v29 -= 1;
                        if v29 == 0 { break; }
                    }
                }
                v26 += 1;
                if v26 >= ((v10 >> 1) & 0x3FFFFFFF) { break; }
            }
            v4 = v32;
        }

        // Commit state atomically using a 64-bit Compare-Exchange primitive loop
        let target_val = v4 & 0x7FFFFFFFFFFFFFFF;
        core::intrinsics::atomic_cxchg_seqcst_seqcst(v5, v9, target_val);
    }

    1
}

pub unsafe fn ProcessReloc_Rva28_Len0(
    pRelocBlockRva: *mut RelocTableHeader,
    a2: *mut u32,
) -> i64 {
    let v2 = *a2 as u64;
    let entry_count_marker = (*pRelocBlockRva).entry_count_marker as usize;
    let mut v4: i64 = 0;

    // v32 = off_1403C45D0;
    let mut v32 = off_1403C45D0 as *mut usize;
    let mut v30: u32 = 0;

    // v6 = entry_count_marker + 335544320;
    let mut v6 = (entry_count_marker + 335544320) as *mut RelocTableHeader;
    let v5 = (0x140000000u64 + v2) as *mut i64;

    let mut v7: i64 = 0;
    let v41 = v6;
    let mut v31: i64 = 0;
    let mut v8: u32 = 0;
    let v9 = *v5;
    let mut v10 = (*v6).entry_count_marker;
    let mut v11 = (*v5 as u64) >> 63;
    let v43 = *v5;
    let v40 = v11;

    if (v10 & 0x3FFFFFFF) != 0 {
        loop {
            let mut v12: u32 = 0;
            let mut v34: i64 = 0;
            let mut v13: u8 = 0;
            let mut v33: u32 = 0;
            let mut v35: i32 = 0;

            // v14 = *(_DWORD *)&v6->entries[(unsigned int)v8] & 0xFFFFFFF;
            let entries_base_ptr = core::ptr::addr_of!((*v6).entries[v8 as usize]);
            let mut v14 = *(entries_base_ptr as *const u32) & 0xFFFFFFF;

            // v15 = (*(_QWORD *)&v6->entries[(unsigned int)v8] >> 28) & 0xFFFFFFFLL;
            let v15 = ((*entries_base_ptr) >> 28) & 0xFFFFFFF;
            let mut v38 = v14;

            let mut v16 = 0x140000000u64 + v15;
            let v39 = v16;

            // Dynamic storage tracking variables mapping continuous local stack spans
            let mut v36: u32 = 0;
            let mut v37: u32 = 0;

            if v11 != 0 {
                let stack_context_ptr = core::ptr::addr_of_mut!(v33) as usize;
                binary_search_lookup(stack_context_ptr as *mut usize, v15 as u32, v14 as i32);

                v36 = *((stack_context_ptr + 24) as *const u32);
                v37 = *((stack_context_ptr + 28) as *const u32);
                v34 = *((stack_context_ptr + 4) as *const i64);

                if v37 < v33 && (*((v34 + 4 * v37 as i64) as *const u32) & 0xFFFFFFF) < v36 {
                    v12 = *((v34 + 4 * v37 as i64) as *const u32);
                    v37 += 1;
                    *((stack_context_ptr + 28) as *mut u32) = v37;
                    v13 = 1;
                }
                v16 = v39;
                v8 = v30;
            }

            let mut v17: u32 = 0;
            if v14 != 0 {
                let v18 = v40 as u32;
                loop {
                    if v18 != 0 && v13 != 0 && (v17 + v15 as u32) == (v12 & 0xFFFFFFF) {
                        let v19 = qword_14045FD10.wrapping_sub(0x140000000);
                        let v20 = v12 >> 28;
                        let mut v21: u32 = 0;
                        let mut v22: u64;
                        let mut v44 = [0u64; 2];

                        if v20 != 0 {
                            if v20 == 3 {
                                v22 = 4;
                                let source_val = *((v17 as u64 + v16) as *const u32);
                                let casting_ptr = v44.as_mut_ptr() as *mut u32;
                                *casting_ptr = source_val.wrapping_add(v19 as u32);
                            } else if v20 == 10 {
                                v22 = 8;
                                let source_val = *((v17 as u64 + v16) as *const u64);
                                v44[0] = source_val.wrapping_add(v19);
                            } else {
                                v22 = u64::MAX;
                            }

                            let mut v23: u64 = 0;
                            let v32_func_ptr = *(v32.add(1) as *const usize);
                            let stream_hasher_fn = core::mem::transmute::<
                                usize,
                                unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                            >(v32_func_ptr);

                            loop {
                                v8 = *((v44.as_ptr() as usize + v23 as usize) as *const u8) as u32;
                                stream_hasher_fn(
                                    core::ptr::addr_of_mut!(v32),
                                    core::ptr::addr_of_mut!(v31),
                                    v8 as u64,
                                );
                                v21 += 1;
                                v23 = v21 as u64;
                                if (v21 as u64) >= v22 { break; }
                            }
                            v14 = v38;
                        }

                        v17 += v21;
                        if v37 >= v33 {
                            v13 = 0;
                        } else {
                            let current_check_val = *((v34 + 4 * v37 as i64) as *const u32);
                            if (current_check_val & 0xFFFFFFF) >= v36 {
                                v13 = 0;
                            } else {
                                v12 = *((v34 + 4 * v37 as i64) as *const u32);
                                v37 += 1;
                                v13 = 1;
                            }
                        }
                    } else {
                        let v24_val = *((v17 as u64 + v16) as *const u8) as u64;
                        let v32_func_ptr = *(v32.add(1) as *const usize);
                        let stream_hasher_fn_short = core::mem::transmute::<
                            usize,
                            unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                        >(v32_func_ptr);

                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v32),
                            core::ptr::addr_of_mut!(v31),
                            v24_val,
                        );
                        v17 += 1;
                    }

                    v16 = v39;
                    if v17 >= v14 { break; }
                }
                v6 = v41;
                v8 = v30;
            }

            v10 = (*v6).entry_count_marker;
            v11 = v40;
            v8 += 1;
            v30 = v8;

            if v8 >= (v10 & 0x3FFFFFFF) { break; }
        }
        v7 = v31;
    }

    // 2. Parity validation check
    let v25 = 0x8000000000000000u64;
    if (v7 as u64 | v25) != (v9 as u64 | v25) {
        HandleFatalRelocError();
    }

    // 3. Fallback tracking execution block under active relocation flags
    if v11 != 0 {
        v31 = 0;
        let mut v26: u32 = 0;
        if (v10 & 0x3FFFFFFF) != 0 {
            loop {
                let entries_base_ptr = core::ptr::addr_of!((*v6).entries[v26 as usize]);
                let calculated_rva = ((*entries_base_ptr) >> 28) & 0xFFFFFFF;
                let mut v27 = (0x140000000u64 + calculated_rva) as *mut u8;

                let dword_entry_val = *(entries_base_ptr as *const u32);
                if (dword_entry_val & 0xFFFFFFF) != 0 {
                    let mut v28 = dword_entry_val & 0xFFFFFFF;
                    let v32_func_ptr = *(v32.add(1) as *const usize);
                    let stream_hasher_fn_short = core::mem::transmute::<
                        usize,
                        unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                    >(v32_func_ptr);

                    loop {
                        let v25_val = *v27 as u64;
                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v32),
                            core::ptr::addr_of_mut!(v31),
                            v25_val,
                        );
                        v27 = v27.add(1);
                        v28 -= 1;
                        if v28 == 0 { break; }
                    }
                }
                v26 += 1;
                if v26 >= (v10 & 0x3FFFFFFF) { break; }
            }
            v4 = v31;
        }

        let target_val = v4 & 0x7FFFFFFFFFFFFFFF;
        core::intrinsics::atomic_cxchg_seqcst_seqcst(v5, v9, target_val);
    }

    1
}

pub unsafe fn ProcessReloc_Rva2_Len30(
    pRelocBlockRva: *mut RelocTableHeader,
    a2: *mut u32,
) -> i64 {
    let v2 = *a2 as u64;
    let entry_count_marker = (*pRelocBlockRva).entry_count_marker as usize;
    let mut v4: i64 = 0;

    // v34 = off_1403C4570;
    let mut v34 = off_1403C4570 as *mut usize;
    let mut v32: u32 = 0;

    // v6 = entry_count_marker + 335544320;
    let mut v6 = (entry_count_marker + 335544320) as *mut RelocTableHeader;
    let v5 = (0x140000000u64 + v2) as *mut i64;

    let mut v7: i64 = 0;
    let v42 = v6;
    let mut v33: i64 = 0;
    let mut v8: u32 = 0;
    let v9 = *v5;
    let mut v10 = (*v6).entry_count_marker;
    let mut v11 = (*v5 as u64) >> 63;
    let v44 = *v5;
    let v35 = v11;

    if (v10 & 0x3FFFFFFF) != 0 {
        loop {
            let mut v12: u32 = 0;
            let mut v37: i64 = 0;
            let mut v13: u8 = 0;
            let mut v36: u32 = 0;
            let mut v38: i32 = 0;

            // v14 = (unsigned __int64)v6->entries[(unsigned int)v8];
            let v14 = (*v6).entries[v8 as usize];

            // v15 = (v14 >> 2) & 0xFFFFFFF;
            let v15 = (v14 >> 2) & 0xFFFFFFF;

            // v16 = (v14 >> 30) & 0xFFFFFFF;
            let v16 = (v14 >> 30) & 0xFFFFFFF;

            let mut v17 = 0x140000000u64 + v15;
            let v41 = v17;

            // Dynamic storage tracking variables mapping continuous local stack spans
            let mut v39: u32 = 0;
            let mut v40: u32 = 0;

            if v11 != 0 {
                let stack_context_ptr = core::ptr::addr_of_mut!(v36) as usize;
                binary_search_lookup(stack_context_ptr as i64, v15, v16 as i64);

                v39 = *((stack_context_ptr + 24) as *const u32);
                v40 = *((stack_context_ptr + 28) as *const u32);
                v37 = *((stack_context_ptr + 4) as *const i64);

                if v40 < v36 && (*((v37 + 4 * v40 as i64) as *const u32) & 0xFFFFFFF) < v39 {
                    v12 = *((v37 + 4 * v40 as i64) as *const u32);
                    v40 += 1;
                    *((stack_context_ptr + 28) as *mut u32) = v40;
                    v13 = 1;
                }
                v17 = v41;
                v11 = v35;
                v8 = v32;
            }

            let mut v18: u32 = 0;
            if v16 != 0 {
                loop {
                    if v11 != 0 && v13 != 0 && (v18 + v15 as u32) == (v12 & 0xFFFFFFF) {
                        let v19 = qword_14045FD10.wrapping_sub(0x140000000);
                        let v20 = v12 >> 28;
                        let mut v21: u32 = 0;
                        let mut v22: u64;
                        let mut v45 = [0u64; 2];

                        if v20 != 0 {
                            if v20 == 3 {
                                v22 = 4;
                                let source_val = *((v18 as u64 + v17) as *const u32);
                                let casting_ptr = v45.as_mut_ptr() as *mut u32;
                                *casting_ptr = source_val.wrapping_add(v19 as u32);
                            } else if v20 == 10 {
                                v22 = 8;
                                let source_val = *((v18 as u64 + v17) as *const u64);
                                v45 = source_val.wrapping_add(v19);
                            } else {
                                v22 = u64::MAX;
                            }

                            let mut v23: u64 = 0;
                            let v34_func_ptr = *(v34.add(1) as *const usize);
                            let stream_hasher_fn = core::mem::transmute::<
                                usize,
                                unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                            >(v34_func_ptr);

                            loop {
                                v8 = *((v45.as_ptr() as usize + v23 as usize) as *const u8) as u32;
                                stream_hasher_fn(
                                    core::ptr::addr_of_mut!(v34),
                                    core::ptr::addr_of_mut!(v33),
                                    v8 as u64,
                                );
                                v21 += 1;
                                v23 = v21 as u64;
                                if (v21 as u64) >= v22 { break; }
                            }
                        }

                        v18 += v21;
                        if v40 >= v36 {
                            v13 = 0;
                        } else {
                            let current_check_val = *((v37 + 4 * v40 as i64) as *const u32);
                            if (current_check_val & 0xFFFFFFF) >= v39 {
                                v13 = 0;
                            } else {
                                v12 = *((v37 + 4 * v40 as i64) as *const u32);
                                v40 += 1;
                                v13 = 1;
                            }
                        }
                    } else {
                        let v24_val = *((v18 as u64 + v17) as *const u8) as u64;
                        let v34_func_ptr = *(v34.add(1) as *const usize);
                        let stream_hasher_fn_short = core::mem::transmute::<
                            usize,
                            unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                        >(v34_func_ptr);

                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v34),
                            core::ptr::addr_of_mut!(v33),
                            v24_val,
                        );
                        v18 += 1;
                    }

                    v11 = v35;
                    v17 = v41;
                    if v18 >= (v16 as u32) { break; }
                }
                v8 = v32;
                v6 = v42;
            }

            v10 = (*v6).entry_count_marker;
            v11 = v35;
            v8 += 1;
            v32 = v8;

            if v8 >= (v10 & 0x3FFFFFFF) { break; }
        }
        v7 = v33;
    }

    // 2. Parity validation check
    let v25 = 0x8000000000000000u64;
    if (v7 as u64 | v25) != (v9 as u64 | v25) {
        HandleFatalRelocError();
    }

    // 3. Fallback tracking execution block under active relocation flags
    if v11 != 0 {
        v33 = 0;
        let mut v26: u32 = 0;
        if (v10 & 0x3FFFFFFF) != 0 {
            loop {
                let v27 = (*v6).entries[v26 as usize];
                let mut v28 = (0x140000000u64 + ((v27 >> 2) & 0xFFFFFFF)) as *mut u8;
                let v29 = ((v27 >> 30) & 0xFFFFFFF) as u32;

                if v29 != 0 {
                    let mut v30 = v29;
                    let v34_func_ptr = *(v34.add(1) as *const usize);
                    let stream_hasher_fn_short = core::mem::transmute::<
                        usize,
                        unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                    >(v34_func_ptr);

                    loop {
                        let v25_val = *v28 as u64;
                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v34),
                            core::ptr::addr_of_mut!(v33),
                            v25_val,
                        );
                        v28 = v28.add(1);
                        v30 -= 1;
                        if v30 == 0 { break; }
                    }
                }
                v26 += 1;
                if v26 >= (v10 & 0x3FFFFFFF) { break; }
            }
            v4 = v33;
        }

        let target_val = v4 & 0x7FFFFFFFFFFFFFFF;
        core::intrinsics::atomic_cxchg_seqcst_seqcst(v5, v9, target_val);
    }

    1
}

pub unsafe fn ProcessReloc_Rva30_Len0(
    pRelocBlockRva: *mut RelocTableHeader,
    a2: *mut u32,
) -> i64 {
    let v2 = *a2 as u64;
    let entry_count_marker = (*pRelocBlockRva).entry_count_marker as usize;
    let mut v4: i64 = 0;

    // v32 = off_1403C4550;
    let mut v32 = off_1403C4550 as *mut usize;
    let mut v30: u32 = 0;

    // v6 = entry_count_marker + 335544320;
    let mut v6 = (entry_count_marker + 335544320) as *mut RelocTableHeader;
    let v5 = (0x140000000u64 + v2) as *mut i64;

    let mut v7: i64 = 0;
    let v41 = v6;
    let mut v31: i64 = 0;
    let mut v8: u32 = 0;
    let v9 = *v5;
    let mut v10 = (*v6).entry_count_marker;
    let mut v11 = (*v5 as u64) >> 63;
    let v43 = *v5;
    let v40 = v11;

    if (v10 & 0x3FFFFFFF) != 0 {
        loop {
            let mut v12: u32 = 0;
            let mut v34: i64 = 0;
            let mut v13: u8 = 0;
            let mut v33: u32 = 0;
            let mut v35: i32 = 0;

            // v14 = *(_DWORD *)&v6->entries[(unsigned int)v8] & 0xFFFFFFF;
            let entries_base_ptr = core::ptr::addr_of!((*v6).entries[v8 as usize]);
            let mut v14 = *(entries_base_ptr as *const u32) & 0xFFFFFFF;

            // v15 = (*(_QWORD *)&v6->entries[(unsigned int)v8] >> 30) & 0xFFFFFFFLL;
            let v15 = ((*entries_base_ptr) >> 30) & 0xFFFFFFF;
            let mut v38 = v14;

            let mut v16 = 0x140000000u64 + v15;
            let v39 = v16;

            // Dynamic storage tracking variables mapping continuous local stack spans
            let mut v36: u32 = 0;
            let mut v37: u32 = 0;

            if v11 != 0 {
                let stack_context_ptr = core::ptr::addr_of_mut!(v33) as usize;
                binary_search_lookup(stack_context_ptr as i64, v15 as u32, v14);

                v36 = *((stack_context_ptr + 24) as *const u32);
                v37 = *((stack_context_ptr + 28) as *const u32);
                v34 = *((stack_context_ptr + 4) as *const i64);

                if v37 < v33 && (*((v34 + 4 * v37 as i64) as *const u32) & 0xFFFFFFF) < v36 {
                    v12 = *((v34 + 4 * v37 as i64) as *const u32);
                    v37 += 1;
                    *((stack_context_ptr + 28) as *mut u32) = v37;
                    v13 = 1;
                }
                v16 = v39;
                v8 = v30;
            }

            let mut v17: u32 = 0;
            if v14 != 0 {
                let v18 = v40 as u32;
                loop {
                    if v18 != 0 && v13 != 0 && (v17 + v15 as u32) == (v12 & 0xFFFFFFF) {
                        let v19 = qword_14045FD10.wrapping_sub(0x140000000);
                        let v20 = v12 >> 28;
                        let mut v21: u32 = 0;
                        let mut v22: u64;
                        let mut v44 = [0u64; 2];

                        if v20 != 0 {
                            if v20 == 3 {
                                v22 = 4;
                                let source_val = *((v17 as u64 + v16) as *const u32);
                                let casting_ptr = v44.as_mut_ptr() as *mut u32;
                                *casting_ptr = source_val.wrapping_add(v19 as u32);
                            } else if v20 == 10 {
                                v22 = 8;
                                let source_val = *((v17 as u64 + v16) as *const u64);
                                v44 = source_val.wrapping_add(v19);
                            } else {
                                v22 = u64::MAX;
                            }

                            let mut v23: u64 = 0;
                            let v32_func_ptr = *(v32.add(1) as *const usize);
                            let stream_hasher_fn = core::mem::transmute::<
                                usize,
                                unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                            >(v32_func_ptr);

                            loop {
                                v8 = *((v44.as_ptr() as usize + v23 as usize) as *const u8) as u32;
                                stream_hasher_fn(
                                    core::ptr::addr_of_mut!(v32),
                                    core::ptr::addr_of_mut!(v31),
                                    v8 as u64,
                                );
                                v21 += 1;
                                v23 = v21 as u64;
                                if (v21 as u64) >= v22 { break; }
                            }
                            v14 = v38;
                        }

                        v17 += v21;
                        if v37 >= v33 {
                            v13 = 0;
                        } else {
                            let current_check_val = *((v34 + 4 * v37 as i64) as *const u32);
                            if (current_check_val & 0xFFFFFFF) >= v36 {
                                v13 = 0;
                            } else {
                                v12 = *((v34 + 4 * v37 as i64) as *const u32);
                                v37 += 1;
                                v13 = 1;
                            }
                        }
                    } else {
                        let v24_val = *((v17 as u64 + v16) as *const u8) as u64;
                        let v32_func_ptr = *(v32.add(1) as *const usize);
                        let stream_hasher_fn_short = core::mem::transmute::<
                            usize,
                            unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                        >(v32_func_ptr);

                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v32),
                            core::ptr::addr_of_mut!(v31),
                            v24_val,
                        );
                        v17 += 1;
                    }

                    v16 = v39;
                    if v17 >= v14 { break; }
                }
                v6 = v41;
                v8 = v30;
            }

            v10 = (*v6).entry_count_marker;
            v11 = v40;
            v8 += 1;
            v30 = v8;

            if v8 >= (v10 & 0x3FFFFFFF) { break; }
        }
        v7 = v31;
    }

    // 2. Parity validation check
    let v25 = 0x8000000000000000u64;
    if (v7 as u64 | v25) != (v9 as u64 | v25) {
        HandleFatalRelocError();
    }

    // 3. Fallback tracking execution block under active relocation flags
    if v11 != 0 {
        v31 = 0;
        let mut v26: u32 = 0;
        if (v10 & 0x3FFFFFFF) != 0 {
            loop {
                let entries_base_ptr = core::ptr::addr_of!((*v6).entries[v26 as usize]);
                let calculated_rva = ((*entries_base_ptr) >> 30) & 0xFFFFFFF;
                let mut v27 = (0x140000000u64 + calculated_rva) as *mut u8;

                let dword_entry_val = *(entries_base_ptr as *const u32);
                if (dword_entry_val & 0xFFFFFFF) != 0 {
                    let mut v28 = dword_entry_val & 0xFFFFFFF;
                    let v32_func_ptr = *(v32.add(1) as *const usize);
                    let stream_hasher_fn_short = core::mem::transmute::<
                        usize,
                        unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                    >(v32_func_ptr);

                    loop {
                        let v25_val = *v27 as u64;
                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v32),
                            core::ptr::addr_of_mut!(v31),
                            v25_val,
                        );
                        v27 = v27.add(1);
                        v28 -= 1;
                        if v28 == 0 { break; }
                    }
                }
                v26 += 1;
                if v26 >= (v10 & 0x3FFFFFFF) { break; }
            }
            v4 = v31;
        }

        let target_val = v4 & 0x7FFFFFFFFFFFFFFF;
        core::intrinsics::atomic_cxchg_seqcst_seqcst(v5, v9, target_val);
    }

    1
}

pub unsafe fn ProcessReloc_Rva33_Len3(
    pRelocBlockRva: *mut RelocTableHeader,
    a2: *mut u32,
) -> i64 {
    let v2 = *a2 as u64;
    let entry_count_marker = (*pRelocBlockRva).entry_count_marker as usize;
    let mut v4: i64 = 0;

    // v34 = off_1403C45A0;
    let mut v34 = off_1403C45A0 as *mut usize;
    let mut v32: u32 = 0;

    // v6 = entry_count_marker + 335544320;
    let v6 = (entry_count_marker + 335544320) as *mut RelocTableHeader;
    let v5 = (0x140000000u64 + v2) as *mut i64;

    let mut v7: i64 = 0;
    let v42 = v6;
    let mut v33: i64 = 0;
    let mut v8: u32 = 0;
    let v9 = *v5;
    let mut v10 = (*v6).entry_count_marker;
    let mut v11 = (*v5 as u64) >> 63;
    let v44 = *v5;
    let v35 = v11;

    if (v10 & 0x7FFFFFFE) != 0 {
        loop {
            let mut v12: u32 = 0;
            let mut v37: i64 = 0;
            let mut v13: u8 = 0;
            let mut v36: u32 = 0;
            let mut v38: i32 = 0;

            // v14 = (unsigned __int64)v6->entries[(unsigned int)v8];
            let v14 = (*v6).entries[v8 as usize];

            // v15 = (v14 >> 33) & 0xFFFFFFF;
            let v15 = (v14 >> 33) & 0xFFFFFFF;

            // v16 = (v14 >> 3) & 0xFFFFFFF;
            let v16 = (v14 >> 3) & 0xFFFFFFF;

            let mut v17 = 0x140000000u64 + v15;
            let v41 = v17;

            // Dynamic storage tracking variables mapping continuous local stack spans
            let mut v39: u32 = 0;
            let mut v40: u32 = 0;

            if v11 != 0 {
                let stack_context_ptr = core::ptr::addr_of_mut!(v36) as usize;
                binary_search_lookup(stack_context_ptr as i64, v15, v16 as i64);

                v39 = *((stack_context_ptr + 24) as *const u32);
                v40 = *((stack_context_ptr + 28) as *const u32);
                v37 = *((stack_context_ptr + 4) as *const i64);

                if v40 < v36 && (*((v37 + 4 * v40 as i64) as *const u32) & 0xFFFFFFF) < v39 {
                    v12 = *((v37 + 4 * v40 as i64) as *const u32);
                    v40 += 1;
                    *((stack_context_ptr + 28) as *mut u32) = v40;
                    v13 = 1;
                }
                v17 = v41;
                v11 = v35;
                v8 = v32;
            }

            let mut v18: u32 = 0;
            if v16 != 0 {
                loop {
                    if v11 != 0 && v13 != 0 && (v18 + v15 as u32) == (v12 & 0xFFFFFFF) {
                        let v19 = qword_14045FD10.wrapping_sub(0x140000000);
                        let v20 = v12 >> 28;
                        let mut v21: u32 = 0;
                        let mut v22: u64;
                        let mut v45 = [0u64; 2];

                        if v20 != 0 {
                            if v20 == 3 {
                                v22 = 4;
                                let source_val = *((v18 as u64 + v17) as *const u32);
                                let casting_ptr = v45.as_mut_ptr() as *mut u32;
                                *casting_ptr = source_val.wrapping_add(v19 as u32);
                            } else if v20 == 10 {
                                v22 = 8;
                                let source_val = *((v18 as u64 + v17) as *const u64);
                                v45 = source_val.wrapping_add(v19);
                            } else {
                                v22 = u64::MAX;
                            }

                            let mut v23: u64 = 0;
                            let v34_func_ptr = *(v34.add(1) as *const usize);
                            let stream_hasher_fn = core::mem::transmute::<
                                usize,
                                unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                            >(v34_func_ptr);

                            loop {
                                v8 = *((v45.as_ptr() as usize + v23 as usize) as *const u8) as u32;
                                stream_hasher_fn(
                                    core::ptr::addr_of_mut!(v34),
                                    core::ptr::addr_of_mut!(v33),
                                    v8 as u64,
                                );
                                v21 += 1;
                                v23 = v21 as u64;
                                if (v21 as u64) >= v22 { break; }
                            }
                        }

                        v18 += v21;
                        if v40 >= v36 {
                            v13 = 0;
                        } else {
                            let current_check_val = *((v37 + 4 * v40 as i64) as *const u32);
                            if (current_check_val & 0xFFFFFFF) >= v39 {
                                v13 = 0;
                            } else {
                                v12 = *((v37 + 4 * v40 as i64) as *const u32);
                                v40 += 1;
                                v13 = 1;
                            }
                        }
                    } else {
                        let v24_val = *((v18 as u64 + v17) as *const u8) as u64;
                        let v34_func_ptr = *(v34.add(1) as *const usize);
                        let stream_hasher_fn_short = core::mem::transmute::<
                            usize,
                            unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                        >(v34_func_ptr);

                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v34),
                            core::ptr::addr_of_mut!(v33),
                            v24_val,
                        );
                        v18 += 1;
                    }

                    v11 = v35;
                    v17 = v41;
                    if v18 >= (v16 as u32) { break; }
                }
                v8 = v32;
                v6 = v42;
            }

            v10 = (*v6).entry_count_marker;
            v11 = v35;
            v8 += 1;
            v32 = v8;

            if (v8 as u32) >= ((v10 >> 1) & 0x3FFFFFFF) { break; }
        }
        v7 = v33;
    }

    // 2. Parity validation check
    let v25 = 0x8000000000000000u64;
    if (v7 as u64 | v25) != (v9 as u64 | v25) {
        HandleFatalRelocError();
    }

    // 3. Fallback tracking execution block under active relocation flags
    if v11 != 0 {
        v33 = 0;
        let mut v26: u32 = 0;
        if (v10 & 0x7FFFFFFE) != 0 {
            loop {
                let mut v27 = (*v6).entries[v26 as usize];
                let mut v28 = (0x140000000u64 + ((v27 >> 33) & 0xFFFFFFF)) as *mut u8;
                let v29 = ((v27 >> 3) & 0xFFFFFFF) as u32;

                if v29 != 0 {
                    let mut v30 = v29;
                    let v34_func_ptr = *(v34.add(1) as *const usize);
                    let stream_hasher_fn_short = core::mem::transmute::<
                        usize,
                        unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                    >(v34_func_ptr);

                    loop {
                        let v25_val = *v28 as u64;
                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v34),
                            core::ptr::addr_of_mut!(v33),
                            v25_val,
                        );
                        v27 = v27.add(1);
                        v30 -= 1;
                        if v30 == 0 { break; }
                    }
                }
                v26 += 1;
                if v26 >= ((v10 >> 1) & 0x3FFFFFFF) { break; }
            }
            v4 = v33;
        }

        let target_val = v4 & 0x7FFFFFFFFFFFFFFF;
        core::intrinsics::atomic_cxchg_seqcst_seqcst(v5, v9, target_val);
    }

    1
}

pub unsafe fn ProcessReloc_Rva36_Len0(
    pRelocBlockRva: *mut RelocTableHeader,
    a2: *mut u32,
) -> i64 {
    let v2 = *a2 as u64;
    let entry_count_marker = (*pRelocBlockRva).entry_count_marker as usize;
    let mut v4: i64 = 0;

    // v32 = off_1403C4560;
    let mut v32 = off_1403C4560 as *mut usize;
    let mut v30: u32 = 0;

    // v6 = entry_count_marker + 335544320;
    let mut v6 = (entry_count_marker + 335544320) as *mut RelocTableHeader;
    let v5 = (0x140000000u64 + v2) as *mut i64;

    let mut v7: i64 = 0;
    let v41 = v6;
    let mut v31: i64 = 0;
    let mut v8: u32 = 0;
    let v9 = *v5;
    let mut v10 = (*v6).entry_count_marker;
    let mut v11 = (*v5 as u64) >> 63;
    let v43 = *v5;
    let v33 = v11;

    if (v10 & 0xFFFFFFFC) != 0 {
        loop {
            let mut v12: u32 = 0;
            let mut v35: i64 = 0;
            let mut v13: u8 = 0;
            let mut v34: u32 = 0;
            let mut v36: i32 = 0;

            // v14 = *(_QWORD *)&v6->entries[(unsigned int)v8] >> 36;
            let entries_base_ptr = core::ptr::addr_of!((*v6).entries[v8 as usize]);
            let v14 = (*entries_base_ptr) >> 36;

            // v15 = *(_DWORD *)&v6->entries[(unsigned int)v8] & 0xFFFFFFF;
            let v15 = *(entries_base_ptr as *const u32) & 0xFFFFFFF;

            let v40 = v14;
            let mut v16 = 0x140000000u64 + v14;
            let v39 = v16;

            // Dynamic storage tracking variables mapping continuous local stack spans
            let mut v37: u32 = 0;
            let mut v38: u32 = 0;

            if v11 != 0 {
                let stack_context_ptr = core::ptr::addr_of_mut!(v34) as usize;
                binary_search_lookup(stack_context_ptr as i64, v14, v15 as i64);

                v37 = *((stack_context_ptr + 24) as *const u32);
                v38 = *((stack_context_ptr + 28) as *const u32);
                v35 = *((stack_context_ptr + 4) as *const i64);

                if v38 < v34 && (*((v35 + 4 * v38 as i64) as *const u32) & 0xFFFFFFF) < v37 {
                    v12 = *((v35 + 4 * v38 as i64) as *const u32);
                    v38 += 1;
                    *((stack_context_ptr + 28) as *mut u32) = v38;
                    v13 = 1;
                }
                v16 = v39;
                v11 = v33;
                v8 = v30;
            }

            let mut v17: u32 = 0;
            if v15 != 0 {
                let v18 = v40 as u32;
                loop {
                    if v11 != 0 && v13 != 0 && (v17 + v18) == (v12 & 0xFFFFFFF) {
                        let v19 = qword_14045FD10.wrapping_sub(0x140000000);
                        let v20 = v12 >> 28;
                        let mut v21: u32 = 0;
                        let mut v22: u64;
                        let mut v44 = [0u64; 2];

                        if v20 != 0 {
                            if v20 == 3 {
                                v22 = 4;
                                let source_val = *((v17 as u64 + v16) as *const u32);
                                let casting_ptr = v44.as_mut_ptr() as *mut u32;
                                *casting_ptr = source_val.wrapping_add(v19 as u32);
                            } else if v20 == 10 {
                                v22 = 8;
                                let source_val = *((v17 as u64 + v16) as *const u64);
                                v44 = source_val.wrapping_add(v19);
                            } else {
                                v22 = u64::MAX;
                            }

                            let mut v23: u64 = 0;
                            let v32_func_ptr = *(v32.add(1) as *const usize);
                            let stream_hasher_fn = core::mem::transmute::<
                                usize,
                                unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                            >(v32_func_ptr);

                            loop {
                                v8 = *((v44.as_ptr() as usize + v23 as usize) as *const u8) as u32;
                                stream_hasher_fn(
                                    core::ptr::addr_of_mut!(v32),
                                    core::ptr::addr_of_mut!(v31),
                                    v8 as u64,
                                );
                                v21 += 1;
                                v23 = v21 as u64;
                                if (v21 as u64) >= v22 { break; }
                            }
                        }

                        v17 += v21;
                        if v38 >= v34 {
                            v13 = 0;
                        } else {
                            let current_check_val = *((v35 + 4 * v38 as i64) as *const u32);
                            if (current_check_val & 0xFFFFFFF) >= v37 {
                                v13 = 0;
                            } else {
                                v12 = *((v35 + 4 * v38 as i64) as *const u32);
                                v38 += 1;
                                v13 = 1;
                            }
                        }
                    } else {
                        let v24_val = *((v17 as u64 + v16) as *const u8) as u64;
                        let v32_func_ptr = *(v32.add(1) as *const usize);
                        let stream_hasher_fn_short = core::mem::transmute::<
                            usize,
                            unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                        >(v32_func_ptr);

                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v32),
                            core::ptr::addr_of_mut!(v31),
                            v24_val,
                        );
                        v17 += 1;
                    }

                    v11 = v33;
                    v16 = v39;
                    if v17 >= v15 { break; }
                }
                v6 = v41;
                v8 = v30;
            }

            v10 = (*v6).entry_count_marker;
            v11 = v33;
            v8 += 1;
            v30 = v8;

            if v8 >= (v10 >> 2) { break; }
        }
        v7 = v31;
    }

    // 2. Parity validation check
    let v25 = 0x8000000000000000u64;
    if (v7 as u64 | v25) != (v9 as u64 | v25) {
        HandleFatalRelocError();
    }

    // 3. Fallback tracking execution block under active relocation flags
    if v11 != 0 {
        v31 = 0;
        let mut v26: u32 = 0;
        if (v10 & 0xFFFFFFFC) != 0 {
            loop {
                let entries_base_ptr = core::ptr::addr_of!((*v6).entries[v26 as usize]);
                let calculated_rva = (*entries_base_ptr) >> 36;
                let mut v27 = (0x140000000u64 + calculated_rva) as *mut u8;

                let dword_entry_val = *(entries_base_ptr as *const u32);
                if (dword_entry_val & 0xFFFFFFF) != 0 {
                    let mut v28 = dword_entry_val & 0xFFFFFFF;
                    let v32_func_ptr = *(v32.add(1) as *const usize);
                    let stream_hasher_fn_short = core::mem::transmute::<
                        usize,
                        unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                    >(v32_func_ptr);

                    loop {
                        let v25_val = *v27 as u64;
                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v32),
                            core::ptr::addr_of_mut!(v31),
                            v25_val,
                        );
                        v27 = v27.add(1);
                        v28 -= 1;
                        if v28 == 0 { break; }
                    }
                }
                v26 += 1;
                if v26 >= (v10 >> 2) { break; }
            }
            v4 = v31;
        }

        let target_val = v4 & 0x7FFFFFFFFFFFFFFF;
        core::intrinsics::atomic_cxchg_seqcst_seqcst(v5, v9, target_val);
    }

    1
}

pub unsafe fn ProcessReloc_Rva36_Len3(
    pRelocHeader: *mut RelocTableHeader,
    pRvaTargetState: *mut u32,
) -> i64 {
    let v2 = *pRvaTargetState as u64;
    let entry_count_marker = (*pRelocHeader).entry_count_marker as usize;
    let mut v4: i64 = 0;

    // v34 = off_1403C4540;
    let mut v34 = off_1403C4540 as *mut usize;
    let mut v32: u32 = 0;

    // v6 = entry_count_marker + 335544320;
    let mut v6 = (entry_count_marker + 335544320) as *mut RelocTableHeader;
    let v5 = (0x140000000u64 + v2) as *mut i64;

    let mut v7: i64 = 0;
    let v42 = v6;
    let mut v33: i64 = 0;
    let mut v8: u32 = 0;
    let v9 = *v5;
    let mut v10 = (*v6).entry_count_marker;
    let mut v11 = (*v5 as u64) >> 63;
    let v44 = *v5;
    let v35 = v11;

    if (v10 & 0xFFFFFFFC) != 0 {
        loop {
            let mut v12: u32 = 0;
            let mut v37: i64 = 0;
            let mut v13: u8 = 0;
            let mut v36: u32 = 0;
            let mut v38: i32 = 0;

            // v14 = (unsigned __int64)v6->entries[(unsigned int)v8];
            let v14 = (*v6).entries[v8 as usize];

            // v15 = v14 >> 36;
            let v15 = v14 >> 36;

            // v16 = (v14 >> 3) & 0xFFFFFFF;
            let v16 = (v14 >> 3) & 0xFFFFFFF;

            let mut v17 = 0x140000000u64 + v15;
            let v41 = v17;

            // Dynamic storage tracking variables mapping continuous local stack spans
            let mut v39: u32 = 0;
            let mut v40: u32 = 0;

            if v11 != 0 {
                let stack_context_ptr = core::ptr::addr_of_mut!(v36) as usize;
                binary_search_lookup(stack_context_ptr as i64, v15, v16 as i64);

                v39 = *((stack_context_ptr + 24) as *const u32);
                v40 = *((stack_context_ptr + 28) as *const u32);
                v37 = *((stack_context_ptr + 4) as *const i64);

                if v40 < v36 && (*((v37 + 4 * v40 as i64) as *const u32) & 0xFFFFFFF) < v39 {
                    v12 = *((v37 + 4 * v40 as i64) as *const u32);
                    v40 += 1;
                    *((stack_context_ptr + 28) as *mut u32) = v40;
                    v13 = 1;
                }
                v17 = v41;
                v11 = v35;
                v8 = v32;
            }

            let mut v18: u32 = 0;
            if v16 != 0 {
                loop {
                    if v11 != 0 && v13 != 0 && (v18 + v15 as u32) == (v12 & 0xFFFFFFF) {
                        let v19 = qword_14045FD10.wrapping_sub(0x140000000);
                        let v20 = v12 >> 28;
                        let mut v21: u32 = 0;
                        let mut v22: u64;
                        let mut v45 = [0u64; 2];

                        if v20 != 0 {
                            if v20 == 3 {
                                v22 = 4;
                                let source_val = *((v18 as u64 + v17) as *const u32);
                                let casting_ptr = v45.as_mut_ptr() as *mut u32;
                                *casting_ptr = source_val.wrapping_add(v19 as u32);
                            } else if v20 == 10 {
                                v22 = 8;
                                let source_val = *((v18 as u64 + v17) as *const u64);
                                v45 = source_val.wrapping_add(v19);
                            } else {
                                v22 = u64::MAX;
                            }

                            let mut v23: u64 = 0;
                            let v34_func_ptr = *(v34.add(1) as *const usize);
                            let stream_hasher_fn = core::mem::transmute::<
                                usize,
                                unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                            >(v34_func_ptr);

                            loop {
                                v8 = *((v45.as_ptr() as usize + v23 as usize) as *const u8) as u32;
                                stream_hasher_fn(
                                    core::ptr::addr_of_mut!(v34),
                                    core::ptr::addr_of_mut!(v33),
                                    v8 as u64,
                                );
                                v21 += 1;
                                v23 = v21 as u64;
                                if (v21 as u64) >= v22 { break; }
                            }
                        }

                        v18 += v21;
                        if v40 >= v36 {
                            v13 = 0;
                        } else {
                            let current_check_val = *((v37 + 4 * v40 as i64) as *const u32);
                            if (current_check_val & 0xFFFFFFF) >= v39 {
                                v13 = 0;
                            } else {
                                v12 = *((v37 + 4 * v40 as i64) as *const u32);
                                v40 += 1;
                                v13 = 1;
                            }
                        }
                    } else {
                        let v24_val = *((v18 as u64 + v17) as *const u8) as u64;
                        let v34_func_ptr = *(v34.add(1) as *const usize);
                        let stream_hasher_fn_short = core::mem::transmute::<
                            usize,
                            unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                        >(v34_func_ptr);

                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v34),
                            core::ptr::addr_of_mut!(v33),
                            v24_val,
                        );
                        v18 += 1;
                    }

                    v11 = v35;
                    v17 = v41;
                    if v18 >= (v16 as u32) { break; }
                }
                v6 = v42;
                v8 = v32;
            }

            v10 = (*v6).entry_count_marker;
            v11 = v35;
            v8 += 1;
            v32 = v8;

            if v8 >= (v10 >> 2) { break; }
        }
        v7 = v33;
    }

    // 2. Parity validation check
    let v25 = 0x8000000000000000u64;
    if (v7 as u64 | v25) != (v9 as u64 | v25) {
        HandleFatalRelocError();
    }

    // 3. Fallback tracking execution block under active relocation flags
    if v11 != 0 {
        v33 = 0;
        let mut v26: u32 = 0;
        if (v10 & 0xFFFFFFFC) != 0 {
            loop {
                let mut v27 = (*v6).entries[v26 as usize];
                let mut v28 = (0x140000000u64 + (v27 >> 36)) as *mut u8;
                let v29 = ((v27 >> 3) & 0xFFFFFFF) as u32;

                if v29 != 0 {
                    let mut v30 = v29;
                    let v34_func_ptr = *(v34.add(1) as *const usize);
                    let stream_hasher_fn_short = core::mem::transmute::<
                        usize,
                        unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                    >(v34_func_ptr);

                    loop {
                        let v25_val = *v28 as u64;
                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v34),
                            core::ptr::addr_of_mut!(v33),
                            v25_val,
                        );
                        v27 = v27.add(1);
                        v30 -= 1;
                        if v30 == 0 { break; }
                    }
                }
                v26 += 1;
                if v26 >= (v10 >> 2) { break; }
            }
            v4 = v33;
        }

        let target_val = v4 & 0x7FFFFFFFFFFFFFFF;
        core::intrinsics::atomic_cxchg_seqcst_seqcst(v5, v9, target_val);
    }

    1
}

pub unsafe fn ProcessReloc_Rva3_Len31(
    pRelocBlockRva: *mut RelocTableHeader,
    a2: *mut u32,
) -> i64 {
    let v2 = *a2 as u64;
    let entry_count_marker = (*pRelocBlockRva).entry_count_marker as usize;
    let mut v4: i64 = 0;

    // v34 = off_1403C45B0;
    let mut v34 = off_1403C45B0 as *mut usize;
    let mut v32: u32 = 0;

    // v6 = entry_count_marker + 335544320;
    let mut v6 = (entry_count_marker + 335544320) as *mut RelocTableHeader;
    let v5 = (0x140000000u64 + v2) as *mut i64;

    let mut v7: i64 = 0;
    let v42 = v6;
    let mut v33: i64 = 0;
    let mut v8: u32 = 0;
    let v9 = *v5;
    let mut v10 = (*v6).entry_count_marker;
    let mut v11 = (*v5 as u64) >> 63;
    let v44 = *v5;
    let v35 = v11;

    if (v10 & 0x7FFFFFFE) != 0 {
        loop {
            let mut v12: u32 = 0;
            let mut v37: i64 = 0;
            let mut v13: u8 = 0;
            let mut v36: u32 = 0;
            let mut v38: i32 = 0;

            // v14 = (unsigned __int64)v6->entries[(unsigned int)v8];
            let v14 = (*v6).entries[v8 as usize];

            // v15 = (v14 >> 3) & 0xFFFFFFF;
            let v15 = (v14 >> 3) & 0xFFFFFFF;

            // v16 = (v14 >> 31) & 0xFFFFFFF;
            let v16 = (v14 >> 31) & 0xFFFFFFF;

            let mut v17 = 0x140000000u64 + v15;
            let v41 = v17;

            // Dynamic storage tracking variables mapping continuous local stack spans
            let mut v39: u32 = 0;
            let mut v40: u32 = 0;

            if v11 != 0 {
                let stack_context_ptr = core::ptr::addr_of_mut!(v36) as usize;
                binary_search_lookup(stack_context_ptr as i64, v15, v16 as i64);

                v39 = *((stack_context_ptr + 24) as *const u32);
                v40 = *((stack_context_ptr + 28) as *const u32);
                v37 = *((stack_context_ptr + 4) as *const i64);

                if v40 < v36 && (*((v37 + 4 * v40 as i64) as *const u32) & 0xFFFFFFF) < v39 {
                    v12 = *((v37 + 4 * v40 as i64) as *const u32);
                    v40 += 1;
                    *((stack_context_ptr + 28) as *mut u32) = v40;
                    v13 = 1;
                }
                v17 = v41;
                v11 = v35;
                v8 = v32;
            }

            let mut v18: u32 = 0;
            if v16 != 0 {
                loop {
                    if v11 != 0 && v13 != 0 && (v18 + v15 as u32) == (v12 & 0xFFFFFFF) {
                        let v19 = qword_14045FD10.wrapping_sub(0x140000000);
                        let v20 = v12 >> 28;
                        let mut v21: u32 = 0;
                        let mut v22: u64;
                        let mut v45 = [0u64; 2];

                        if v20 != 0 {
                            if v20 == 3 {
                                v22 = 4;
                                let source_val = *((v18 as u64 + v17) as *const u32);
                                let casting_ptr = v45.as_mut_ptr() as *mut u32;
                                *casting_ptr = source_val.wrapping_add(v19 as u32);
                            } else if v20 == 10 {
                                v22 = 8;
                                let source_val = *((v18 as u64 + v17) as *const u64);
                                v45 = source_val.wrapping_add(v19);
                            } else {
                                v22 = u64::MAX;
                            }

                            let mut v23: u64 = 0;
                            let v34_func_ptr = *(v34.add(1) as *const usize);
                            let stream_hasher_fn = core::mem::transmute::<
                                usize,
                                unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                            >(v34_func_ptr);

                            loop {
                                v8 = *((v45.as_ptr() as usize + v23 as usize) as *const u8) as u32;
                                stream_hasher_fn(
                                    core::ptr::addr_of_mut!(v34),
                                    core::ptr::addr_of_mut!(v33),
                                    v8 as u64,
                                );
                                v21 += 1;
                                v23 = v21 as u64;
                                if (v21 as u64) >= v22 { break; }
                            }
                        }

                        v18 += v21;
                        if v40 >= v36 {
                            v13 = 0;
                        } else {
                            let current_check_val = *((v37 + 4 * v40 as i64) as *const u32);
                            if (current_check_val & 0xFFFFFFF) >= v39 {
                                v13 = 0;
                            } else {
                                v12 = *((v37 + 4 * v40 as i64) as *const u32);
                                v40 += 1;
                                v13 = 1;
                            }
                        }
                    } else {
                        let v24_val = *((v18 as u64 + v17) as *const u8) as u64;
                        let v34_func_ptr = *(v34.add(1) as *const usize);
                        let stream_hasher_fn_short = core::mem::transmute::<
                            usize,
                            unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                        >(v34_func_ptr);

                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v34),
                            core::ptr::addr_of_mut!(v33),
                            v24_val,
                        );
                        v18 += 1;
                    }

                    v11 = v35;
                    v17 = v41;
                    if v18 >= (v16 as u32) { break; }
                }
                v8 = v32;
                v6 = v42;
            }

            v10 = (*v6).entry_count_marker;
            v11 = v35;
            v8 += 1;
            v32 = v8;

            if (v8 as u32) >= ((v10 >> 1) & 0x3FFFFFFF) { break; }
        }
        v7 = v33;
    }

    // 2. Parity validation check
    let v25 = 0x8000000000000000u64;
    if (v7 as u64 | v25) != (v9 as u64 | v25) {
        HandleFatalRelocError();
    }

    // 3. Fallback tracking execution block under active relocation flags
    if v11 != 0 {
        v33 = 0;
        let mut v26: u32 = 0;
        if (v10 & 0x7FFFFFFE) != 0 {
            loop {
                let v27 = (*v6).entries[v26 as usize];
                let mut v28 = (0x140000000u64 + ((v27 >> 3) & 0xFFFFFFF)) as *mut u8;
                let v29 = ((v27 >> 31) & 0xFFFFFFF) as u32;

                if v29 != 0 {
                    let mut v30 = v29;
                    let v34_func_ptr = *(v34.add(1) as *const usize);
                    let stream_hasher_fn_short = core::mem::transmute::<
                        usize,
                        unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                    >(v34_func_ptr);

                    loop {
                        let v25_val = *v28 as u64;
                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v34),
                            core::ptr::addr_of_mut!(v33),
                            v25_val,
                        );
                        v28 = v28.add(1);
                        v30 -= 1;
                        if v30 == 0 { break; }
                    }
                }
                v26 += 1;
                if v26 >= ((v10 >> 1) & 0x3FFFFFFF) { break; }
            }
            v4 = v33;
        }

        let target_val = v4 & 0x7FFFFFFFFFFFFFFF;
        core::intrinsics::atomic_cxchg_seqcst_seqcst(v5, v9, target_val);
    }

    1
}

pub unsafe fn ProcessReloc_Rva3_Len36(
    pRelocBlockRva: *mut RelocTableHeader,
    pTargetStateRva: *mut u32,
) -> i64 {
    let v2 = *pTargetStateRva as u64;
    let entry_count_marker = (*pRelocBlockRva).entry_count_marker as usize;
    let mut v4: i64 = 0;

    // v35 = off_1403C45C0;
    let mut v35 = off_1403C45C0 as *mut usize;
    let mut v33: u32 = 0;

    // v6 = entry_count_marker + 335544320;
    let mut v6 = (entry_count_marker + 335544320) as *mut RelocTableHeader;
    let v5 = (0x140000000u64 + v2) as *mut i64;

    let mut v7: i64 = 0;
    let v43 = v6;
    let mut v34: i64 = 0;
    let mut v8: u32 = 0;
    let v9 = *v5;
    let mut v10 = (*v6).entry_count_marker;
    let mut v11 = (*v5 as u64) >> 63;
    let v45 = *v5;
    let v36 = v11;

    if (v10 & 0xFFFFFFFC) != 0 {
        loop {
            let mut v12: u32 = 0;
            let mut v38: i64 = 0;
            let mut v13: u8 = 0;
            let mut v37: u32 = 0;
            let mut v39: i32 = 0;

            // v14 = v6->entries[(unsigned int)v8];
            let v14 = (*v6).entries[v8 as usize];

            // v15 = (v14 >> 3) & 0xFFFFFFF;
            let v15 = (v14 >> 3) & 0xFFFFFFF;

            // v16 = v14 >> 36;
            let v16 = v14 >> 36;

            let mut v17 = 0x140000000u64 + v15;
            let v42 = v17;

            // Dynamic storage tracking variables mapping continuous local stack spans
            let mut v40: u32 = 0;
            let mut v41: u32 = 0;

            if v11 != 0 {
                let stack_context_ptr = core::ptr::addr_of_mut!(v37) as usize;
                binary_search_lookup(stack_context_ptr as i64 as *mut usize, v15, v16 as i32);

                v40 = *((stack_context_ptr + 24) as *const u32);
                v41 = *((stack_context_ptr + 28) as *const u32);
                v38 = *((stack_context_ptr + 4) as *const i64);

                if v41 < v37 && (*((v38 + 4 * v41 as i64) as *const u32) & 0xFFFFFFF) < v40 {
                    v12 = *((v38 + 4 * v41 as i64) as *const u32);
                    v41 += 1;
                    *((stack_context_ptr + 28) as *mut u32) = v41;
                    v13 = 1;
                }
                v17 = v42;
                v11 = v36;
                v8 = v33;
            }

            let mut v18: u32 = 0;
            if v16 != 0 {
                loop {
                    if v11 != 0 && v13 != 0 && (v18 + v15 as u32) == (v12 & 0xFFFFFFF) {
                        let v19 = qword_14045FD10.wrapping_sub(0x140000000);
                        let v20 = v12 >> 28;
                        let mut v21: u32 = 0;
                        let mut v22: u64;
                        let mut v46 = [0u64; 2];

                        if v20 != 0 {
                            if v20 == 3 {
                                v22 = 4;
                                let source_val = *((v18 as u64 + v17) as *const u32);
                                let casting_ptr = v46.as_mut_ptr() as *mut u32;
                                *casting_ptr = source_val.wrapping_add(v19 as u32);
                            } else if v20 == 10 {
                                v22 = 8;
                                let source_val = *((v18 as u64 + v17) as *const u64);
                                v46 = source_val.wrapping_add(v19);
                            } else {
                                v22 = u64::MAX;
                            }

                            let mut v23: u64 = 0;
                            let v35_func_ptr = *(v35.add(1) as *const usize);
                            let stream_hasher_fn = core::mem::transmute::<
                                usize,
                                unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                            >(v35_func_ptr);

                            loop {
                                v8 = *((v46.as_ptr() as usize + v23 as usize) as *const u8) as u32;
                                stream_hasher_fn(
                                    core::ptr::addr_of_mut!(v35),
                                    core::ptr::addr_of_mut!(v34),
                                    v8 as u64,
                                );
                                v21 += 1;
                                v23 = v21 as u64;
                                if (v21 as u64) >= v22 { break; }
                            }
                        }

                        v18 += v21;
                        if v41 >= v37 {
                            v13 = 0;
                        } else {
                            let current_check_val = *((v38 + 4 * v41 as i64) as *const u32);
                            if (current_check_val & 0xFFFFFFF) >= v40 {
                                v13 = 0;
                            } else {
                                v12 = *((v38 + 4 * v41 as i64) as *const u32);
                                v41 += 1;
                                v13 = 1;
                            }
                        }
                    } else {
                        let v24_val = *((v18 as u64 + v17) as *const u8) as u64;
                        let v35_func_ptr = *(v35.add(1) as *const usize);
                        let stream_hasher_fn_short = core::mem::transmute::<
                            usize,
                            unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                        >(v35_func_ptr);

                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v35),
                            core::ptr::addr_of_mut!(v34),
                            v24_val,
                        );
                        v18 += 1;
                    }

                    v11 = v36;
                    v17 = v42;
                    if v18 >= (v16 as u32) { break; }
                }
                v6 = v43;
                v8 = v33;
            }

            v10 = (*v6).entry_count_marker;
            v11 = v36;
            v8 += 1;
            v33 = v8;

            if v8 >= (v10 >> 2) { break; }
        }
        v7 = v34;
    }

    // 2. Parity validation check
    let v25 = 0x8000000000000000u64;
    if (v7 as u64 | v25) != (v9 as u64 | v25) {
        HandleFatalRelocError();
    }

    // 3. Fallback tracking execution block under active relocation flags
    if v11 != 0 {
        v34 = 0;
        let mut v26: u32 = 0;
        if (v10 & 0xFFFFFFFC) != 0 {
            loop {
                let v27 = (*v6).entries[v26 as usize];
                let v28 = (v27 >> 3) & 0xFFFFFFF;
                let v29 = v27 >> 36;
                let mut v30 = (0x140000000u64 + v28) as *mut u8;

                if v29 != 0 {
                    let mut v31 = v29 as u32;
                    let v35_func_ptr = *(v35.add(1) as *const usize);
                    let stream_hasher_fn_short = core::mem::transmute::<
                        usize,
                        unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                    >(v35_func_ptr);

                    loop {
                        let v25_val = *v30 as u64;
                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v35),
                            core::ptr::addr_of_mut!(v34),
                            v25_val,
                        );
                        v30 = v30.add(1);
                        v31 -= 1;
                        if v31 == 0 { break; }
                    }
                }
                v26 += 1;
                if v26 >= (v10 >> 2) { break; }
            }
            v4 = v34;
        }

        let target_val = v4 & 0x7FFFFFFFFFFFFFFF;
        core::intrinsics::atomic_cxchg_seqcst_seqcst(v5, v9, target_val);
    }

    1
}

pub unsafe fn ProcessReloc_Rva8_Len36(
    pRelocBlockRva: *mut RelocTableHeader,
    a2: *mut u32,
) -> i64 {
    let v2 = *a2 as u64;
    let entry_count_marker = (*pRelocBlockRva).entry_count_marker as usize;
    let mut v4: i64 = 0;

    // v35 = off_1403C4590;
    let mut v35 = off_1403C4590 as *mut usize;
    let mut v33: u32 = 0;

    // v6 = entry_count_marker + 335544320;
    let mut v6 = (entry_count_marker + 335544320) as *mut RelocTableHeader;
    let v5 = (0x140000000u64 + v2) as *mut i64;

    let mut v7: i64 = 0;
    let v43 = v6;
    let mut v34: i64 = 0;
    let mut v8: u32 = 0;
    let v9 = *v5;
    let mut v10 = (*v6).entry_count_marker;
    let mut v11 = (*v5 as u64) >> 63;
    let v45 = *v5;
    let v36 = v11;

    if (v10 & 0xFFFFFFFC) != 0 {
        loop {
            let mut v12: u32 = 0;
            let mut v38: i64 = 0;
            let mut v13: u8 = 0;
            let mut v37: u32 = 0;
            let mut v39: i32 = 0;

            // v14 = (unsigned __int64)v6->entries[(unsigned int)v8];
            let v14 = (*v6).entries[v8 as usize];

            // v15 = (v14 >> 8) & 0xFFFFFFF;
            let v15 = (v14 >> 8) & 0xFFFFFFF;

            // v16 = v14 >> 36;
            let v16 = v14 >> 36;

            let mut v17 = 0x140000000u64 + v15;
            let v42 = v17;

            // Dynamic storage tracking variables mapping continuous local stack spans
            let mut v40: u32 = 0;
            let mut v41: u32 = 0;

            if v11 != 0 {
                let stack_context_ptr = core::ptr::addr_of_mut!(v37) as usize;
                binary_search_lookup(stack_context_ptr as i64, v15, v16 as i64);

                v40 = *((stack_context_ptr + 24) as *const u32);
                v41 = *((stack_context_ptr + 28) as *const u32);
                v38 = *((stack_context_ptr + 4) as *const i64);

                if v41 < v37 && (*((v38 + 4 * v41 as i64) as *const u32) & 0xFFFFFFF) < v40 {
                    v12 = *((v38 + 4 * v41 as i64) as *const u32);
                    v41 += 1;
                    *((stack_context_ptr + 28) as *mut u32) = v41;
                    v13 = 1;
                }
                v17 = v42;
                v11 = v36;
                v8 = v33;
            }

            let mut v18: u32 = 0;
            if v16 != 0 {
                loop {
                    if v11 != 0 && v13 != 0 && (v18 + v15 as u32) == (v12 & 0xFFFFFFF) {
                        let v19 = qword_14045FD10.wrapping_sub(0x140000000);
                        let v20 = v12 >> 28;
                        let mut v21: u32 = 0;
                        let mut v22: u64;
                        let mut v46 = [0u64; 2];

                        if v20 != 0 {
                            if v20 == 3 {
                                v22 = 4;
                                let source_val = *((v18 as u64 + v17) as *const u32);
                                let casting_ptr = v46.as_mut_ptr() as *mut u32;
                                *casting_ptr = source_val.wrapping_add(v19 as u32);
                            } else if v20 == 10 {
                                v22 = 8;
                                let source_val = *((v18 as u64 + v17) as *const u64);
                                v46 = source_val.wrapping_add(v19);
                            } else {
                                v22 = u64::MAX;
                            }

                            let mut v23: u64 = 0;
                            let v35_func_ptr = *(v35.add(1) as *const usize);
                            let stream_hasher_fn = core::mem::transmute::<
                                usize,
                                unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                            >(v35_func_ptr);

                            loop {
                                v8 = *((v46.as_ptr() as usize + v23 as usize) as *const u8) as u32;
                                stream_hasher_fn(
                                    core::ptr::addr_of_mut!(v35),
                                    core::ptr::addr_of_mut!(v34),
                                    v8 as u64,
                                );
                                v21 += 1;
                                v23 = v21 as u64;
                                if (v21 as u64) >= v22 { break; }
                            }
                        }

                        v18 += v21;
                        if v41 >= v37 {
                            v13 = 0;
                        } else {
                            let current_check_val = *((v38 + 4 * v41 as i64) as *const u32);
                            if (current_check_val & 0xFFFFFFF) >= v40 {
                                v13 = 0;
                            } else {
                                v12 = *((v38 + 4 * v41 as i64) as *const u32);
                                v41 += 1;
                                v13 = 1;
                            }
                        }
                    } else {
                        let v24_val = *((v18 as u64 + v17) as *const u8) as u64;
                        let v35_func_ptr = *(v35.add(1) as *const usize);
                        let stream_hasher_fn_short = core::mem::transmute::<
                            usize,
                            unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                        >(v35_func_ptr);

                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v35),
                            core::ptr::addr_of_mut!(v34),
                            v24_val,
                        );
                        v18 += 1;
                    }

                    v11 = v36;
                    v17 = v42;
                    if v18 >= (v16 as u32) { break; }
                }
                v6 = v43;
                v8 = v33;
            }

            v10 = (*v6).entry_count_marker;
            v11 = v36;
            v8 += 1;
            v33 = v8;

            if v8 >= (v10 >> 2) { break; }
        }
        v7 = v34;
    }

    // 2. Parity validation check
    let v25 = 0x8000000000000000u64;
    if (v7 as u64 | v25) != (v9 as u64 | v25) {
        HandleFatalRelocError();
    }

    // 3. Fallback tracking execution block under active relocation flags
    if v11 != 0 {
        v34 = 0;
        let mut v26: u32 = 0;
        if (v10 & 0xFFFFFFFC) != 0 {
            loop {
                let v27 = (*v6).entries[v26 as usize];
                let v28 = (v27 >> 8) & 0xFFFFFFF;
                let v29 = v27 >> 36;
                let mut v30 = (0x140000000u64 + v28) as *mut u8;

                if v29 != 0 {
                    let mut v31 = v29 as u32;
                    let v35_func_ptr = *(v35.add(1) as *const usize);
                    let stream_hasher_fn_short = core::mem::transmute::<
                        usize,
                        unsafe extern "fastcall" fn(*mut *mut usize, *mut i64, u64)
                    >(v35_func_ptr);

                    loop {
                        let v25_val = *v30 as u64;
                        stream_hasher_fn_short(
                            core::ptr::addr_of_mut!(v35),
                            core::ptr::addr_of_mut!(v34),
                            v25_val,
                        );
                        v30 = v30.add(1);
                        v31 -= 1;
                        if v31 == 0 { break; }
                    }
                }
                v26 += 1;
                if v26 >= (v10 >> 2) { break; }
            }
            v4 = v34;
        }

        let target_val = v4 & 0x7FFFFFFFFFFFFFFF;
        core::intrinsics::atomic_cxchg_seqcst_seqcst(v5, v9, target_val);
    }

    1
}