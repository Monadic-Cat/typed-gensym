use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{Ident, Token, Path, bracketed, parse::{Nothing, ParseStream, Parser}};

/// Implementation of `gensym` for either kind of symgen.
fn make_gensym(krate: Path) -> proc_macro2::TokenStream {
    let expanded = quote! {
        pub fn gensym(&mut self) -> #krate::TypedSymbol<Self> {
            static mut COUNTER: u64 = 0;
            // NOTE(unsafe) since there can only exist one instance of Self
            // ever, there is no way to produce a data race.
            unsafe {
                let sym = #krate::__create_typed_symbol(COUNTER, Self {
                    _x: (),
                });
                COUNTER += 1;
                sym
            }
        }
    };
    proc_macro2::TokenStream::from(expanded)
}

fn parse_symgen_input(input: ParseStream<'_>) -> Result<(Path, Ident), syn::Error> {
    Ok({
        input.parse::<Token![#]>()?;
        input.parse::<Token![!]>()?;
        let content; bracketed!(content in input);
        content.parse::<Token![crate]>()?;
        content.parse::<Token![=]>()?;
        let krate = content.parse::<Path>()?;
        content.parse::<Nothing>()?;
        let name = input.parse::<Ident>()?;
        (krate, name)
    })
}

#[proc_macro]
pub fn symgen(input: TokenStream) -> TokenStream {
    let (krate, name) = match parse_symgen_input.parse(input) {
        | Ok(it) => it,
        | Err(err) => return err.to_compile_error().into(),
    };
    let gensym = make_gensym(krate.clone());
    let str_name = format!("{}", name);
    let module_name = Ident::new(
        &format!("__typed_gensym_{}", name),
        proc_macro2::Span::call_site(),
    );
    let expanded = quote! {
        mod #module_name {
            use #krate::__create_typed_symbol;
            pub struct #name {
                // We want to be zero sized,
                // but we also don't want the constructor to be public
                _x: (),
            }
            impl core::fmt::Debug for #name {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(f, #str_name)
                }
            }
            impl #name {
                pub fn claim() -> Option<#name> {
                    use core::sync::atomic::{self, AtomicBool, Ordering};
                    static CLAIMED: AtomicBool = AtomicBool::new(false);
                    if CLAIMED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                        Some(Self {
                            _x: (),
                        })
                    } else {
                        None
                    }
                }
                #gensym
            }
        }
        use #module_name::#name;
    };
    TokenStream::from(expanded)
}
#[proc_macro_hack]
pub fn local_symgen(input: TokenStream) -> TokenStream {
    let (krate, name) = match parse_symgen_input.parse(input) {
        | Ok(it) => it,
        | Err(err) => return err.to_compile_error().into(),
    };
    let gensym = make_gensym(krate);
    let str_name = format!("{}", name);
    let expanded = quote! {{
        struct #name {
            _x: (),
        }
        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, #str_name)
            }
        }
        impl #name {
            #gensym
        }
        #name { _x: () }
    }};
    TokenStream::from(expanded)
}
