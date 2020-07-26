#![allow(non_snake_case)]
use crate::ffi::{ICorProfilerMethodEnum, IUnknown, COR_PRF_METHOD, HRESULT, ULONG};

#[repr(C)]
pub struct CorProfilerMethodEnumVtbl {
    pub IUnknown: IUnknown<CorProfilerMethodEnum>,
    pub ICorProfilerMethodEnum: ICorProfilerMethodEnum<CorProfilerMethodEnum>,
}

#[repr(C)]
pub struct CorProfilerMethodEnum {
    pub lpVtbl: *const CorProfilerMethodEnumVtbl,
}

impl CorProfilerMethodEnum {
    unsafe fn i_cor_profiler_method_enum(&self) -> &ICorProfilerMethodEnum<Self> {
        &(*self.lpVtbl).ICorProfilerMethodEnum
    }
    unsafe fn Skip(&self, celt: ULONG) -> HRESULT {
        (self.i_cor_profiler_method_enum().Skip)(self, celt)
    }
    unsafe fn Reset(&self) -> HRESULT {
        (self.i_cor_profiler_method_enum().Reset)(self)
    }
    unsafe fn Clone(&self, ppEnum: *mut *mut Self) -> HRESULT {
        (self.i_cor_profiler_method_enum().Clone)(self, ppEnum)
    }
    unsafe fn GetCount(&self, pcelt: *mut ULONG) -> HRESULT {
        (self.i_cor_profiler_method_enum().GetCount)(self, pcelt)
    }
    unsafe fn Next(
        &self,
        celt: ULONG,
        elements: *mut COR_PRF_METHOD,
        pceltFetched: *mut ULONG,
    ) -> HRESULT {
        (self.i_cor_profiler_method_enum().Next)(self, celt, elements, pceltFetched)
    }
}
