extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_str, GenericParam, TypeParam, WherePredicate, Generics, Lifetime, LifetimeDef};

#[proc_macro_derive(SerializeFn)]
pub fn sfn(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    impl_serialize_fn(&ast)
}
#[proc_macro_derive(Packet)]
pub fn packet(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    let mut result = impl_packet(&ast);
    result.extend(impl_serialize_fn(&ast));
    //eprintln!("{}", result);
    result
}
fn impl_packet(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Struct(data) = &ast.data {
        let life = syn::parse_str::<Lifetime>("'this").unwrap();
        let (i, t, w) = ast.generics.split_for_impl();
        let name = &ast.ident;
        let mut im = syn::parse::<Generics>(i.into_token_stream().into()).unwrap();
        if !im.lifetimes().any(|x| x.lifetime.ident == life.ident) {
            im.params.push(GenericParam::Lifetime(LifetimeDef::new(life.clone())));
        }
        //let mut t = syn::parse::<Generics>(t.into_token_stream().into()).unwrap();
        //t.params.push(GenericParam::Lifetime(LifetimeDef::new(life)));
        let fields: Vec<_> = match &data.fields {
            syn::Fields::Named(n) => n
                .named
                .iter()
                .map(|x| (x.ident.clone().unwrap().into_token_stream(), x.ty.clone()))
                .collect(),
            syn::Fields::Unnamed(x) => x
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, f)| (syn::Index::from(index).into_token_stream(), f.ty.clone()))
                .collect(),
            syn::Fields::Unit => todo!(),
        };
        let f_names: Vec<_> = fields.iter().map(|(x, _)| x).collect();
        let f_types: Vec<_> = fields.iter().map(|(_, x)| x).collect();
        quote! {
            impl #im Packet<#life> for #name #t #w {
                fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
                    #(
                        let w = <#f_types as Packet>::serialize(&self.#f_names, w)?;
                    )*
                    Ok(w)
                }

                fn deserialize(input: &#life [u8]) -> IResult<&#life [u8], Self> {
                    nom::combinator::map(nom::sequence::tuple((
                        #(
                            <#f_types as Packet>::deserialize
                        ),*
                    )), |(#(#f_names),*)| Self {#(#f_names),*})(input)
                }
            }
        }.into()
    } else {
        panic!("xd")
    }
}
fn impl_serialize_fn(ast: &syn::DeriveInput) -> TokenStream {
    let (_, ty_generics, _) = ast.generics.split_for_impl();
    let name = ast.ident.clone();
    let mut type_params = ast.generics.params.clone();
    type_params.push(GenericParam::Type(parse_str::<TypeParam>("__W").unwrap()));
    let mut g = ast.generics.clone();
    let where_clause = g.make_where_clause();
    where_clause
        .predicates
        .push(parse_str::<WherePredicate>("__W: ::std::io::Write").unwrap());
    let r = quote! {
        impl <#type_params> FnOnce<(WriteContext<__W>, )> for #name #ty_generics #where_clause{
            type Output = GenResult<__W>;

            extern "rust-call" fn call_once(self, args: (WriteContext<__W>, )) -> Self::Output {
                self.serialize(args.0)
            }
        }
        impl <#type_params> Fn<(WriteContext<__W>, )> for #name #ty_generics #where_clause{
            extern "rust-call" fn call(&self, args: (WriteContext<__W>, )) -> Self::Output {
                self.serialize(args.0)
            }
        }
        impl <#type_params> FnMut<(WriteContext<__W>, )> for #name #ty_generics #where_clause{
            extern "rust-call" fn call_mut(&mut self, args: (WriteContext<__W>, )) -> Self::Output {
                self.serialize(args.0)
            }
        }
    }
    .into();
    r
}
