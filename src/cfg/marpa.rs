//! Marpa, a Modern Earley Parser

use std::iter::ExactSizeIterator;
use super::{Cfg, Rule, PackedSymbol};

/// Parse a string of grammar symbols into a chart.
pub fn parse<T, U, I>(grammar: &Cfg<T>, string: I) where I: ExactSizeIterator<Item = U>, U: Into<PackedSymbol> {
}
