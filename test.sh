#!/bin/sh
export CORECLR_ENABLE_PROFILING=1
export CORECLR_PROFILER={846F5F1C-F9AE-4B07-969E-05C26BC060D8}
export CORECLR_PROFILER_PATH=/home/creslink/git/camdenreslink/clr-profiler/target/debug/libclr_profiler.so

cargo build
dotnet run --project /home/creslink/git/camdenreslink/clr-profiler/samples/HelloWorld/HelloWorld.csproj