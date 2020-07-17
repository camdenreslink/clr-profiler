#![allow(non_snake_case)]
use super::{IUnknown, E_NOINTERFACE, GUID, HRESULT, LPVOID, REFIID, S_OK, ULONG};
use std::{
    ffi::c_void,
    ptr,
    sync::atomic::{AtomicU32, Ordering},
};
// use i_cor_profiler_callback::ICorProfilerCallback;
// use i_cor_profiler_callback_2::ICorProfilerCallback2;
// use i_cor_profiler_callback_3::ICorProfilerCallback3;
// use i_cor_profiler_callback_4::ICorProfilerCallback4;
// use i_cor_profiler_callback_5::ICorProfilerCallback5;
// use i_cor_profiler_callback_6::ICorProfilerCallback6;
// use i_cor_profiler_callback_7::ICorProfilerCallback7;
// use i_cor_profiler_callback_8::ICorProfilerCallback8;
// use i_cor_profiler_callback_9::ICorProfilerCallback9;

// mod i_cor_profiler_callback;
// mod i_cor_profiler_callback_2;
// mod i_cor_profiler_callback_3;
// mod i_cor_profiler_callback_4;
// mod i_cor_profiler_callback_5;
// mod i_cor_profiler_callback_6;
// mod i_cor_profiler_callback_7;
// mod i_cor_profiler_callback_8;
// mod i_cor_profiler_callback_9;

#[repr(C)]
pub struct CorProfilerCallbackVtbl {
    pub IUnknown: IUnknown<CorProfilerCallback>,
    // pub ICorProfilerCallback: ICorProfilerCallback<CorProfilerCallback>,
    // pub ICorProfilerCallback2: ICorProfilerCallback2<CorProfilerCallback>,
    // pub ICorProfilerCallback3: ICorProfilerCallback3<CorProfilerCallback>,
    // pub ICorProfilerCallback4: ICorProfilerCallback4<CorProfilerCallback>,
    // pub ICorProfilerCallback5: ICorProfilerCallback5<CorProfilerCallback>,
    // pub ICorProfilerCallback6: ICorProfilerCallback6<CorProfilerCallback>,
    // pub ICorProfilerCallback7: ICorProfilerCallback7<CorProfilerCallback>,
    // pub ICorProfilerCallback8: ICorProfilerCallback8<CorProfilerCallback>,
    // pub ICorProfilerCallback9: ICorProfilerCallback9<CorProfilerCallback>,
}

#[repr(C)]
pub struct CorProfilerCallback {
    pub lpVtbl: *const CorProfilerCallbackVtbl,
    ref_count: AtomicU32,
}

impl CorProfilerCallback {
    pub fn new<'a>() -> &'a mut CorProfilerCallback {
        let profiler = CorProfilerCallback {
            lpVtbl: &CorProfilerCallbackVtbl {
                IUnknown: IUnknown {
                    QueryInterface: Self::query_interface,
                    AddRef: Self::add_ref,
                    Release: Self::release,
                },
            },
            ref_count: AtomicU32::new(0),
        };
        Box::leak(Box::new(profiler))
    }

    // DF63A541-5A33-4611-8829-F4E495985EE3
    pub const clsid: GUID = GUID {
        data1: 0xDF63A541,
        data2: 0x5A33,
        data3: 0x4611,
        data4: [0x88, 0x29, 0xF4, 0xE4, 0x95, 0x98, 0x5E, 0xE3],
    };

    pub unsafe extern "system" fn query_interface(
        &mut self,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT {
        println!(
            "CorProfilerCallback hit query_interface! Querying riid: {:?}",
            *riid
        );
        if *riid == IUnknown::iid
        // || *riid == ICorProfilerCallback::iid
        // || *riid == ICorProfilerCallback2::iid
        // || *riid == ICorProfilerCallback3::iid
        // || *riid == ICorProfilerCallback4::iid
        // || *riid == ICorProfilerCallback5::iid
        // || *riid == ICorProfilerCallback6::iid
        // || *riid == ICorProfilerCallback7::iid
        // || *riid == ICorProfilerCallback8::iid
        // || *riid == ICorProfilerCallback9::iid
        {
            *ppvObject = self as *mut CorProfilerCallback as LPVOID;
            self.add_ref();
            S_OK
        } else {
            *ppvObject = ptr::null_mut();
            E_NOINTERFACE
        }
    }

    pub unsafe extern "system" fn add_ref(&mut self) -> ULONG {
        println!("CorProfilerCallback hit add_ref!");
        // TODO: Which ordering is appropriate?
        let prev_ref_count = self.ref_count.fetch_add(1, Ordering::Relaxed);
        prev_ref_count + 1
    }

    pub unsafe extern "system" fn release(&mut self) -> ULONG {
        println!("CorProfilerCallback hit release!");
        // Ensure we are not trying to release the memory twice if
        // client calls release despite the ref_count being zero.
        // TODO: Which ordering is appropriate?
        if self.ref_count.load(Ordering::Relaxed) == 0 {
            panic!("Cannot release the COM object, it has already been released.");
        }

        let prev_ref_count = self.ref_count.fetch_sub(1, Ordering::Relaxed);
        let ref_count = prev_ref_count - 1;

        if ref_count == 0 {
            drop(Box::from_raw(self as *mut CorProfilerCallback));
        }

        ref_count
    }
}
