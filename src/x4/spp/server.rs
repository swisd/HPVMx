use core::ffi::c_void;
use crate::u16_str;
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{x4_getprocessheap, x4_heapfree, x4_ntquerysysteminformation};
use crate::x4::globals::{GlobalPtrSppNamespace, SERVER_PROPS_NAMESPACE};
use crate::x4::spp::format::SppFormatString;
use crate::x4::spp::string::{SppGetAndDuplicateString, SppStringStartsWith};
use crate::x4::types::{ISppCollection, HRESULT, ISppPluginParams, PCWSTR, ISppPropertyBag};

pub unsafe fn SppClearServerValidationProperties(
    pszBaseNamespace: PCWSTR,
) -> HRESULT {
    let mut v23: *mut u16 = core::ptr::null_mut();
    let mut ppsz_destination: *mut u16 = core::ptr::null_mut();
    let mut v22: *mut u16 = core::ptr::null_mut();

    let mut v4 = SppFormatString(
        &mut v23,
        u16_str!("%s:%s"),
        pszBaseNamespace,
        u16_str!("serverproperties"),
    );
    let v5 = v23;
    let mut v6 = v4;

    if v4 < 0 {
        HandleSubsystemError(v4);
    } else {
        let global_ptr = GlobalPtrSppNamespace;
        let remove_prop_fn = ((*(*global_ptr).lpVtbl).RemoveProperty);

        v4 = remove_prop_fn(
            global_ptr as *mut c_void,
            1,
            SERVER_PROPS_NAMESPACE,
            v5,
        );
        v6 = v4;

        if v4 < 0 {
            HandleSubsystemError(v4);
        } else {
            v4 = SppFormatString(
                &mut ppsz_destination,
                u16_str!("%s:%s"),
                pszBaseNamespace,
                u16_str!("signature"),
            );
            v6 = v4;

            if v4 < 0 {
                HandleSubsystemError(v4);
            } else {
                let v2 = ppsz_destination;
                v4 = remove_prop_fn(
                    global_ptr as *mut c_void,
                    1,
                    SERVER_PROPS_NAMESPACE,
                    v2,
                );
                v6 = v4;

                if v4 < 0 {
                    HandleSubsystemError(v4);
                } else {
                    v4 = SppFormatString(
                        &mut v22,
                        u16_str!("%s:%s"),
                        pszBaseNamespace,
                        u16_str!("avskey"),
                    );
                    v6 = v4;

                    if v4 < 0 {
                        HandleSubsystemError(v4);
                    } else {
                        let v3 = v22;
                        v4 = remove_prop_fn(
                            global_ptr as *mut c_void,
                            1,
                            SERVER_PROPS_NAMESPACE,
                            v3,
                        );
                        v6 = v4;

                        if v4 < 0 {
                            HandleSubsystemError(v4);
                        } else {
                            v4 = remove_prop_fn(
                                global_ptr as *mut c_void,
                                1,
                                SERVER_PROPS_NAMESPACE,
                                pszBaseNamespace,
                            );
                            v6 = v4;

                            if v4 < 0 {
                                HandleSubsystemError(v4);
                            }
                        }
                    }
                }
            }
        }
    }

    LogTraceEvent(v6);

    let v3 = v22;
    if !v3.is_null() {
        let process_heap = x4_getprocessheap();
        // Allocation buffer offset adjustment matching C source (v3 - 2 wide chars)
        x4_heapfree(process_heap, 0, v3.offset(-2) as *mut c_void);
        LogTraceEvent(0);
    }

    let v2 = ppsz_destination;
    if !v2.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, v2.offset(-2) as *mut c_void);
        LogTraceEvent(0);
    }

    if !v5.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, v5.offset(-2) as *mut c_void);
        LogTraceEvent(0);
    }

    v6
}

