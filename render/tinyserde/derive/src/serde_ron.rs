use crate::shared::*;
use proc_macro2::{TokenStream};
use syn::{
    parse_quote,
    //Ident,
    DeriveInput,
   // Fields,
    FieldsNamed,
   // FieldsUnnamed,
   // DataEnum,
   // LitInt,
   LitStr,
   // Generics,
   // WherePredicate,
   // WhereClause
};
use quote::quote;

pub fn derive_ser_ron_struct(input: &DeriveInput, fields: &FieldsNamed) -> TokenStream {
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let bound = parse_quote!(SerRon);
    let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
    let ident = &input.ident;
    let fieldname = fields.named.iter().map( | f | f.ident.clone().unwrap()).collect::<Vec<_>>();
    let fieldstring = fields.named.iter().map( | f | {
        let ident = f.ident.clone().unwrap();
        LitStr::new(&ident.to_string(), ident.span())
    }).collect::<Vec<_>>();
    
    quote! {
        impl #impl_generics SerRon for #ident #ty_generics #bounded_where_clause {
            fn ser_ron(&self, d:usize, p:&str, s: &mut String) {
                makepad_tinyserde::ser_ron_struct_pre(d,p,s);
                #(
                    self.#fieldname.ser_ron(d+1, #fieldstring, s);
                ) *
                makepad_tinyserde::ser_ron_struct_post(d,p,s);
            }
        }
    }
}


pub fn derive_de_ron_struct(input: &DeriveInput, fields:&FieldsNamed) -> TokenStream {
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let bound = parse_quote!(DeRon);
    let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
    let ident = &input.ident;
    let fieldname = fields.named.iter().map( | f | f.ident.clone().unwrap()).collect::<Vec<_>>();
    let fieldstring = fields.named.iter().map( | f | {
        let ident = f.ident.clone().unwrap();
        LitStr::new(&ident.to_string(), ident.span())
    }).collect::<Vec<_>>();
    
    quote! {
        impl #impl_generics DeRon for #ident #ty_generics #bounded_where_clause {
            fn de_ron(p: &str, s: &mut makepad_tinyserde::DeRonState, i: &mut std::str::Chars)->Result<Self, String> {
                s.is_prefix_paren_open(p, i)?;
                let r = #ident{#(
                    #fieldname: DeRon::de_ron(#fieldstring, s, i)?,
                ) *};
                s.is_paren_close(i)?;
                Ok(r)
            }
        }
    }
}

/*
pub fn derive_ser_ron_struct_unnamed(input: &DeriveInput, fields:&FieldsUnnamed) -> TokenStream {
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let bound = parse_quote!(SerBin);
    let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
    let ident = &input.ident;

    let mut fieldname = Vec::new();
    for (index, field) in fields.unnamed.iter().enumerate() {
        fieldname.push(LitInt::new(&format!("{}", index), field.span()));
    }
    quote! {
        impl #impl_generics SerBin for #ident #ty_generics #bounded_where_clause {
            fn ser_bin(&self, s: &mut Vec<u8>) {
                #(
                    self.#fieldname.ser_bin(s);
                ) *
            }
        }
    }
}


pub fn derive_de_ron_struct_unnamed(input: &DeriveInput, fields:&FieldsUnnamed) -> TokenStream {
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let ident = &input.ident;
    let bound = parse_quote!(DeBin);
    let bounded_where_clause = where_clause_with_bound(&input.generics, bound);

    let mut fieldname = Vec::new();
    for (index, field) in fields.unnamed.iter().enumerate() {
        fieldname.push(LitInt::new(&format!("{}", index), field.span()));
    }

    quote! {
        impl #impl_generics DeBin for #ident #ty_generics #bounded_where_clause {
            fn de_bin(o:&mut usize, d:&[u8]) -> Self {
                Self {
                    #(
                        #fieldname: DeBin::de_bin(o,d),
                    ) *
                }
            }
        }
    }
}

pub fn derive_ser_ron_enum(input: &DeriveInput, enumeration: &DataEnum) -> TokenStream {
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let bound = parse_quote!(SerBin);
    let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
    
    let ident = &input.ident;
    
    let mut match_item = Vec::new();
    
    for (index, variant) in enumeration.variants.iter().enumerate() {
        let lit = LitInt::new(&format!("{}u16", index), ident.span());
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => {
                match_item.push(quote!{
                    Self::#ident => #lit.ser_bin(s),
                })
            },
            Fields::Named(fields_named) => {
                let mut field_names = Vec::new();
                for field in &fields_named.named {
                    if let Some(ident) = &field.ident {
                        field_names.push(ident);
                    }
                }
                match_item.push(quote!{
                    Self::#ident {#(#field_names,) *} => {
                        #lit.ser_bin(s);
                        #(#field_names.ser_bin(s);) *
                    }
                });
            },
            Fields::Unnamed(fields_unnamed) => {
                let mut field_names = Vec::new();
                for (index, field) in fields_unnamed.unnamed.iter().enumerate() {
                    field_names.push(Ident::new(&format!("f{}", index), field.span()));
                }
                match_item.push(quote!{
                    Self::#ident (#(#field_names,) *) => {
                        #lit.ser_bin(s);
                        #(#field_names.ser_bin(s);) *
                    }
                });
            },
        }
    }
    
    quote! {
        impl #impl_generics SerBin for #ident #ty_generics #bounded_where_clause {
            fn ser_bin(&self, s: &mut Vec<u8>) {
                match self {
                    #(
                        #match_item
                    ) *
                }
            }
        }
    }
}

pub fn derive_de_ron_enum(input: &DeriveInput, enumeration: &DataEnum) -> TokenStream {
    
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let ident = &input.ident;
    let bound = parse_quote!(DeBin);
    let bounded_where_clause = where_clause_with_bound(&input.generics, bound);
    
    let mut match_item = Vec::new();
    
    for (index, variant) in enumeration.variants.iter().enumerate() {
        let lit = LitInt::new(&format!("{}u16", index), ident.span());
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => {
                match_item.push(quote!{
                    #lit => Self::#ident,
                })
            },
            Fields::Named(fields_named) => {
                let mut field_names = Vec::new();
                for field in &fields_named.named {
                    if let Some(ident) = &field.ident {
                        field_names.push(quote!{#ident: DeBin::de_bin(o,d)});
                    }
                }
                match_item.push(quote!{
                    #lit => Self::#ident {#(#field_names,) *},
                });
            },
            Fields::Unnamed(fields_unnamed) => {
                let mut field_names = Vec::new();
                for _ in &fields_unnamed.unnamed {
                    field_names.push(quote! {DeBin::de_bin(o,d)});
                }
                match_item.push(quote!{
                    #lit => Self::#ident(#(#field_names,) *),
                });
            },
        }
    }
    
    quote! {
        impl #impl_generics DeBin for #ident #ty_generics #bounded_where_clause {
            fn de_bin(o:&mut usize, d:&[u8]) -> Self {
                let id: u16 = DeBin::de_bin(o,d);
                match id {
                    #(
                        #match_item
                    ) *
                    _ => panic!("enum match failed")
                }
            }
        }
    }
}
*/