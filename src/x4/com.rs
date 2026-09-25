
// #[repr(C)]
// pub struct IUnknownVtbl {
//     pub QueryInterface: unsafe extern "system" fn(
//         this: *mut IUnknown,
//         riid: *const GUID,
//         ppv_object: *mut *mut c_void,
//     ) -> HRESULT,
//     pub AddRef: unsafe extern "system" fn(this: *mut IUnknown) -> ULONG,
//     pub Release: unsafe extern "system" fn(this: *mut IUnknown) -> ULONG,
// }
//
// #[repr(C)]
// pub struct IUnknown {
//     pub lp_vtbl: *const IUnknownVtbl,
// }

use core::ffi::c_void;
use crate::x4::types::{IUnknown, ULONG};

struct UnknownObject // sizeof=0x68
{
    vtable: *mut c_void,
    unknown_pad: *mut c_void,
    resource_ptr_2: *mut IUnknown,
    resource_ptr_3: *mut IUnknown,
    resource_ptr_4: *mut IUnknown,
    ref_field_5: *mut IUnknown,
    ref_field_6: *mut IUnknown,
    ref_field_7: *mut IUnknown,
    ref_field_8: *mut IUnknown,
    ref_field_9: *mut IUnknown,
    ref_field_10: *mut IUnknown,
    ref_field_11: *mut IUnknown,
    ref_field_12: *mut IUnknown,
}


impl UnknownObject {
    pub unsafe fn teardown(&mut self) -> u32 {
        // Helper closure to safely release a raw IUnknown pointer
        let mut release_ptr = |ptr_field: &mut *mut IUnknown| -> u32 {
            let ptr = core::mem::replace(ptr_field, core::ptr::null_mut());
            if !ptr.is_null() {
                // Cast to a basic COM interface pointer and call Release through vtbl
                // vtbl is at offset 0 of the COM object: *mut *mut Vtbl
                type ReleaseFn = unsafe extern "system" fn(*mut IUnknown) -> ULONG;
                let vtable = *(ptr as *const *const ReleaseFn);
                let release_func = *vtable.add(2); // Release is the 3rd entry (index 2)
                release_func(ptr)
            } else {
                0
            }
        };

        // Release reference fields in sequence
        release_ptr(&mut self.ref_field_12);
        release_ptr(&mut self.ref_field_11);
        release_ptr(&mut self.ref_field_10);
        release_ptr(&mut self.ref_field_9);
        release_ptr(&mut self.ref_field_8);
        release_ptr(&mut self.ref_field_7);
        release_ptr(&mut self.ref_field_6);
        let result = release_ptr(&mut self.ref_field_5);

        // Release resource pointers
        release_ptr(&mut self.resource_ptr_4);
        release_ptr(&mut self.resource_ptr_3);
        release_ptr(&mut self.resource_ptr_2);

        result
    }
}