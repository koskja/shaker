use syn::Lifetime;

pub fn get_type_lifetimes(t: &syn::Type) -> Vec<Lifetime> {
    let mut res = vec![];
    l_ty(t, &mut res);
    res
}
fn l_ty(t: &syn::Type, out: &mut Vec<Lifetime>) {
    match t {
        syn::Type::Array(x) => l_ty(&x.elem, out),
        syn::Type::BareFn(x) => {
            if let syn::ReturnType::Type(_, ty) = &x.output {
                l_ty(ty, out)
            }
            for input in &x.inputs {
                l_ty(&input.ty, out)
            }
        }
        syn::Type::Group(x) => l_ty(&x.elem, out),
        syn::Type::ImplTrait(x) => l_type_param_bounds(&x.bounds, out),
        syn::Type::Infer(_) => {}
        syn::Type::Macro(_) => {}
        syn::Type::Never(_) => {}
        syn::Type::Paren(x) => l_ty(&x.elem, out),
        syn::Type::Path(x) => {
            if let Some(q) = &x.qself {
                l_ty(&q.ty, out)
            }
            l_path(&x.path, out)
        }
        syn::Type::Ptr(_) => {}
        syn::Type::Reference(x) => {
            l_ty(&x.elem, out);
            if let Some(l) = &x.lifetime {
                out.push(l.clone())
            }
        }
        syn::Type::Slice(x) => l_ty(&x.elem, out),
        syn::Type::TraitObject(x) => l_type_param_bounds(&x.bounds, out),
        syn::Type::Tuple(x) => {
            for ty in &x.elems {
                l_ty(ty, out)
            }
        }
        syn::Type::Verbatim(_) => {}
        _ => {}
    }
}
fn l_type_param_bounds<'a>(
    t: impl IntoIterator<Item = &'a syn::TypeParamBound>,
    out: &mut Vec<Lifetime>,
) {
    for bound in t {
        l_type_param_bound(bound, out)
    }
}
fn l_type_param_bound(t: &syn::TypeParamBound, out: &mut Vec<Lifetime>) {
    match t {
        syn::TypeParamBound::Trait(t) => l_path(&t.path, out),
        syn::TypeParamBound::Lifetime(l) => out.push(l.clone()),
    }
}
fn l_path(t: &syn::Path, out: &mut Vec<Lifetime>) {
    for x in t.segments.iter() {
        match &x.arguments {
            syn::PathArguments::None => {}
            syn::PathArguments::AngleBracketed(x) => {
                for arg in &x.args {
                    l_generic_argument(arg, out)
                }
            }
            syn::PathArguments::Parenthesized(x) => {
                if let syn::ReturnType::Type(_, t) = &x.output {
                    l_ty(t, out)
                }
                for i in &x.inputs {
                    l_ty(i, out)
                }
            }
        }
    }
}
fn l_generic_argument(t: &syn::GenericArgument, out: &mut Vec<Lifetime>) {
    match t {
        syn::GenericArgument::Lifetime(l) => out.push(l.clone()),
        syn::GenericArgument::Type(t) => l_ty(t, out),
        syn::GenericArgument::Binding(b) => l_ty(&b.ty, out),
        syn::GenericArgument::Constraint(c) => l_type_param_bounds(&c.bounds, out),
        syn::GenericArgument::Const(_) => {}
    }
}
