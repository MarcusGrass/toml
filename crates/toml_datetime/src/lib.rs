//! A [TOML]-compatible datetime type
//!
//! [TOML]: https://github.com/toml-lang/toml

#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
// Makes rustc abort compilation if there are any unsafe blocks in the crate.
// Presence of this annotation is picked up by tools such as cargo-geiger
// and lets them ensure that there is indeed no unsafe code as opposed to
// something they couldn't detect (e.g. unsafe added via macro expansion, etc).
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

extern crate alloc;

mod datetime;

pub use crate::datetime::Date;
pub use crate::datetime::Datetime;
pub use crate::datetime::DatetimeParseError;
pub use crate::datetime::Offset;
pub use crate::datetime::Time;

#[doc(hidden)]
#[cfg(feature = "serde")]
pub mod __unstable {
    pub use crate::datetime::DatetimeFromString;
    pub use crate::datetime::FIELD;
    pub use crate::datetime::NAME;
}
