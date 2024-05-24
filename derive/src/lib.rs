use quote::quote;

#[proc_macro_attribute]
pub fn box_dyn(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // let mut input: syn::DeriveInput = syn::parse2(input.into()).unwrap();
    let trait_item: syn::ItemTrait = syn::parse2(input.into()).unwrap();
    let trait_name = &trait_item.ident;
    dbg!(trait_name.to_string());

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

                    // }

                    let param_names: Vec<_> = func
                        .sig
                        .inputs
                        .iter()
                        .filter_map(|arg| match arg {
                            syn::FnArg::Typed(ty) => Some(ty.pat.clone()),
                            _ => None,
                        })
                        .collect();

                    func.default = Some(
                        syn::parse2::<syn::Block>(quote! {{
                            #trait_name::#func_name(#self_typ, #(#param_names)*)
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

    // let trait_impls: Vec<syn::ItemImpl> = vec![];
    // syn::parse2::<syn::ItemImpl>(quote! {
    //             impl<T> #trait_path for Box<T> where T: #trait_path {
    //             }
    //         })

    let out = quote! {
        #trait_item

        impl<T> #trait_name for Box<T> where T: #trait_name {
            #(#trait_items)*
        }
        // #trait_impl
        // #(#trait_impls)*
    };
    println!("{}", pretty_print(&out));
    out.into()
}

// #[proc_macro_derive(BoxDyn)]
// pub fn impl_trait_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let mut input: syn::DeriveInput = syn::parse2(input.into()).unwrap();
//     let trait_impls: Vec<syn::ItemImpl> = vec![];
//     // syn::parse2::<syn::ItemImpl>(quote! {
//     //             impl<T> #trait_path for Box<T> where T: #trait_path {
//     //             }
//     //         })
//
//     let out = quote! {
//         impl<T> #trait_path for Box<T> where T: #trait_path {
//         }
//         // #trait_impl
//         // #(#trait_impls)*
//     };
//     println!("{}", pretty_print(&out));
//     out.into()
// }

// #[proc_macro]
// pub fn impl_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let mut input: syn::ItemFn = syn::parse2(input.clone().into()).unwrap();
//     // let mut func_ast: syn::ItemFn = syn::parse2(input.clone().into()).unwrap();
//     // let mut test: Vec<syn::Path> =
//     // let mut test: syn::ItemStruct =
//     let types: Vec<syn::Type> =
//         syn::parse_macro_input!(input with syn::punctuated::Punctuated::<syn::Type, syn::Token![,]>::parse_terminated)
//             .into_iter()
//             .collect();
//
//     // dbg!(&types.iter().map(|t| quote! { #t }).collect::<Vec<_>>());
//     let traits: Vec<_> = types
//         .into_iter()
//         .map(|t| match t {
//             syn::Type::Path(typ) => typ.path,
//             _ => panic!("expected list of trait names"),
//         })
//         .collect();
//
//     dbg!(&traits
//         .iter()
//         .map(|typ| quote! { #typ }.to_string())
//         .collect::<Vec<_>>());
//     // dbg!(&types.iter().map(|t| quote! { #t }).collect::<Vec<_>>());
//
//     // let mut test: syn::Attribute =
//     //     syn::parse2("MyTrait, crate::test::Test2".parse().unwrap()).unwrap();
//
//     // dbg!(&test.in);
//     // let values = vec![];
//     // let a = "test";
//     // if a == "uwe" {
//     //     println!("hi");
//     //
//     //     if a == "petra" {
//     //         println!("ho");
//     //     }
//     //     // here
//     // }
//     // "fn answer() -> u32 { 42 }".parse().unwrap()
//
//     // fn get_target_traits(ast: ItemStruct) -> Vec<String> {
//     // let tests: Vec<_> = test
//     //     // .attrs
//     //     .iter()
//     //     .filter_map(|attr| match &attr.meta {
//     //         syn::Meta::List(l) => match l.path.segments.first() {
//     //             Some(seg) => Some((&seg.ident, l)),
//     //             _ => None,
//     //         },
//     //         _ => None,
//     //     })
//     //     .filter(|a| a.0 == "box_dyn_traits")
//     //     .flat_map(|a| {
//     //         a.1.tokens
//     //             .clone()
//     //             .into_iter()
//     //             .map(|t| t.to_string())
//     //             .filter(|t| t != ",")
//     //     })
//     //     .collect();
//
//     // dbg!(&tests);
//
//     let trait_impls: Vec<_> = traits
//         .into_iter()
//         .map(|trait_path| {
//             // let method_signatures =
//             syn::parse2::<syn::ItemImpl>(quote! {
//                 impl<T> #trait_path for Box<T> where T: #trait_path {
//                 }
//             })
//             .unwrap()
//         })
//         .collect();
//
//     // let test: Vec<proc_macro2::TokenStream> = vec![];
//     let out = quote! {
//         // struct _Test {}
//         // #test
//         #(#trait_impls)*
//         // #trait_impls
//     };
//     println!("{}", pretty_print(&out));
//     out.into()
// }

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
