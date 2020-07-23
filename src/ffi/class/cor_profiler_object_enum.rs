#![allow(non_snake_case)]
use crate::ffi::{ICorProfilerObjectEnum, IUnknown};

#[repr(C)]
pub struct CorProfilerObjectEnumVtbl {
    pub IUnknown: IUnknown<CorProfilerObjectEnum>,
    pub ICorProfilerObjectEnum: ICorProfilerObjectEnum<CorProfilerObjectEnum>,
}

#[repr(C)]
pub struct CorProfilerObjectEnum {
    pub lpVtbl: *const CorProfilerObjectEnumVtbl,
}
