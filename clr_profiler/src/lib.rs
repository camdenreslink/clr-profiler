mod errors;
pub mod ffi;
mod traits;

pub use clr_profiler_macros::*;
pub use errors::*;
pub use traits::{
    CorProfilerCallback, CorProfilerCallback2, CorProfilerCallback3, CorProfilerCallback4,
    CorProfilerCallback5, CorProfilerCallback6, CorProfilerCallback7, CorProfilerCallback8,
    CorProfilerCallback9,
};
