use crate::{errors::Error, ffi::FunctionID, CorProfilerCallback8};

pub trait CorProfilerCallback9: CorProfilerCallback8 {
    fn dynamic_method_unloaded(&mut self, function_id: FunctionID) -> Result<(), Error> {
        Ok(())
    }
}
