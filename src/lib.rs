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

#[derive(Debug)]
pub struct TypedSymbol<T> {
    id: u64,
    /// Note, this MUST ALWAYS be a zero sized type.
    /// Given that the only way to create these symbols is to
    /// go through the generated constructors,
    /// there shouldn't be any way to violate that invariant.
    /// But, if it is somehow violated, know that we're transmuting
    /// a `()` into this whenever we make a copy of a symbol.
    tag: T,
}

impl<T: UnsafeClone> Clone for TypedSymbol<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            tag: unsafe { UnsafeClone::clone(&self.tag) },
        }
    }
}
impl<T> PartialEq for TypedSymbol<T> {
    fn eq(&self, s: &Self) -> bool {
        self.id == s.id
    }
}
impl<T> Eq for TypedSymbol<T> {}

/// Not an intentional part of the public API.
/// Don't frickin' touch.
#[doc(hidden)]
pub unsafe fn __create_typed_symbol<T>(id: u64, tag: T) -> TypedSymbol<T> {
    TypedSymbol { id, tag }
}
/// This is also not part of the public API.
/// I use it for copying the ZST tags in places that can't be reached
/// through the public API.
/// If misused, this can cause undefined behavior.
#[doc(hidden)]
pub unsafe trait UnsafeClone {
    unsafe fn clone(&self) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
