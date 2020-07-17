#![allow(non_snake_case)]
use crate::ffi::GUID;

#[repr(C)]
pub struct ICorProfilerInfo3<T> {
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerInfo3<()> {
    // B555ED4F-452A-4E54-8B39-B5360BAD32A0
    pub const iid: GUID = GUID {
        data1: 0xB555ED4F,
        data2: 0x452A,
        data3: 0x4E54,
        data4: [0x8B, 0x39, 0xB5, 0x36, 0x0B, 0xAD, 0x32, 0xA0],
    };
}
