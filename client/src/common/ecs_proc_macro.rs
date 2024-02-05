use std::collections::HashSet as Set;
use std::any::TypeId;
use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input,punctuated,AttributeArgs, Field,ItemStruct, Token,parse_quote, Ident};
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

/*
 * Generates a function that Adds a component of the specified Type
 *
 */
fn gen_add_component_function( component : &Ident ) -> TokenStream2 {
    //let field_name = Ident::new(&("Add".to_owned() + &component.to_string()),proc_macro2::Span::CODE_POINT);
    let field_name = &component;
    return quote!{
        pub fn #field_name(&self, item : &#component, entity_id : ecs::EntityId ) -> ecs::ComponentId {
            let type_id = std::any::TypeId::of::<#component>();
            let all_components : &std::collections::HashMap<std::any::TypeId,ComponentArrayType> = 
                self.entity_manager.Components();
            
            let component_array : &ComponentArrayType = all_components.get(&type_id).unwrap();
            if let &ComponentArrayType::#field_name(array) = &component_array {
                let index = array.Allocate(entity_id,item);
                return self.entity_manager.RegisterComponent(entity_id,&type_id,index);
            }
            return ecs::ComponentId(0);
        }
    };
}

/*
* Generates 
*/
fn gen_new_function(component : &Ident) -> TokenStream2 {
    return quote!{
        unsafe{
            let component_array = ComponentArrayType::#component(ecs::ComponentArray::<#component>::new());
            let type_id = std::any::TypeId::of::<#component>();
            entity_manager._ComponentsMut().insert(type_id,component_array);    
        }
    }
}

fn gen_iterator_function(component : &Ident) -> proc_macro2::TokenStream {
    let field_name = &component;
    return quote!{
        pub fn #field_name(&self) -> impl Iterator<Item = &mut #field_name> {
            let all_components = self.entity_manager.Components();
            if ComponentArrayType::#field_name(component_array) match all_components {
                let index = component_array.Allocate(entity_id,item);
                self.entity_manager.RegisterComponent(entity_id,index);
            }else{
                //assert
            }
        }
    };
}

#[proc_macro_attribute]
pub fn ecs(attr: TokenStream, input: TokenStream) -> TokenStream {
    
    let args = parse_macro_input!(attr as ArgsHoldingIdents);
    let idents = &args.idents;
    // Parse the input tokens into a syntax tree
    let input_struct = parse_macro_input!(input as ItemStruct);
    let struct_ident = &input_struct.ident;
    
    let mut fields : std::vec::Vec<_> = input_struct.fields.iter().map(|item| item).collect();

    let enum_quote = quote!{
        #[derive(Debug)]
        enum ComponentType {
            #(#idents(#idents)),*
        }
    };

    let enum_component_array_quote = quote!{
        #[derive(Debug)]
        enum ComponentArrayType {
            #(#idents(ecs::ComponentArray<#idents>)),*
        }
    };

    let mut generated_functions : std::vec::Vec<TokenStream2> = std::vec::Vec::new();
    for component in  idents {
       let add_func = gen_add_component_function(&component); 
       generated_functions.push(add_func);
    }

    let mut generated_new : std::vec::Vec<TokenStream2> = std::vec::Vec::new();
    for component in  idents {
       let new_func = gen_new_function(&component); 
       generated_new.push(new_func);
    }

    let gen = quote! {
        #enum_quote
        #enum_component_array_quote
        
        struct #struct_ident {
            entity_manager : ecs::EntityManager<ComponentType,ComponentArrayType>,
        }
    
        impl #struct_ident {
            pub fn new() -> Self{
                let mut entity_manager = ecs::EntityManager::<ComponentType,ComponentArrayType>::new();
                #(#generated_new)*
                return Self{
                    entity_manager : entity_manager
                }; 
            }
            #(#generated_functions)*
        }

    };
    return gen.into();
}
