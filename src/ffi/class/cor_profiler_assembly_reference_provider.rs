#![allow(non_snake_case)]
use crate::ffi::{ICorProfilerAssemblyReferenceProvider, IUnknown};

#[repr(C)]
pub struct CorProfilerAssemblyReferenceProviderVtbl {
    pub IUnknown: IUnknown<CorProfilerAssemblyReferenceProvider>,
    pub ICorProfilerAssemblyReferenceProvider:
        ICorProfilerAssemblyReferenceProvider<CorProfilerAssemblyReferenceProvider>,
}

#[repr(C)]
pub struct CorProfilerAssemblyReferenceProvider {
    pub lpVtbl: *const CorProfilerAssemblyReferenceProviderVtbl,
}
