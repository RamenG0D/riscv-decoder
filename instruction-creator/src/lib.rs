use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct InputFragment {
    name: syn::Ident,
    contents: Vec<syn::Item>,
}

impl Parse for InputFragment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;

        let ctx;
        braced!(ctx in input);

        let mut contents = Vec::new();
        while !ctx.is_empty() {
            contents.push(ctx.parse()?);
        }

        Ok(InputFragment { name, contents })
    }
}

struct Instructions {
    fragments: Vec<InputFragment>,
}

impl Parse for Instructions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut fragments = Vec::new();
        while !input.is_empty() {
            fragments.push(input.parse()?);
        }
        Ok(Instructions { fragments })
    }
}

#[proc_macro]
pub fn instructions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Instructions);

    let mut output = Vec::new();
    for fragment in input.fragments {
        let (name, consts) = (fragment.name, fragment.contents);
        output.push(quote! {
            pub mod #name {
                #(#consts)*
            }
        });
    }

    quote!(#(#output)*).into()
}
