#![allow(unused_variables)]
use crate::{errors::Error, ffi::CorProfilerAssemblyReferenceProvider, CorProfilerCallback5};

pub trait CorProfilerCallback6: CorProfilerCallback5 {
    fn get_assembly_references(
        &mut self,
        wsz_assembly_path: &str,
        p_asm_ref_provider: &CorProfilerAssemblyReferenceProvider,
    ) -> Result<(), Error> {
        Ok(())
    }
}
