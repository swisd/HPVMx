#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct _GUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
}



pub type KUID = _GUID;

impl KUID {
    pub fn none() -> KUID {
        KUID {
            Data1: 0,
            Data2: 0,
            Data3: 0,
            Data4: [0; 8],
        }
    }

    pub fn new() -> KUID {
        KUID {
            Data1: 0,
            Data2: 0,
            Data3: 0,
            Data4: [0; 8],
        }
    }
}