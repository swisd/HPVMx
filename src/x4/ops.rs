use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn unrolled_find_u16s(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();
    let mut start = haystack;

    // For performance reasons unfold the loop eight times max.
    while start.len() >= 8 {
        macro_rules! if_return {
            ($($n:literal,)+) => {
                $(
                    if start[$n] == needle {
                        return Some(((&start[$n] as *const u16).addr() - ptr.addr()) / 2);
                    }
                )+
            }
        }

        if_return!(0, 1, 2, 3, 4, 5, 6, 7,);

        start = &start[8..];
    }

    for c in start {
        if *c == needle {
            return Some(((c as *const u16).addr() - ptr.addr()) / 2);
        }
    }
    None
}

pub fn encode_wide(s: String) -> Vec<u16> {
    todo!()
}

pub fn to_u16s(s: String) -> Result<Vec<u16>, String> {
    fn inner(s: String) -> Result<Vec<u16>, String> {
        let mut maybe_result = Vec::with_capacity(s.len() + 1);
        maybe_result.extend(encode_wide(s));

        if unrolled_find_u16s(0, &maybe_result).is_some() {
            return Err("strings passed cannot contain NULs".parse().unwrap());
        }
        maybe_result.push(0);
        Ok(maybe_result)
    }
    inner(s)
}

pub fn truncate_utf16_at_nul(v: &[u16]) -> &[u16] {
    match unrolled_find_u16s(0, v) {
        // don't include the 0
        Some(i) => &v[..i],
        None => v,
    }
}

pub fn ensure_no_nuls(s: String) -> Result<String, String> {
    if encode_wide(s.clone()).iter().any(|b| *b == 0) {
        Err("nul byte found in provided data".parse().unwrap())
    } else {
        Ok(s)
    }
}