pub unsafe fn SppFlushOrResetServerProperties(
    pszTargetPropertyRoot: PCWSTR,
) -> HRESULT {
    let mut p_collection: *mut ISppPropertyBag = core::ptr::null_mut();
    let mut count: u32 = 0;
    let mut item_data: [u64; 2] = [0; 2];

    let global_ptr = GlobalPtrSppNamespace;
    let query_fn = ((*(*global_ptr).lpVtbl).QueryProperties);

    let mut status = query_fn(
        global_ptr as *mut c_void,
        1,
        SERVER_PROPS_NAMESPACE,
        pszTargetPropertyRoot,
        &mut p_collection,
    );

    if status >= 0 {
        let collection_vtbl = (*p_collection).lpVtbl;
        status = ((*collection_vtbl).get_count)(p_collection as *mut c_void, &mut count);

        if status >= 0 {
            let mut index: u32 = 0;
            if count != 0 {
                loop {
                    status = ((*collection_vtbl).get_item)(
                        p_collection as *mut c_void,
                        index,
                        &mut item_data,
                    );
                    if status < 0 {
                        break;
                    }

                    // pszBaseNamespace is the first 64-bit field in the item tuple
                    let base_namespace = item_data[0] as PCWSTR;
                    status = SppClearServerValidationProperties(base_namespace);
                    if status < 0 {
                        break;
                    }

                    index += 1;
                    if index >= count {
                        // All elements processed successfully, proceed to root cleanup
                        let remove_fn = ((*(*global_ptr).lpVtbl).RemoveProperty);
                        status = remove_fn(
                            global_ptr as *mut c_void,
                            1,
                            SERVER_PROPS_NAMESPACE,
                            pszTargetPropertyRoot,
                        );
                        break;
                    }
                }
            } else {
                let remove_fn = ((*(*global_ptr).lpVtbl).RemoveProperty);
                status = remove_fn(
                    global_ptr as *mut c_void,
                    1,
                    SERVER_PROPS_NAMESPACE,
                    pszTargetPropertyRoot,
                );
            }
        }
    }

    if status < 0 {
        HandleSubsystemError(status);
    }

    LogTraceEvent(status);

    if !p_collection.is_null() {
        let release_fn = (*(*p_collection).lpVtbl).release;
        release_fn(p_collection as *mut c_void);
    }

    status
}

