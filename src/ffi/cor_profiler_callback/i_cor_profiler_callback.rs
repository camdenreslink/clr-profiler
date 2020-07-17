#![allow(non_snake_case)]
use crate::ffi::{CorProfilerInfo, GUID, HRESULT};

#[repr(C)]
pub struct ICorProfilerCallback<T> {
    pub Initialize: fn(this: &mut T, pICorProfilerInfoUnk: *const CorProfilerInfo) -> HRESULT,
    // TODO: fill in FFI interface functions here
}

impl ICorProfilerCallback<()> {
    // 176FBED1-A55C-4796-98CA-A9DA0EF883E7
    pub const iid: GUID = GUID {
        data1: 0x176FBED1,
        data2: 0xA55C,
        data3: 0x4796,
        data4: [0x98, 0xCA, 0xA9, 0xDA, 0x0E, 0xF8, 0x83, 0xE7],
    };
}
