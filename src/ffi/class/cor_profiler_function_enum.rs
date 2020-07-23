#![allow(non_snake_case)]
use crate::ffi::{ICorProfilerFunctionEnum, IUnknown};

#[repr(C)]
pub struct CorProfilerFunctionEnumVtbl {
    pub IUnknown: IUnknown<CorProfilerFunctionEnum>,
    pub ICorProfilerFunctionEnum: ICorProfilerFunctionEnum<CorProfilerFunctionEnum>,
}

#[repr(C)]
pub struct CorProfilerFunctionEnum {
    pub lpVtbl: *const CorProfilerFunctionEnumVtbl,
}
