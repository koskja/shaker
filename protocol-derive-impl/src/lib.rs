mod lifetime;

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use std::collections::HashSet;

use lifetime::get_type_lifetimes;
use proc_macro2::TokenStream;
use quote::{ToTokens, __private::Literal};
use syn::{
    parse::Parser, punctuated::Punctuated, ExprLit, Lifetime, MetaList,
    NestedMeta,
};

/// Derives a `Packet` implementation for a given struct.
/// The struct is represented as a heterogenous list of its fields, and the `Packet` impl is deferred to each of their respective impls.
/// # Lifetimes
/// Similar to `serde`, deserialization requires explicit lifetime bounds when working with non-`'static` fields.
/// This macro uses a lifetime `'_t`, (resulting in `Packet<'_t>`), and by default adds bounds by extracting lifetimes from all the struct's fields.
/// ```rust
/// #[derive(Packet)]
/// struct Foo<'a, 'b> {
///     a: Cow<'a, str>,
///     b: HashMap<&'b str, &'a u32>
/// }
/// ```
/// This macro use would result in an implementation analogous to:
/// ```rust
/// impl<'_t: 'a + 'b, 'a, 'b> Packet<'_t> for Foo<'a, 'b> { ... }
/// ```
/// If implicit lifetime bounds aren't desired, one can apply `#[packet(borrow = "explicit")]` to the entire struct,
/// disabling implicit lifetime bounds generation. Then it is neccessary to use `#[packet(borrow)]` or `#[packet(borrow = "'a + 'b")]` for the individual fields. It is also possible to use these attributes without explicit borrows. In that case, they override the default.

