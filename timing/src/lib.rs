extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;

//my first macro!!
//a
//attr is whatever you pass to the macro itself
//item is the attribute this macro is being called on
#[proc_macro_attribute]
pub fn measure_duration(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = input_fn.sig.ident;
    let fn_inputs = input_fn.sig.inputs;
    let fn_body = input_fn.block;
    let fn_return = input_fn.sig.output;
    let fn_name_str = format!("{}", fn_name);
    let tokens = quote! {
        fn #fn_name(#fn_inputs) #fn_return {
            let _start = Instant::now();
            let _in = ||
                #fn_body


            ;println!("Duration of {} : {:?}", #fn_name_str, _start.elapsed());

            _in()
        }
    };

    TokenStream::from(tokens)
}
