use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::globals::{E_INVALIDARG, INTSAFE_E_ARITHMETIC_OVERFLOW, S_OK};
use crate::x4::types::HRESULT;

pub unsafe fn SppSafeIntAdd(
    value1: i32,
    value2: i32,
    pOutResult: *mut i32,
) -> HRESULT {
    let mut v3 = 0;
    let mut v5;

    if value1 < 0 || value2 < 0 {
        v5 = E_INVALIDARG;
        HandleSubsystemError(v5);
    } else {
        // Unsigned addition check for overflow detection matching standard safeint pattern
        let sum_u = (value1 as u32).wrapping_add(value2 as u32);

        if sum_u < (value1 as u32) {
            v5 = INTSAFE_E_ARITHMETIC_OVERFLOW;
            HandleSubsystemError(INTSAFE_E_ARITHMETIC_OVERFLOW);
        } else {
            v3 = sum_u as i32;
            v5 = S_OK;
        }

        LogTraceEvent(v5);

        if v5 >= 0 {
            if v3 < 0 {
                v5 = INTSAFE_E_ARITHMETIC_OVERFLOW;
                HandleSubsystemError(v5);
            } else {
                if !pOutResult.is_null() {
                    *pOutResult = v3;
                }
            }
        }
    }

    LogTraceEvent(v5);
    v5
}