pub fn packet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    let result = impl_packet(&ast);
    //eprintln!("{}", result);
    result.into()
}
fn impl_packet(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let g = with_this_lifetime(ast);
    let (impl_generics, _, where_clause) = g.split_for_impl();
    let (_, type_generics, _) = ast.generics.split_for_impl();
    match &ast.data {
        syn::Data::Struct(data) => {
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
            if fields.len() == 0 {
                return quote! {
                impl #impl_generics Packet<'_t> for #name #type_generics #where_clause {
                    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                        Ok(w)
                    }

                    fn deserialize(input: &'_t [u8]) -> nom::IResult<&'_t [u8], Self> {
                        Ok((input, Self {}))
                    }
                }

                }
            }
            let f_names: Vec<_> = fields.iter().map(|(x, _)| x).collect();
            let f_types: Vec<_> = fields.iter().map(|(_, x)| x).collect();
            quote! {
                impl #impl_generics Packet<'_t> for #name #type_generics #where_clause {
                    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                        #(
                            let w = <#f_types as Packet>::serialize(&self.#f_names, w)?;
                        )*
                        Ok(w)
                    }

                    fn deserialize(input: &'_t [u8]) -> nom::IResult<&'_t [u8], Self> {
                        nom::combinator::map(nom::sequence::tuple((
                            #(
                                <#f_types as Packet>::deserialize
                            ),*,
                        )), |(#(#f_names),*,)| Self {#(#f_names),*})(input)
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
                            quote!(#last_discriminant)
                        } else {
                            quote!(#last_discriminant + #d_offset)
                        };
                        d_offset += 1;
                        variants.push((
                            discriminant_expr,
                            variant.ident.clone(),
                            variant.fields.clone(),
                        ));
                        (variants, last_discriminant, d_offset)
                    },
                )
                .0;
            let l: Vec<_> = variants.iter().map(|(x, _, _)| x).collect();
            let variant_contents: Vec<_> = variants
                .iter()
                .map(|(_, ident, x)| match x {
                    syn::Fields::Named(n) => {
                        let names: Vec<_> =
                            n.named.iter().map(|x| x.ident.clone().unwrap()).collect();
                        (
                            quote! {
                                #ident {
                                    #(
                                        #names
                                    ),*
                                }
                            },
                            names,
                        )
                    }
                    syn::Fields::Unnamed(u) => {
                        let vars: Vec<_> = gen_identifiers(u.unnamed.len()).collect();
                        (
                            quote! {
                                #ident(#(
                                    #vars
                                ),*)
                            },
                            vars,
                        )
                    }
                    syn::Fields::Unit => todo!(),
                })
                .collect();
            let variant_patterns: Vec<_> = variant_contents.iter().map(|(x, _)| x).collect();
            let variant_values: Vec<Vec<_>> =
                variant_contents.iter().map(|(_, x)| x.clone()).collect();
            quote! {
                impl #impl_generics Packet<'_t> for #name #type_generics #where_clause {
                    #[allow(unused_variables)]
                    fn serialize<W: std::io::Write>(&self, w: cookie_factory::WriteContext<W>) -> cookie_factory::GenResult<W> {
                        let discriminant = match self {
                            #(
                                Self::#variant_patterns => #l
                            ),*
                        };
                        let w = cookie_factory::bytes::be_u8(discriminant as u8)(w)?;
                        match self {
                            #(
                                Self::#variant_patterns => {
                                    #(
                                        let w = Packet::serialize(#variant_values, w)?;
                                    )*
                                    Ok(w)
                                }
                            ),*
                        }
                    }

                    fn deserialize(input: &'_t [u8]) -> nom::IResult<&'_t [u8], Self> {
                        todo!()
                    }
                }
            }
        }
        syn::Data::Union(_) => panic!("Cannot derive for a union"),
    }
}
fn cmp_str(p: &impl ToTokens, s: &str) -> bool {
    format!("{}", p.to_token_stream()) == s
}
fn parse_attr(a: &syn::Attribute, path: &str, key: &str) -> Option<Option<syn::Lit>> {
    let m = a.parse_meta().ok()?;
    if !cmp_str(m.path(), path) {
        return None;
    }
    match m {
        syn::Meta::Path(_) => None,
        syn::Meta::List(MetaList { nested, .. }) => {
            let n = &nested[0];
            if let NestedMeta::Meta(syn::Meta::NameValue(nested)) = n {
                if cmp_str(&nested.path, key) {
                    Some(Some(nested.lit.clone()))
                } else {
                    None
                }
            } else if let NestedMeta::Meta(syn::Meta::Path(p)) = n {
                if cmp_str(&p.segments, key) {
                    Some(None)
                } else {
                    None
                }
            } else {
                None
            }
        }
        syn::Meta::NameValue(_) => None,
    }
}
fn get_borrow_lifetimes(ast: &syn::DeriveInput) -> Punctuated<Lifetime, syn::Token![+]> {
    let outer = ast.attrs.iter().find(|x| cmp_str(&x.path, "packet"));
    let implicit_lifetimes = if let Some(a) = outer {
        if let Some(syn::Lit::Str(s)) = parse_attr(a, "packet", "borrow").flatten() {
            s.value() != "explicit"
        } else {
            true
        }
    } else {
        true
    };

    let a: Vec<Punctuated<Lifetime, syn::Token![+]>> = match &ast.data {
        syn::Data::Struct(s) => s
            .fields
            .iter()
            .map(|x| {
                let attr = x
                    .attrs
                    .iter()
                    .find_map(|a| parse_attr(a, "packet", "borrow"));
                if let Some(y) = attr {
                    if let Some(y) = y {
                        if let syn::Lit::Str(s) = y {
                            let parser =
                                Punctuated::<Lifetime, syn::Token![+]>::parse_separated_nonempty;
                            parser.parse_str(&s.value()).unwrap()
                        } else {
                            Punctuated::new()
                        }
                    } else {
                        let mut p = Punctuated::new();
                        p.extend(get_type_lifetimes(&x.ty));
                        p
                    }
                } else if implicit_lifetimes {
                    let mut p = Punctuated::new();
                    p.extend(get_type_lifetimes(&x.ty));
                    p
                } else {
                    Punctuated::new()
                }
            })
            .collect(),
        syn::Data::Enum(e) => e
            .variants
            .iter()
            .map(|x| {
                let attr = x
                    .attrs
                    .iter()
                    .find_map(|a| parse_attr(a, "packet", "borrow"));
                if let Some(x) = attr {
                    syn::parse_quote!(#x)
                } else if implicit_lifetimes {
                    let mut p = Punctuated::new();
                    match &x.fields {
                        syn::Fields::Named(syn::FieldsNamed { named: fields, .. })
                        | syn::Fields::Unnamed(syn::FieldsUnnamed {
                            unnamed: fields, ..
                        }) => p.extend(
                            fields
                                .iter()
                                .flat_map(|field| get_type_lifetimes(&field.ty).into_iter()),
                        ),
                        syn::Fields::Unit => {}
                    };
                    p
                } else {
                    Punctuated::new()
                }
            })
            .collect(),
        syn::Data::Union(_) => panic!("unions are not supported"),
    };
    let mut lifetimes = HashSet::new();
    lifetimes.extend(a.into_iter().flat_map(Punctuated::into_iter));
    let mut p: Punctuated<_, syn::Token![+]> = Punctuated::new();
    p.extend(lifetimes.into_iter());
    let l: Punctuated<Lifetime, syn::Token![+]> = syn::parse_quote!(#p);
    //eprintln!("{}", l.to_token_stream());
    l
}
fn add_this_lifetime(
    generics: &mut syn::Generics,
    borrow_bounds: Punctuated<Lifetime, syn::Token![+]>,
) {
    let this = syn::parse_str::<Lifetime>("'_t").unwrap();
    if generics.lifetimes().any(|x| x.lifetime.ident == this.ident) {
        panic!("'_t is a reserved lifetime for Packet<'_t>!")
    }
    if borrow_bounds.is_empty() {
        generics.params.push(syn::parse_quote!(#this))
    } else {
        generics
            .params
            .push(syn::parse_quote!(#this: #borrow_bounds))
    }
}
fn with_this_lifetime(ast: &syn::DeriveInput) -> syn::Generics {
    let mut generics = ast.generics.clone();
    add_this_lifetime(&mut generics, get_borrow_lifetimes(ast));
    generics
}
fn gen_identifiers(count: usize) -> impl Iterator<Item = syn::Ident> {
    (0..count)
        .into_iter()
        .map(|x| syn::Ident::new(&format!("field{}", x), proc_macro2::Span::call_site()))
}
