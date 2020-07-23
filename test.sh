#!/bin/sh

cargo build
dotnet build samples/HelloWorld/HelloWorld.csproj

export CORECLR_ENABLE_PROFILING=1
export CORECLR_PROFILER={DF63A541-5A33-4611-8829-F4E495985EE3}
export CORECLR_PROFILER_PATH=target/debug/libclr_profiler.so

dotnet samples/HelloWorld/bin/Debug/netcoreapp2.2/HelloWorld.dll