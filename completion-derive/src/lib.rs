use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// TODO: docs
#[proc_macro_derive(RuntimeSource)]
pub fn derive_runtime_source(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    TokenStream::from(quote! {
        #[no_mangle]
        pub extern "C" fn _nvim_completion_runtime_source() -> (
            ::nvim_completion_core::SourceId,
            *const dyn ::nvim_completion_core::ObjectSafeCompletionSource,
        ) {
            let name =
                <#struct_name as ::nvim_completion_core::CompletionSource>::NAME;

            let ptr = Box::into_raw(Box::new(#struct_name {})) as *const _;

            (name, ptr)
        }
    })
}
