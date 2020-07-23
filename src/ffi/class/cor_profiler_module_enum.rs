#![allow(non_snake_case)]
use crate::ffi::{ICorProfilerModuleEnum, IUnknown};

#[repr(C)]
pub struct CorProfilerModuleEnumVtbl {
    pub IUnknown: IUnknown<CorProfilerModuleEnum>,
    pub ICorProfilerModuleEnum: ICorProfilerModuleEnum<CorProfilerModuleEnum>,
}

#[repr(C)]
pub struct CorProfilerModuleEnum {
    pub lpVtbl: *const CorProfilerModuleEnumVtbl,
}
