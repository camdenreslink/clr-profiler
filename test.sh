#!/bin/sh
export CORECLR_ENABLE_PROFILING=1
export CORECLR_PROFILER={DF63A541-5A33-4611-8829-F4E495985EE3}
export CORECLR_PROFILER_PATH=target/debug/libclr_profiler.so

cargo build
dotnet run --project samples/HelloWorld/HelloWorld.csproj