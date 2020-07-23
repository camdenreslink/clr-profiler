#![allow(non_snake_case)]
use crate::ffi::{ICorProfilerFunctionControl, IUnknown};

#[repr(C)]
pub struct CorProfilerFunctionControlVtbl {
    pub IUnknown: IUnknown<CorProfilerFunctionControl>,
    pub ICorProfilerFunctionControl: ICorProfilerFunctionControl<CorProfilerFunctionControl>,
}

#[repr(C)]
pub struct CorProfilerFunctionControl {
    pub lpVtbl: *const CorProfilerFunctionControlVtbl,
}
