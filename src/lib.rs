use quote::quote;
use syn::parse::Parse;

#[proc_macro_attribute]
pub fn box_dyn(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // let mut input: syn::DeriveInput = syn::parse2(input.into()).unwrap();
    let additional_bounds = syn::parse_macro_input!(args with syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated);
    // let args_parsed: syn::punctuated::Punctuated<syn::Path, syn::Token![,]>::parse_terminated =
    //     syn::parse2(input.into()).unwrap();
    // let args_parsed = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
    //     .parse2(args)
    //     .unwrap();
    let trait_item: syn::ItemTrait = syn::parse2(input.into()).unwrap();
    let trait_name = &trait_item.ident;
    let trait_generics = &trait_item.generics;
    // let trait_with_generics = quote! { #trait_name };
    // generics
    // dbg!(trait_name.to_string());

    let trait_items: Vec<_> = trait_item
        .items
        .clone()
        .into_iter()
        .filter_map(|mut item| {
            match item {
                // An associated constant within the definition of a trait.
                syn::TraitItem::Const(ref mut val) => {
                    // default
                    // syn::TraitItemConst
                    val.default = todo!();
                    Some(item)
                    // Some(val.into_token_stream())
                }
                // An associated function within the definition of a trait.
                syn::TraitItem::Fn(ref mut func) => {
                    // syn::TraitItemFn
                    let func_name = &func.sig.ident;
                    // dbg!(&func_name);
                    let receiver = func
                        .sig
                        .inputs
                        .iter()
                        .find_map(|arg| match arg {
                            syn::FnArg::Receiver(ty) => Some(ty.clone()),
                            _ => None,
                        })
                        .expect("trait functions need receiver type");
                    let self_typ =
                        match (receiver.reference.is_some(), receiver.mutability.is_some()) {
                            (true, true) => quote! { self.as_mut() },
                            (true, false) => quote! { self.as_ref() },
                            (false, _) => quote! { self },
                        };
                    // pub reference: Option<(Token![&], Option<Lifetime>)>,
                    // pub mutability: Option<Token![mut]>,

                    let param_names: Vec<_> = func
                        .sig
                        .inputs
                        .iter()
                        .filter_map(|arg| match arg {
                            syn::FnArg::Typed(ty) => Some(ty.pat.clone()),
                            _ => None,
                        })
                        .collect();

                    // dbg!(&trait_name);
                    // dbg!(&func_name);
                    // dbg!(&self_typ);
                    // dbg!(&param_names
                    //     .iter()
                    //     .map(|p| quote! { #p })
                    //     .collect::<Vec<_>>());

                    // println!(
                    //     "{}",
                    //     pretty_print(quote! {{
                    //         #trait_name::#func_name(#self_typ, #(#param_names),*)
                    //     }})
                    // );
                    func.default = Some(
                        syn::parse2::<syn::Block>(quote! {{
                            #trait_name::#func_name(#self_typ, #(#param_names),*)
                        }})
                        .unwrap(),
                    );
                    Some(item)
                    // Some(func)
                    // pub attrs: Vec<Attribute>,
                    // pub sig: Signature,
                    // pub default: Option<Block>,
                    //
                }

                // An associated type within the definition of a trait.
                syn::TraitItem::Type(typ) => {
                    // syn::TraitItemType
                    typ.default = todo!();
                    Some(item)
                }
                // A macro invocation within the definition of a trait.
                // syn::TraitItemMacro
                // syn::TraitItem::Macro(_) => None,
                // Tokens within the definition of a trait not interpreted by Syn.
                // syn::TokenStream
                // syn::TraitItem::Verbatim(_) => None,
                _ => None,
            }
        })
        .collect();

    let t = quote! { __BoxDynT };
    let trait_where_predicates = &trait_generics
        .where_clause
        .as_ref()
        .map(|clause| &clause.predicates);
    let trait_generic_params = &trait_generics.params;

    let t_bounds: Vec<_> = [quote! { #trait_name #trait_generics }]
        .into_iter()
        .chain(additional_bounds.into_iter().map(|b| quote! { #b }))
        .collect();

    let out = quote! {
        #trait_item

        impl<#t, #trait_generic_params> #trait_name #trait_generics for Box<#t>
        where
            #t: #(#t_bounds)+*,
            #trait_where_predicates
        {
            #(#trait_items)*
        }
    };
    println!("{}", pretty_print(&out));
    out.into()
}

#[allow(dead_code)]
fn pretty_print<T>(input: T) -> String
where
    T: quote::ToTokens,
{
    let file: syn::File = syn::parse2(quote! {
        fn main() {
            #input
        }
    })
    .unwrap();
    prettyplease::unparse(&file)
}
