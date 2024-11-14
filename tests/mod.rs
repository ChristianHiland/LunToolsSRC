//! Lunbin tests snippet
//! This is a modules/scripts that contains functions that help test every
//! tool, or struct, Vars to errors.


// Importing Only When Running Cargo Test, saves compiled size.
#[cfg(test)]
mod path_testing;

#[cfg(test)]
mod json_testing;

#[cfg(test)]
mod vartypes_testing;