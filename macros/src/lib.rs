
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, parse_quote, ItemImpl, TraitItemFn};

#[proc_macro_attribute]
pub fn object(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemImpl);

    let mut items = Vec::new();
    let mut sender_items = Vec::new();
    for item in item.items {
        match item {
            syn::ImplItem::Fn(fn_item) => {
                let mut fn_item = fn_item.clone();
                if fn_item
                    .attrs
                    .iter()
                    .find(|attr| {
                        attr.meta
                            .path()
                            .get_ident()
                            .map(|ident| ident.to_string())
                            .as_deref()
                            == Some("slot")
                    })
                    .is_some()
                {
                    fn_item.attrs.clear();
                    items.push(fn_item.clone());

                    fn_item.sig.inputs[0] = parse_quote!(&self);
                    let fn_ident = &fn_item.sig.ident;
                    let inputs: Vec<_> = fn_item
                        .sig
                        .inputs
                        .iter()
                        .filter_map(|arg| match arg {
                            syn::FnArg::Receiver(_) => None,
                            syn::FnArg::Typed(ty) => Some(ty.pat.to_token_stream()),
                        })
                        .collect();
                    fn_item.block = parse_quote!({
                        self.handle.update(move |me| {
                            me.#fn_ident(#(#inputs)*)
                        });
                    });

                    sender_items.push(fn_item.clone());
                } else {
                    fn_item.attrs.clear();
                    items.push(fn_item.clone());
                }
            }
            syn::ImplItem::Verbatim(tokens) => {
                let item: TraitItemFn = syn::parse2(tokens).unwrap();
                let sig = item.sig;

                let input_pats = sig.inputs.iter().filter_map(|arg| match arg {
                    syn::FnArg::Receiver(_) => None,
                    syn::FnArg::Typed(pat_ty) => Some(&pat_ty.pat),
                });

                items.push(parse_quote! {
                    #[allow(unused_variables)]
                    pub #sig {
                        viewbuilder::Runtime::current().emit(Box::new((#(#input_pats),*)))
                    }
                });

                let input_tys = sig.inputs.iter().filter_map(|arg| match arg {
                    syn::FnArg::Receiver(_) => None,
                    syn::FnArg::Typed(pat_ty) => Some(&pat_ty.ty),
                });
                let ident = sig.ident;
                sender_items.push(parse_quote! {
                    pub fn #ident(&self) -> viewbuilder::Signal<(#(#input_tys),*)> {
                        viewbuilder::Signal::new(self.handle.key)
                    }
                });
            }
            _ => {}
        }
    }

    let ident = format_ident!("{}", item.self_ty.to_token_stream().to_string());
    let sender_ident = format_ident!("{}Sender", &ident);
    let output = quote! {
        impl Object for #ident {
            type Sender = #sender_ident;
        }

        impl #ident {
            #(#items)*
        }

        #[derive(Clone)]
        pub struct #sender_ident {
            handle: viewbuilder::HandleState<#ident>,
        }

        impl From<viewbuilder::HandleState<#ident>> for #sender_ident {
            fn from(value: viewbuilder::HandleState<#ident>) -> Self {
                Self { handle: value }
            }
        }


        impl #sender_ident {
            #(#sender_items)*
        }
    };
    output.into_token_stream().into()
}
