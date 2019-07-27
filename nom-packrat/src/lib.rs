//! `nom-packrat` is an extension of [nom](https://docs.rs/nom) to apply "Packrat Parsing".
//!
//! ## Examples
//!
//! The following example show a quick example.
//!
//! ```
//! use nom::character::complete::char;
//! use nom::IResult;
//! use nom_packrat::{init, packrat_parser, storage};
//!
//! // Declare storage used by packrat_parser
//! storage!(String);
//!
//! // Apply packrat_parser by custom attribute
//! #[packrat_parser]
//! pub fn parser(s: &str) -> IResult<&str, String> {
//!     let (s, x) = char('a')(s)?;
//!     Ok((s, x.to_string()))
//! }
//!
//! fn main() {
//!     let input = "a";
//!
//!     // Initialize before parsing
//!     init!();
//!     let result = parser(input);
//!
//!     println!("{:?}", result);
//! }
//! ```

extern crate nom_packrat_macros;
#[doc(inline)]
pub use nom_packrat_macros::packrat_parser;

/// Initialize packrat storage
///
/// This must be called before each parsing.
/// If this is not called, the parse result may be wrong.
#[macro_export]
macro_rules! init {
    () => {
        crate::PACKRAT_STORAGE.with(|storage| storage.borrow_mut().clear())
    };
}

/// Declare packrat storage
///
/// # Arguments
/// * An output type of parser. The type must implement `Clone`.
///
/// # Examples
///
/// ```compile_fail
/// storage!(String);
/// ```
#[macro_export]
macro_rules! storage {
    ($t:ty) => {
        thread_local!(
            pub(crate) static PACKRAT_STORAGE: core::cell::RefCell<
                std::collections::HashMap<(&'static str, *const u8, ()), Option<($t, usize)>>
            > = {
                core::cell::RefCell::new(std::collections::HashMap::new())
            }
        );
    };
    ($t:ty, $u:ty) => {
        thread_local!(
            pub(crate) static PACKRAT_STORAGE: core::cell::RefCell<
                std::collections::HashMap<(&'static str, *const u8, $u), Option<($t, usize)>>
            > = {
                core::cell::RefCell::new(std::collections::HashMap::new())
            }
        );
    };
}

pub trait HasExtraState<T> {
    fn get_extra_state(&self) -> T;
}

impl HasExtraState<()> for &str {
    fn get_extra_state(&self) -> () {
        ()
    }
}

impl HasExtraState<()> for &[u8] {
    fn get_extra_state(&self) -> () {
        ()
    }
}

impl<T, U, V> HasExtraState<T> for nom_locate::LocatedSpanEx<U, V>
where
    V: HasExtraState<T>,
{
    fn get_extra_state(&self) -> T {
        self.extra.get_extra_state()
    }
}
