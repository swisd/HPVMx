use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::globals::{ERROR_HANDLE_EOF, E_INVALIDARG, INTSAFE_E_ARITHMETIC_OVERFLOW};
use crate::x4::spp::string::{SppDuplicateStringLocal, SppGetStringCharacterCount};
use crate::x4::types::{SppStringTokenizer, HRESULT, PCWSTR, PWSTR};

pub unsafe fn SppTokenizerGetTokenString(
    this: *mut SppStringTokenizer,
    ppwszOutToken: *mut PCWSTR,
) -> HRESULT {
    let mut status: HRESULT = 0;
    let mut out_token: PWSTR = core::ptr::null_mut();

    if this.is_null() || ppwszOutToken.is_null() {
        return -2147024809; // E_INVALIDARG
    }

    let tokenizer = &mut *this;
    let mut current_ptr = tokenizer.pCurrentTokenStart;

    if !current_ptr.is_null() {
        if *current_ptr != 0 {
            while current_ptr < tokenizer.pCurrentTokenEnd {
                let next_ptr = current_ptr.add(1);

                if *current_ptr == tokenizer.wchDelimiter {
                    // Replace delimiter with null-terminator and advance
                    *current_ptr = 0;
                    current_ptr = current_ptr.add(1);

                    if next_ptr < tokenizer.pCurrentTokenEnd {
                        break;
                    }

                    status = ERROR_HANDLE_EOF;
                    HandleSubsystemError(ERROR_HANDLE_EOF);
                    LogTraceEvent(status);
                    return status;
                }

                current_ptr = current_ptr.add(1);

                if *next_ptr == 0 {
                    break;
                }
            }
        }

        out_token = tokenizer.pCurrentTokenStart;

        // Advance start pointer if not reaching end pointer boundary
        if current_ptr < tokenizer.pCurrentTokenEnd {
            tokenizer.pCurrentTokenStart = current_ptr;
        } else {
            tokenizer.pCurrentTokenStart = core::ptr::null_mut();
        }
    }

    *ppwszOutToken = out_token as PCWSTR;

    LogTraceEvent(status);
    status
}

pub unsafe fn SppTokenizerInitialize(
    this: *mut SppStringTokenizer,
    pwszSourceString: PCWSTR,
    wchDelimiter: u16,
) -> HRESULT {
    let mut status: HRESULT;

    if this.is_null() || pwszSourceString.is_null() {
        status = E_INVALIDARG;
        HandleSubsystemError(status);
        LogTraceEvent(status);
        return status;
    }

    let tokenizer = &mut *this;

    if *pwszSourceString != 0 {
        tokenizer.wchDelimiter = wchDelimiter;

        status = SppDuplicateStringLocal(pwszSourceString, &mut tokenizer.pSourceStringBase);
        if status < 0 {
            HandleSubsystemError(status);
            LogTraceEvent(status);
            return status;
        }

        status = SppGetStringCharacterCount(tokenizer.pSourceStringBase, &mut tokenizer.cchTotalCharacters);
        if status < 0 {
            HandleSubsystemError(status);
            LogTraceEvent(status);
            return status;
        }

        let base_ptr = tokenizer.pSourceStringBase;
        let end_ptr = base_ptr.add(tokenizer.cchTotalCharacters as usize);
        tokenizer.pCurrentTokenEnd = end_ptr;

        if base_ptr < end_ptr {
            tokenizer.pCurrentTokenStart = base_ptr;
            LogTraceEvent(status);
            return status;
        }

        status = INTSAFE_E_ARITHMETIC_OVERFLOW;
    } else {
        status = E_INVALIDARG;
    }

    HandleSubsystemError(status);
    LogTraceEvent(status);

    status
}