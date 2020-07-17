#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerCallback4<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerCallback4<()> {
    // 7B63B2E3-107D-4D48-B2F6-F61E229470D2
    pub const iid: GUID = GUID {
        data1: 0x7B63B2E3,
        data2: 0x107D,
        data3: 0x4D48,
        data4: [0xB2, 0xF6, 0xF6, 0x1E, 0x22, 0x94, 0x70, 0xD2],
    };
}
