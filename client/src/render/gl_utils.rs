use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn { attrs, vis, sig, block }
        = parse_macro_input!(item as ItemFn);

    let fn_name = sig.ident.to_string();
    let start = quote!(__measure_time_start_instant);

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            let #start = ::std::time::Instant::now();
            let ret = #block;
            ::std::println!("{} took: {:#?}", #fn_name, ::std::time::Instant::now().duration_since(#start));
            ret
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn gl_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn { attrs, vis, sig, block }
        = parse_macro_input!(item as ItemFn);

    let fn_name = sig.ident.to_string();

    let expanded = quote! {
        #[track_caller]
        #(#attrs)*
        #vis #sig {
            unsafe{
                let func = ||{#block};
                let ret = func();

                let error = CheckError(gl_context); 
                match error{
                    Some(e) =>{
                        ::std::println!("[GL_ERROR] {}:{}   {}", ::core::panic::Location::caller() ,#fn_name,e);
                    },
                    None =>{}
                }
                ret
            }
        }
    };

    TokenStream::from(expanded)
}
