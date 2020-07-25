mod errors;
mod ffi;
mod traits;

pub use clr_profiler_macros::*;
pub use traits::{
    CorProfilerCallback, CorProfilerCallback2, CorProfilerCallback3, CorProfilerCallback4,
    CorProfilerCallback5, CorProfilerCallback6, CorProfilerCallback7, CorProfilerCallback8,
    CorProfilerCallback9,
};
extern crate self as clr_profiler;

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

register!(Profiler); // TODO: Remove this when testing mechanism is set up. Needs to be called by clients implementing their own profilers.
