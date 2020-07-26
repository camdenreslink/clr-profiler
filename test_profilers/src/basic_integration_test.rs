use clr_profiler::{
    register, CorProfilerCallback, CorProfilerCallback2, CorProfilerCallback3,
    CorProfilerCallback4, CorProfilerCallback5, CorProfilerCallback6, CorProfilerCallback7,
    CorProfilerCallback8, CorProfilerCallback9,
};

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
struct Profiler {}
impl Profiler {
    pub fn new() -> Profiler {
        Profiler {}
    }
}

register!(Profiler);
