extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ItemFn,
    ReturnType,
    Type,
    parse_macro_input,
    parse_str,
};

#[proc_macro_attribute]
pub fn pipefail(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);
    let ret_ty: Type = match ::std::mem::replace(
        &mut func.sig.output,
        parse_str("-> std::io::Result<()>").unwrap(),
    ) {
        ReturnType::Default => parse_str("()").unwrap(),
        ReturnType::Type(_, ty) => *ty,
    };
    let (attrs, vis, sig, block) = (func.attrs, func.vis, func.sig, func.block);
    let out = quote! {
        #( #attrs )*
        #vis #sig {
            let res: #ret_ty = (|| { #block })();
            match res {
                ::std::result::Result::Err(e) => match e.kind() {
                    ::std::io::ErrorKind::BrokenPipe => Ok(()),
                    _ => ::std::result::Result::Err(e),
                },
                ::std::result::Result::Ok(_) => Ok(()),
            }
        }
    };
    out.into()
}
