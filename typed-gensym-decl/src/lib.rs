use proc_macro_hack::proc_macro_hack;

pub use typed_gensym_macro::symgen;
#[proc_macro_hack]
pub use typed_gensym_macro::local_symgen;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
