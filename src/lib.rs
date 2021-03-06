


#[derive(Debug)]
pub struct TypedSymbol<T> {
    id: u64,
    /// Note, this tag is unsafely copied whenever we copy a symbol.
    /// This is because we use the same affine zero sized type
    /// as generator and tag.
    tag: T,
}

impl<T> Clone for TypedSymbol<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            tag: unsafe { core::mem::transmute_copy::<T, T>(&self.tag) },
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

#[doc(hidden)]
pub use typed_gensym_decl::symgen as __symgen__;

/// Defines a type tagged symbol generator.
/// ```
/// # use typed_gensym::symgen;
/// symgen!(MyGen);
/// fn main() {
///    let mut a = MyGen::claim().unwrap();
///    println!("Symbol: {:?}", a.gensym());
/// }
/// ```
#[macro_export]
macro_rules! symgen {
    ($($input:tt)*) => {
        $crate::__symgen__! {
            #![crate = $crate]
            $($input)*
        }
    }
}

#[doc(hidden)]
pub use typed_gensym_decl::local_symgen as __local_symgen__;

/// Defines and returns an unnameable type tagged symbol generator.
/// ```
/// # use typed_gensym::local_symgen;
/// fn main() {
///    let mut a = local_symgen!(MyGen);
///    println!("Symbol: {:?}", a.gensym());
/// }
/// ```
/// If it turns out that block expression scopes become nameable,
/// this becomes unsafe.
#[macro_export]
macro_rules! local_symgen {
    ($($input:tt)*) => {
        $crate::__local_symgen__! {
            #![crate = $crate]
            $($input)*
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
