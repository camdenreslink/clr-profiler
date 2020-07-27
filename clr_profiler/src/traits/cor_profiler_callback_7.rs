#![allow(unused_variables)]
use crate::{errors::Error, ffi::ModuleID, CorProfilerCallback6};

pub trait CorProfilerCallback7: CorProfilerCallback6 {
    fn module_in_memory_symbols_updated(&mut self, module_id: ModuleID) -> Result<(), Error> {
        Ok(())
    }
}
