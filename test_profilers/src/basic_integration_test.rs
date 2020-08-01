use clr_profiler::{
    ffi::{AssemblyID, ModuleID, COR_PRF_MONITOR, HRESULT},
    register, ClrProfiler, CorProfilerCallback, CorProfilerCallback2, CorProfilerCallback3,
    CorProfilerCallback4, CorProfilerCallback5, CorProfilerCallback6, CorProfilerCallback7,
    CorProfilerCallback8, CorProfilerCallback9, CorProfilerInfo, ProfilerInfo,
};
use uuid::Uuid;

#[derive(Clone)]
struct Profiler {
    clsid: Uuid,
    profiler_info: Option<ProfilerInfo>,
}
impl Profiler {
    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}
impl ClrProfiler for Profiler {
    fn new() -> Profiler {
        Profiler {
            clsid: Uuid::parse_str("DF63A541-5A33-4611-8829-F4E495985EE3").unwrap(),
            profiler_info: None,
        }
    }
    fn clsid(&self) -> &Uuid {
        &self.clsid
    }
}
impl CorProfilerCallback for Profiler {
    fn initialize(&mut self, profiler_info: ProfilerInfo) -> Result<(), HRESULT> {
        // Initialize ICorProfilerInfo reference
        self.profiler_info = Some(profiler_info);

        // Set the event mask
        self.profiler_info()
            .set_event_mask(COR_PRF_MONITOR::COR_PRF_ALL)?;

        Ok(())
    }
    fn module_attached_to_assembly(
        &mut self,
        module_id: ModuleID,
        assembly_id: AssemblyID,
    ) -> Result<(), HRESULT> {
        let module_info = self.profiler_info().get_module_info(module_id)?;
        println!("{:?}", module_info);
        Ok(())
    }
}
impl CorProfilerCallback2 for Profiler {}
impl CorProfilerCallback3 for Profiler {}
impl CorProfilerCallback4 for Profiler {}
impl CorProfilerCallback5 for Profiler {}
impl CorProfilerCallback6 for Profiler {}
impl CorProfilerCallback7 for Profiler {}
impl CorProfilerCallback8 for Profiler {}
impl CorProfilerCallback9 for Profiler {}

register!(Profiler);
