#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerInfo2<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerInfo2<()> {
    // CC0935CD-A518-487d-B0BB-A93214E65478
    pub const iid: GUID = GUID {
        data1: 0xCC0935CD,
        data2: 0xA518,
        data3: 0x487d,
        data4: [0xB0, 0xBB, 0xA9, 0x32, 0x14, 0xE6, 0x54, 0x78],
    };
}
