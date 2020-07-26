#!/bin/sh

cargo build --features "basic_integration_test" --manifest-path test_profilers/Cargo.toml
dotnet build test_clr/HelloWorld/HelloWorld.csproj

export CORECLR_ENABLE_PROFILING=1
export CORECLR_PROFILER={DF63A541-5A33-4611-8829-F4E495985EE3}
export CORECLR_PROFILER_PATH=target/debug/libtest_profilers.so

dotnet test_clr/HelloWorld/bin/Debug/netcoreapp3.0/HelloWorld.dll