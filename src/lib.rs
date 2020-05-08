/// Defines and returns an unnameable type tagged symbol generator.
/// ```
/// # use typed_gensym::local_symgen;
/// fn main() {
///    let mut a = local_symgen!(MyGen);
///    println!("Symbol: {:?}", a.gensym());
/// }
/// ```
pub use typed_gensym_decl::local_symgen;
/// Defines a type tagged symbol generator.
/// ```
/// # use typed_gensym::symgen;
/// symgen!(MyGen);
/// fn main() {
///    let mut a = MyGen::claim().unwrap();
///    println!("Symbol: {:?}", a.gensym());
/// }
/// ```
pub use typed_gensym_decl::symgen;

#[derive(Debug, PartialEq, Eq)]
pub struct TypedSymbol<T> {
    id: u64,
    tag: T,
}

/// Not an intentional part of the public API.
/// Don't frickin' touch.
#[doc(hidden)]
pub unsafe fn __create_typed_symbol<T>(id: u64, tag: T) -> TypedSymbol<T> {
    TypedSymbol { id, tag }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
