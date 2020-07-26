use crate::{
    errors::Error,
    ffi::{GCHandleID, ObjectID, ULONG},
    CorProfilerCallback4,
};

pub trait CorProfilerCallback5: CorProfilerCallback4 {
    fn conditional_weak_table_element_references(
        &mut self,
        c_root_refs: ULONG,
        key_ref_ids: &[ObjectID],
        value_ref_ids: &[ObjectID],
        root_ids: &[GCHandleID],
    ) -> Result<(), Error> {
        Ok(())
    }
}
