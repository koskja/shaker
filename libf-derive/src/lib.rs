extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, __private::{Literal, Group}};
use syn::{
    parse_str, punctuated::Punctuated, Attribute, ExprLit, GenericParam,
    Lifetime, TypeParam, WherePredicate
};

#[proc_macro_derive(SerializeFn)]
pub fn sfn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    impl_serialize_fn(&ast).into()
}
#[proc_macro_derive(Packet, attributes(borrow))]
pub fn packet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    let mut result = impl_packet(&ast);
    result.extend(impl_serialize_fn(&ast));
    //eprintln!("{}", result);
    result.into()
}
fn impl_packet(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(data) => {
            let g = with_this_lifetime(ast);
            let (impl_generics, _, where_clause) = g.split_for_impl();
            let (_, type_generics, _) = ast.generics.split_for_impl();
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
                impl #impl_generics Packet<'this> for #name #type_generics #where_clause {
                    fn serialize<W: Write>(&self, w: WriteContext<W>) -> GenResult<W> {
                        #(
                            let w = <#f_types as Packet>::serialize(&self.#f_names, w)?;
                        )*
                        Ok(w)
                    }

                    fn deserialize(input: &'this [u8]) -> IResult<&'this [u8], Self> {
                        nom::combinator::map(nom::sequence::tuple((
                            #(
                                <#f_types as Packet>::deserialize
                            ),*
                        )), |(#(#f_names),*)| Self {#(#f_names),*})(input)
                    }
                }
            }
            .into()
        }
        syn::Data::Enum(e) => {
            let start_expr = syn::Expr::Lit(ExprLit {
                attrs: vec![],
                lit: syn::Lit::Verbatim(Literal::u64_unsuffixed(1)),
            });
            let variants = e
                .variants
                .iter()
                .fold(
                    (vec![], start_expr, 0),
                    |(mut variants, mut last_discriminant, mut d_offset), variant| {
                        let discriminant_expr = if let Some((_, e)) = &variant.discriminant {
                            last_discriminant = e.clone();
                            d_offset = 0;

                            // No offset to allow exotic tuple discriminants
                            quote!(#last_discriminant)
                        } else {
                            quote!(#last_discriminant + #d_offset)
                        };
                        d_offset += 1;
                        variants.push((discriminant_expr, variant));
                        (variants, last_discriminant, d_offset)
                    },
                )
                .0;
            let l = variants.iter().map(|(x, _)| x);
            let r = variants.iter().map(|(_, x)| x);
            let res = quote! {
                #(#l),*

                #(#r),*
            }
            .into();
            res
        }
        syn::Data::Union(_) => panic!("Cannot derive for a union"),
    }
}
fn get_all_attributes<'a>(ast: &'a syn::DeriveInput) -> impl Iterator<Item = &'a Attribute> {
    let inner: Box<dyn Iterator<Item = _>> = match &ast.data {
        syn::Data::Struct(body) => Box::new(body.fields.iter().flat_map(|x| x.attrs.iter())),
        syn::Data::Enum(body) => Box::new(body.variants.iter().flat_map(|x| x.attrs.iter())),
        syn::Data::Union(body) => Box::new(body.fields.named.iter().flat_map(|x| x.attrs.iter())),
    };
    let outer = ast.attrs.iter();
    outer.chain(inner)
}
fn get_borrow_lifetimes(ast: &syn::DeriveInput) -> Punctuated<Lifetime, syn::Token![+]> {
    let attributes = get_all_attributes(ast)
        .filter(|x: &&Attribute| format!("{}", x.path.to_token_stream()) == "borrow")
        ;
    // Extract lifetime bounds from attribute contents
    let bounds = attributes.map(|x| {
        let x = x.tokens.clone();
        if let Some(TokenTree::Group(group)) = x.into_iter().next() {
            let x = group.stream();
            eprintln!("{}", x);
            x
        } else {
            panic!()
        }
    });
    let mut p: Punctuated<_, syn::Token![+]> = Punctuated::new();
    p.extend(bounds);
    let l:Punctuated<Lifetime, syn::Token![+]>  = syn::parse_quote!(#p);
    eprintln!("{}", l.to_token_stream());
    l
}
fn add_this_lifetime(
    generics: &mut syn::Generics,
    borrow_bounds: Punctuated<Lifetime, syn::Token![+]>,
) {
    let this = syn::parse_str::<Lifetime>("'this").unwrap();
    let has_this = generics.lifetimes().any(|x| x.lifetime.ident == this.ident);
    let has_bounds = !borrow_bounds.is_empty();
    match (has_this, has_bounds) {
        (true, true) => generics.make_where_clause().predicates.push(syn::parse_quote!(#this: #borrow_bounds)),
        (true, false) => {},
        (false, true) => generics.params.push(syn::parse_quote!(#this: #borrow_bounds)),
        (false, false) => generics.params.push(syn::parse_quote!(#this)),
    }
}
fn with_this_lifetime(ast: &syn::DeriveInput) -> syn::Generics {
    let mut generics = ast.generics.clone();
    add_this_lifetime(&mut generics, get_borrow_lifetimes(ast));
    generics
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
