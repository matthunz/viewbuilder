use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let stmts = input.block.stmts;
    let expanded = quote! {
        fn main() {
            let viewbuilder_macros_ui = viewbuilder::native::UserInterface::default();
            let _guard = viewbuilder_macros_ui.enter();

            #(#stmts)*

            viewbuilder_macros_ui.run()
        }
    };
    expanded.into()
}
