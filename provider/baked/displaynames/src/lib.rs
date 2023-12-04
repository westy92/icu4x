// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data for the icu_displaynames crate

#![no_std]
// The source is not readable and is massive as HTML.
#![doc(html_no_source)]

#[cfg(icu4x_custom_data)]
include!(concat!(core::env!("ICU4X_DATA_DIR"), "/macros.rs"));
#[cfg(not(icu4x_custom_data))]
include!("../data/macros.rs");
