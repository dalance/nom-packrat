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
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

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
/// * (Optional) An extra key type. The type must implement `Eq + Hash + Clone`.
/// * (Optional) Maximum entries of storage.
///
/// # Examples
///
/// ```compile_fail
/// storage!(String);
/// storage!(String, 1024);
/// storage!(String, ExtraInfo);
/// storage!(String, ExtraInfo, 1024);
/// ```
#[macro_export]
macro_rules! storage {
    ($t:ty) => {
        thread_local!(
            pub(crate) static PACKRAT_STORAGE: core::cell::RefCell<
                nom_packrat::PackratStorage<$t, ()>
            > = {
                core::cell::RefCell::new(nom_packrat::PackratStorage::new(None))
            }
        );
    };
    ($t:ty, $u:ty) => {
        thread_local!(
            pub(crate) static PACKRAT_STORAGE: core::cell::RefCell<
                nom_packrat::PackratStorage<$t, $u>
            > = {
                core::cell::RefCell::new(nom_packrat::PackratStorage::new(None))
            }
        );
    };
    ($t:ty, $n:expr) => {
        thread_local!(
            pub(crate) static PACKRAT_STORAGE: core::cell::RefCell<
                nom_packrat::PackratStorage<$t, ()>
            > = {
                core::cell::RefCell::new(nom_packrat::PackratStorage::new(Some($n)))
            }
        );
    };
    ($t:ty, $u:ty, $n:expr) => {
        thread_local!(
            pub(crate) static PACKRAT_STORAGE: core::cell::RefCell<
                nom_packrat::PackratStorage<$t, $u>
            > = {
                core::cell::RefCell::new(nom_packrat::PackratStorage::new(Some($n)))
            }
        );
    };
}

pub struct PackratStorage<T, U> {
    size: Option<usize>,
    map: HashMap<(&'static str, *const u8, U), Option<(T, usize)>>,
    keys: VecDeque<(&'static str, *const u8, U)>,
}

impl<T, U> PackratStorage<T, U>
where
    U: Eq + Hash + Clone,
{
    pub fn new(size: Option<usize>) -> Self {
        let init_size = size.unwrap_or_else(|| 0);
        PackratStorage {
            size: size,
            map: HashMap::with_capacity(init_size),
            keys: VecDeque::with_capacity(init_size),
        }
    }

    pub fn get(&self, key: &(&'static str, *const u8, U)) -> Option<&Option<(T, usize)>> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: (&'static str, *const u8, U), value: Option<(T, usize)>) {
        if let Some(size) = self.size {
            if self.keys.len() > size - 1 {
                let key = self.keys.pop_front().unwrap();
                self.map.remove(&key);
            }
        }

        self.keys.push_back(key.clone());
        self.map.insert(key, value);
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.keys.clear();
    }
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

impl<T> HasExtraState<()> for nom_locate::LocatedSpanEx<T, ()> {
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

impl<T> HasExtraState<()> for nom_locate2::LocatedSpan<T, ()> {
    fn get_extra_state(&self) -> () {
        ()
    }
}

impl<T, U, V> HasExtraState<T> for nom_locate2::LocatedSpan<U, V>
where
    V: HasExtraState<T>,
{
    fn get_extra_state(&self) -> T {
        self.extra.get_extra_state()
    }
}
