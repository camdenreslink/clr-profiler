extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

#[proc_macro]
pub fn register(item: TokenStream) -> TokenStream {
    let profiler_type = parse_macro_input!(item as Type);
    let output = quote! {
        use clr_profiler::ffi::{ClassFactory, CorProfilerCallback as FFICorProfilerCallback, E_FAIL, HRESULT, LPVOID, REFCLSID, REFIID};
        #[no_mangle]
        unsafe extern "system" fn DllGetClassObject(
            rclsid: REFCLSID,
            riid: REFIID,
            ppv: *mut LPVOID,
        ) -> HRESULT {
            if ppv.is_null() || *rclsid != FFICorProfilerCallback::<#profiler_type>::CLSID {
                println!("rclsid isn't correct.");
                E_FAIL
            } else {
                //let profiler = Box::new(<#profiler_type>::new());
                let profiler = <#profiler_type>::new();
                let class_factory: &mut ClassFactory<#profiler_type> = ClassFactory::new(profiler);
                class_factory.QueryInterface(riid, ppv)
            }
        }
    };
    output.into()
}
#[proc_macro_derive(CorProfilerCallback)]
pub fn derive_cor_profiler_callback(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback2)]
pub fn derive_cor_profiler_callback_2(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback2 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback3)]
pub fn derive_cor_profiler_callback_3(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback3 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback4)]
pub fn derive_cor_profiler_callback_4(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback4 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback5)]
pub fn derive_cor_profiler_callback_5(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback5 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback6)]
pub fn derive_cor_profiler_callback_6(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback6 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback7)]
pub fn derive_cor_profiler_callback_7(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback7 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback8)]
pub fn derive_cor_profiler_callback_8(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback8 for #name {}
    };
    trait_impl.into()
}
#[proc_macro_derive(CorProfilerCallback9)]
pub fn derive_cor_profiler_callback_9(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let trait_impl = quote! {
        impl clr_profiler::CorProfilerCallback9 for #name {}
    };
    trait_impl.into()
}