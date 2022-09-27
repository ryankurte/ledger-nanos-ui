//! Helper macros for nanos_ui

use proc_macro2::TokenStream;

use syn::{parse_macro_input, Data, DeriveInput, Fields, Attribute, Meta, NestedMeta, Lit};
use quote::quote;
use darling::{FromMeta};


const ATTRIBUTE_GROUP: &str = "menu";

#[proc_macro_derive(Menu, attributes(menu))]
pub fn derive_menu_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse macro inputs
    let DeriveInput { ident, data, generics, attrs, .. } = parse_macro_input!(input);

    // Only applies to Enum types
    let d = match data {
        Data::Enum(e) => e,
        _ => panic!("Unsupported object type for derivation"),
    };
    
    let mut states = vec![];
    let mut event_handles = vec![];
    let mut draw_handles = vec![];

    // Parse variants
    for (_i, v) in d.variants.iter().enumerate() {
        // Track names for state enum
        states.push(&v.ident);

        let ident = &v.ident;

        let h = match &v.fields {
            Fields::Unit => quote!{Self::#ident => ()},
            Fields::Unnamed(u) => quote!{Self::#ident(d) => d.evt(evt)},
            _ => unimplemented!(),
        };
        event_handles.push(h);

        let h = match &v.fields {
            Fields::Unit => quote!{Self::#ident => ()},
            Fields::Unnamed(u) => quote!{Self::#ident(d) => d.draw()},
            _ => unimplemented!(),
        };
        draw_handles.push(h);
    }

    // Generate menu implementation
    
    let menuState = quote::format_ident!("{}State", ident);

    let stateList = quote::format_ident!("{}_STATES", ident);

    quote!{
        #[derive(Copy, Clone, Debug, PartialEq)]
        enum #menuState {
            #(#states),*
        }

        //const #stateList: &[#menuState] = &[ #(#states),* ]

        impl ::nanos_ui_traits::Menu for #ident {
            type State = #menuState;
        }

        impl ::nanos_ui_traits::Element for #ident {
            
            fn evt(&mut self, evt: ::nanos_ui_traits::Event) {
                match self {
                    #(#event_handles),*
                }
            }
            
            fn draw(&self) {
                match self {
                    #(#draw_handles),*
                }
            }
        }
    }.into()
}

struct FieldAttrs {
    pub label: Option<&'static str>,
}

impl Default for FieldAttrs {
    fn default() -> Self {
        Self {
            label: None,
        }
    }
}

impl FieldAttrs {
    /// Parse [`Attrs`] object from field attributes
    pub fn parse<'a>(attrs: impl Iterator<Item=&'a Attribute>) -> Self {
        // Filter for attribute group
        let attribute_args = attrs
            .filter_map(|v| v.parse_meta().ok() )
            .find(|v| v.path().is_ident(ATTRIBUTE_GROUP))
            .map(|v| match v {
                Meta::List(l) => Some(l.nested),
                _ => None,
            })
            .flatten();

        let mut s = Self::default();
        
        // Skip if we don't have a matching group
        let attrs = match attribute_args {
            Some(a) => a,
            None => return s,
        };

        // Parse attributes
        for a in attrs {
            // Filter NameValue attributes
            let v = match a {
                NestedMeta::Meta(Meta::NameValue(v)) => v,
                _ => continue,
            };

            // Process literal from value
            let l = match lit_to_quote(&v.lit) {
                Some(l) => l,
                None => continue,
            };

        }

        // Return attributes
        s
    }
}

fn lit_to_quote(lit: &Lit) -> Option<TokenStream> {
    match lit {
        Lit::Int(v) => Some(quote!{ #v }),
        Lit::Str(v) => {
            let f = v.value();
            let i = syn::Ident::from_string(&f).unwrap();
            Some(quote!{ #i })
        },
        Lit::Verbatim(v) => Some(quote!{ #v }),
        _ => None,
    }
}
