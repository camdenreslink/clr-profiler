use clr_profiler::{
    register, ClrProfiler, CorProfilerCallback, CorProfilerCallback2, CorProfilerCallback3,
    CorProfilerCallback4, CorProfilerCallback5, CorProfilerCallback6, CorProfilerCallback7,
    CorProfilerCallback8, CorProfilerCallback9,
};
use uuid::Uuid;

#[derive(
    Clone,
    CorProfilerCallback,
    CorProfilerCallback2,
    CorProfilerCallback3,
    CorProfilerCallback4,
    CorProfilerCallback5,
    CorProfilerCallback6,
    CorProfilerCallback7,
    CorProfilerCallback8,
    CorProfilerCallback9,
)]
struct Profiler {
    clsid: Uuid,
}

impl ClrProfiler for Profiler {
    fn new() -> Profiler {
        Profiler {
            clsid: Uuid::parse_str("DF63A541-5A33-4611-8829-F4E495985EE3").unwrap(),
        }
    }
    fn clsid(&self) -> &Uuid {
        &self.clsid
    }
}

register!(Profiler);
