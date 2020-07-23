#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerInfo6<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerInfo6<()> {
    // F30A070D-BFFB-46A7-B1D8-8781EF7B698A
    pub const IID: GUID = GUID {
        data1: 0xF30A070D,
        data2: 0xBFFB,
        data3: 0x46A7,
        data4: [0xB1, 0xD8, 0x87, 0x81, 0xEF, 0x7B, 0x69, 0x8A],
    };
}
