#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerCallback8<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerCallback8<()> {
    // 5BED9B15-C079-4D47-BFE2-215A140C07E0
    pub const iid: GUID = GUID {
        data1: 0x5BED9B15,
        data2: 0xC079,
        data3: 0x4D47,
        data4: [0xBF, 0xE2, 0x21, 0x5A, 0x14, 0x0C, 0x07, 0xE0],
    };
}
