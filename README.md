# Brainstorming

## How to create the Profiler COM object in Rust
- Must generate a .so file
  - Specify crate-type=cdylib
  - https://doc.rust-lang.org/reference/linkage.html
- The only symbol that needs exposed is `DllGetClassObject`
  - An example implementation in rust: https://github.com/Rantanen/intercom/blob/master/intercom-common/src/attributes/com_library.rs#L87
  - Rust function def if we were calling it in Rust from C (we're actually doing the opposite): https://docs.rs/winapi/0.3.8/winapi/um/combaseapi/fn.DllGetClassObject.html
- Must have a class that implements ICorProfilerCallback2 (or higher).
  - What does it mean to have a class if we only have C bindings (no C++)
  - What does it mean to implement an interface? COM has a concept of interfaces, but C/C++ doesn't (even though it's in the .h file as an "interface")
- 

## Basic flow is:
1. Some COM client (CLR in this case) calls `DllGetClassObject`, which populates a pointer ([out] parameter) to an instance of a struct that adheres to IClassFactory
2. The COM client then calls `CreateInstance` on the IClassFactory that it now has a handle to. This populates a pointer ([out] parameter) to an instance of a struct that adheres to ICorProfilerCallback9.
3. Now the COM client can call function pointers in this struct that it know will exist. Neat!

Some Helpful Resources:
- Docs on some of the weird Windows specific types (LPVOID, DWORD, HINSTANCE, etc...)
  - https://en.wikibooks.org/wiki/Windows_Programming/Handles_and_Data_Types