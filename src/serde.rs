// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Martin Habovstiak <martin.habovstiak@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

//! This module contains primitives used to implement serde support.

#[cfg(feature = "std")]
use ::std::borrow::Cow;
use ::core::ops::Deref;
use ::serde::Deserialize;

/// This is a helper for deserializing using `FromStr` more efficiently.
///
/// The implementation of Deserialize for Cow doesn't borrow the string,
/// so it allocates needlessly if the string is going to be passed to `FromStr`.
///
/// Our CowHelper is written such that it borrows the str, avoiding the
/// allocation.
#[cfg(feature = "serde")]
#[derive(Deserialize)]
pub struct CowHelper<'a>(#[serde(borrow)] Cow<'a, str>);

impl<'a> Deref for CowHelper<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}