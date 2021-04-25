/*! Filtering attributes for the `calm_io` project.

Procedural macros are required to be kept in a separate crate, as `rustc` builds
them alone and then executes their code during transformation of crates that use
them. This crate defines the `#[calm_io::FILTER]` attributes used to suppress
certain I/O errors from signaling failure.
!*/

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
	parse_macro_input,
	parse_str,
	ItemFn,
	ReturnType,
	Type,
};

/** Produces a `#[calm_io::pipefail]` function attribute.

This attribute fails the build if it is applied to a non-function item, or a
function which does not return `io::Result<_>`.
**/
#[proc_macro_attribute]
pub fn pipefail(_attrs: TokenStream, item: TokenStream) -> TokenStream {
	let mut func = parse_macro_input!(item as ItemFn);
	//  TODO: Figure out how to extract the success type, then rewrite it to be
	// `Result<Option<SUCCESS>, io::Error>`.
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
