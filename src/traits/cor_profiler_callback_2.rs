use crate::{
    errors::Error,
    ffi::{
        int, GCHandleID, ObjectID, ThreadID, BOOL, COR_PRF_GC_REASON, COR_PRF_GC_ROOT_FLAGS,
        COR_PRF_GC_ROOT_KIND, DWORD, UINT_PTR, ULONG, WCHAR,
    },
    CorProfilerCallback,
};

pub trait CorProfilerCallback2: CorProfilerCallback {
    fn thread_name_changed(
        &mut self,
        thread_id: ThreadID,
        cch_name: ULONG,
        name: &[WCHAR],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn garbage_collection_started(
        &mut self,
        c_generations: int,
        generation_collected: &[BOOL],
        reason: COR_PRF_GC_REASON,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn surviving_references(
        &mut self,
        c_surviving_object_id_ranges: ULONG,
        object_id_range_start: &[ObjectID],
        c_object_id_range_length: &[ULONG],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn finalizeable_object_queued(
        &mut self,
        finalizer_flags: DWORD,
        object_id: ObjectID,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn root_references_2(
        &mut self,
        c_root_refs: ULONG,
        root_ref_ids: &[ObjectID],
        root_kinds: &[COR_PRF_GC_ROOT_KIND],
        root_flags: &[COR_PRF_GC_ROOT_FLAGS],
        root_ids: &[UINT_PTR],
    ) -> Result<(), Error> {
        Ok(())
    }

    fn handle_created(
        &mut self,
        handle_id: GCHandleID,
        initial_object_id: ObjectID,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn handle_destroyed(&mut self, handle_id: GCHandleID) -> Result<(), Error> {
        Ok(())
    }
}