pub unsafe fn SppValidateServerProperties(
    pszContextPrefix: PCWSTR,
) -> HRESULT {
    let mut p_collection: *mut ISppCollection = core::ptr::null_mut();
    let mut p_plugin_params: *mut ISppPluginParams = core::ptr::null_mut();
    let mut context_payload: *mut u16 = core::ptr::null_mut();
    let mut buf1: *mut u16 = core::ptr::null_mut();
    let mut count: u32 = 0;
    let mut is_match: BOOL = 0;
    let mut item_tuple: [u64; 2] = [0; 2];

    let global_ns = GlobalPtrSppNamespace;
    let open_ns_fn = ((*(*global_ns).lpVtbl).open_namespace);

    let mut status = open_ns_fn(
        global_ns as *mut c_void,
        1,
        SERVER_PROPS_NAMESPACE,
        SERVER_PROPS_NAMESPACE,
        &mut p_collection,
    );

    if status >= 0 {
        let global_factory = GlobalPtrPluginFactory;
        let create_plugin_fn = ((*(*global_factory).lpVtbl).create_stock_plugin);

        status = create_plugin_fn(
            global_factory as *mut c_void,
            PLUGIN_PARAMS_PATH,
            &qword_1403E9940 as *const u64,
            0,
            &mut p_plugin_params,
        );

        if status >= 0 {
            status = SppGetAndDuplicateString(pszContextPrefix, &mut context_payload);

            if status >= 0 {
                let collection_vtbl = (*p_collection).lpVtbl;
                status = ((*collection_vtbl).get_count)(
                    p_collection as *mut c_void,
                    &mut count,
                );

                if status >= 0 {
                    let mut matched_any = false;
                    let mut index: u32 = 0;

                    if count == 0 {
                        status = SL_E_NOT_FOUND;
                    } else {
                        loop {
                            status = ((*collection_vtbl).get_item_at)(
                                p_collection as *mut c_void,
                                index,
                                &mut item_tuple,
                            );
                            if status < 0 {
                                break;
                            }

                            if !buf1.is_null() {
                                let heap = x4_getprocessheap();
                                x4_heapfree(heap, 0, buf1.offset(-2) as *mut c_void);
                                LogTraceEvent(0);
                                buf1 = core::ptr::null_mut();
                            }

                            let src_str = item_tuple[0] as PCWSTR;
                            status = SppGetAndDuplicateString(src_str, &mut buf1);
                            if status < 0 {
                                HandleSubsystemError(status);
                                break;
                            }

                            status = SppStringStartsWith(buf1, context_payload, &mut is_match);
                            if status < 0 {
                                break;
                            }

                            if is_match != 0 {
                                status = SppFlushOrResetServerProperties(src_str);
                                if status < 0 {
                                    break;
                                }
                                matched_any = true;
                            } else {
                                let params_vtbl = (*p_plugin_params).lpVtbl;
                                status = ((*params_vtbl).skip_or_verify_layout)(
                                    p_plugin_params as *mut c_void,
                                    &item_tuple,
                                    0,
                                );
                                if status < 0 {
                                    break;
                                }
                            }

                            index += 1;
                            if index >= count {
                                if !matched_any {
                                    status = SL_E_NOT_FOUND;
                                    break;
                                }

                                let delete_fn = ((*(*global_ns).lpVtbl).delete_property);
                                status = delete_fn(
                                    global_ns as *mut c_void,
                                    1,
                                    SERVER_PROPS_NAMESPACE,
                                    SERVER_PROPS_NAMESPACE,
                                );

                                if status >= 0 {
                                    let params_vtbl = (*p_plugin_params).lpVtbl;
                                    status = ((*params_vtbl).get_count)(
                                        p_plugin_params as *mut c_void,
                                        &mut count,
                                    );

                                    if status >= 0 && count > 0 {
                                        let mut commit_idx: u32 = 0;
                                        loop {
                                            status = ((*params_vtbl).get_item_at)(
                                                p_plugin_params as *mut c_void,
                                                commit_idx,
                                                &mut item_tuple,
                                            );
                                            if status < 0 {
                                                break;
                                            }

                                            let commit_fn = ((*(*global_ns).lpVtbl).commit_property);
                                            status = commit_fn(
                                                global_ns as *mut c_void,
                                                1,
                                                SERVER_PROPS_NAMESPACE,
                                                SERVER_PROPS_NAMESPACE,
                                                item_tuple[0] as PCWSTR,
                                            );
                                            if status < 0 {
                                                break;
                                            }

                                            commit_idx += 1;
                                            if commit_idx >= count {
                                                break;
                                            }
                                        }
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    if status < 0 {
        HandleSubsystemError(status);
    }

    LogTraceEvent(status);

    if !p_plugin_params.is_null() {
        let release_fn = (*(*p_plugin_params).lpVtbl).release;
        release_fn(p_plugin_params as *mut c_void);
    }

    if !p_collection.is_null() {
        let release_fn = (*(*p_collection).lpVtbl).release;
        release_fn(p_collection as *mut c_void);
    }

    if !buf1.is_null() {
        let heap = x4_getprocessheap();
        x4_heapfree(heap, 0, buf1.offset(-2) as *mut c_void);
        LogTraceEvent(0);
    }

    if !context_payload.is_null() {
        let heap = x4_getprocessheap();
        x4_heapfree(heap, 0, context_payload.offset(-2) as *mut c_void);
        LogTraceEvent(0);
    }

    status
}


pub unsafe fn SppValidateServerPropertiesToken(
    ppwszAppGuid: *mut *mut c_void,
    hSkuGuid: u64,
) -> HRESULT {
    let mut exists: i32 = 0;
    let mut stack_buf: [u8; 84] = [0; 84];
    let mut src_ptr: *mut c_void = core::ptr::null_mut();
    let mut status1: HRESULT = 0;
    let mut status2: HRESULT = 0;

    let global_ns = GlobalPtrSppNamespace;
    let property_exists_fn = ((*(global_ns).lpVtbl).PropertyExists);

    let mut status: HRESULT = property_exists_fn(
        global_ns,
        1,
        SERVER_PROPS_NAMESPACE,
        &mut exists,
    );

    if status >= 0 {
        if exists != 0 {
            let mut params1 = QueryContextParams {
                system_info_type: 3,
                target_ptr: core::ptr::null_mut(),
                global_ref: &qword_140048B40 as *const usize,
                sku_guid: hSkuGuid,
                out_status: status1,
                output_buf: stack_buf.as_mut_ptr(),
                app_guid_ptr: ppwszAppGuid,
                reserved1: 0,
                reserved2: 0,
                reserved3: 0,
            };

            x4_ntquerysysteminformation(
                0x85, // SystemExtendedProcessInformation | 0x80
                &mut params1 as *mut _ as *mut c_void,
                0x40,
                core::ptr::null_mut(),
            );

            status = status1;
            if status1 >= 0 {
                let mut params2 = QueryContextParams {
                    system_info_type: 3,
                    target_ptr: core::ptr::null_mut(),
                    global_ref: &qword_140048180 as *const usize,
                    sku_guid: 0,
                    out_status: status2,
                    output_buf: stack_buf.as_mut_ptr(),
                    app_guid_ptr: &mut src_ptr,
                    reserved1: 0,
                    reserved2: 0,
                    reserved3: 0,
                };

                x4_ntquerysysteminformation(
                    0x85,
                    &mut params2 as *mut _ as *mut c_void,
                    0x40,
                    core::ptr::null_mut(),
                );

                status = status2;
                if status2 >= 0 {
                    status = SppValidateServerProperties(src_ptr as PCWSTR);
                    if status < 0 {
                        HandleSubsystemError(status);
                    }
                } else {
                    HandleSubsystemError(status);
                }
            } else {
                HandleSubsystemError(status);
            }
        }
    } else {
        HandleSubsystemError(status);
    }

    LogTraceEvent(status);

    if !src_ptr.is_null() {
        let heap = x4_getprocessheap();
        // Allocation buffer offset adjustment matching C source (src_ptr - 4 bytes)
        x4_heapfree(heap, 0, (src_ptr as *mut u8).offset(-4) as *mut c_void);
        LogTraceEvent(0);
    }

    status
}