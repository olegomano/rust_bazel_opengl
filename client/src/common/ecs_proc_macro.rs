use std::collections::HashSet as Set;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated,AttributeArgs, ItemStruct, Token,parse_quote, Ident};
use syn::parse::{Parse, ParseStream, Result};

struct ArgsHoldingIdents {
  idents: std::vec::Vec<Ident>,
}

impl Parse for ArgsHoldingIdents {
  fn parse(args: ParseStream) -> Result<Self> {
    let vars = punctuated::Punctuated::<Ident, Token![,]>::parse_terminated(args)?;
    Ok(ArgsHoldingIdents {
      idents: vars.into_iter().collect(),
    })
  }
}

#[proc_macro_attribute]
pub fn ecs(attr: TokenStream, input: TokenStream) -> TokenStream {
    
    let args = parse_macro_input!(attr as ArgsHoldingIdents);
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as ItemStruct);

    // Extract the identifier and generics from the input struct
    let struct_ident = &input.ident;
    let generics = &input.generics;
    let fields = &input.fields;
    let idents = &args.idents;
    
    let enum_quote = quote!{
        #[derive(Debug)]
        enum ComponentType {
            #(#idents(#idents)),*
        }
    };
    
    let gen = quote! {
        #enum_quote

        #[derive(Debug)]
        struct #struct_ident #generics {
            #fields
            //entity_manager : EntityManager<ComponentEnum>,
        }
    };
    return gen.into()
}
