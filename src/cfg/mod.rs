//! Context-free grammar representation
//!
//! # Introduction to CFGs
//!
//! A context-free grammar is a formalism that is able to represent all context-free languages
//! using two sorts of symbols: terminals, and nonterminals. Nonterminals can be replaced,
//! according to the rules of the CFG, by a sequence of other symbols. Terminals cannot be
//! replaced, and thus represent the "termination" of the rewriting.
//!
//! The parsing problem is to determine whether a sequence of terminals can be "derived" by
//! repeated application of any of the rules of the CFG, and if so, what that derivation is (the
//! sequence of rules applied to which non-terminals that produced that string).
//!
//! # Limitations
//!
//! While many interesting languages are context-free, parsing some of them can be problematic. Of
//! particular concern are *ambiguous* grammars, where there are multiple possible derivations of
//! at least one string. Additionally, parsing CFGs is, in general, equivalent to matrix
//! multiplication, which has a complexity of at least roughly O(n^2.37). Some grammars require
//! non-deterministic execution. It is possible to use the limited subset of CFGs that are able to
//! be parsed deterministically.
//!
//! # Representation
//!
//! Here, symbols are represented as 32-bit unsigned integers, with the most significant bit used
//! to distinguish terminals from non-terminals. This leads to 2^31 possible nonterminals, and 2^31
//! possible terminals. This limitation will almost certainly never be hit in practice. Rules are
//! currently represented equivalently to a `Symbol` LHS with a `Vec<Symbol>` RHS, and a
//! `Vec<Rule>` is stored in the the `Cfg`.
//!
//! Each `Cfg` also has associated with it an `extra` field, which is a `TypeMap`. This allows
//! other libraries to store arbitrary data in the `Cfg` without fear of clashing with other
//! libraries.
//!
//! ɛ is represented as a rule with an empty rhs.
//!
//! # Freezing
//!
//! For efficiency, many algorithms may want to compute relations on the set of terminals,
//! nonterminals, or rules of a `Cfg`, and cache the results. However, later mutations to the `Cfg`
//! may invalidate the old computation. As a concrete example, consider the `FOLLOW` sets of a
//! nonterminal. Adding more rules can cause the `FOLLOW` sets of some nonterminals to expand. As
//! such, many of these computations should be run on a `Cfg` that cannot have rules or grammar
//! symbols added to it.  Thus, `Cfg` is a phantom type, with the extra type parameter noting
//! whether the `Cfg` is mutable or frozen.
//!
//! # Future avenues of improvement
//!
//! - Some sort of prefix trie might be nice to store the rules compactly. It seems that efficient
//!   indexing would be challenging.

pub mod marpa;
pub mod util;

/// A Symbol is either a non-terminal or a terminal.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Symbol {
    Terminal(u32),
    Nonterminal(u32),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
/// A PackedSymbol is a more compact representation of a Symbol
pub struct PackedSymbol(u32);

impl PackedSymbol {
    pub fn is_terminal(self) -> bool {
        self.0 & (1 << 31) == 0
    }

    pub fn is_nonterminal(self) -> bool {
        !self.is_terminal()
    }
}

impl<'a> ::std::convert::From<&'a PackedSymbol> for Symbol {
    fn from(v: &'a PackedSymbol) -> Symbol {
        Symbol::from(*v)
    }
}

impl ::std::convert::From<PackedSymbol> for Symbol {
    fn from(v: PackedSymbol) -> Symbol {
        let val = v.0;
        if val & (1 << 31) == 0 {
            Symbol::Terminal(val & !(1 << 31))
        } else {
            Symbol::Nonterminal(val & !(1 << 31))
        }
    }
}

impl<'a> ::std::convert::From<&'a Symbol> for PackedSymbol {
    fn from(v: &'a Symbol) -> PackedSymbol {
        PackedSymbol::from(*v)
    }
}

impl ::std::convert::From<Symbol> for PackedSymbol {
    fn from(v: Symbol) -> PackedSymbol {
        match v {
            Symbol::Terminal(x) => PackedSymbol(x),
            Symbol::Nonterminal(x) => PackedSymbol(x | (1 << 31)),
        }
    }
}

/// A representation of a context-free grammar.
///
/// This implementation admits no more than 2^31 terminals and 2^31 nonterminals and 2^32 rules,
/// where each rule can have at most 2^16 grammar symbols on the right-hand side.
pub struct Cfg<T> {
    phantom: ::std::marker::PhantomData<T>,
    rules: Vec<(PackedSymbol, Vec<PackedSymbol>)>,
    extra: ::typemap::TypeMap,
    start: usize,
    // why i32's? because trying to increment past 2^31 will cause an overflow error, which is
    // precisely what we want.
    max_nonterm: i32,
    max_term: i32,
}

/// A Rule maps from a nonterminal to a sequence of symbols it can be replaced with.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Rule(pub usize);

/// A marker type indicating that a `Cfg` can be mutated.
pub struct Mutable;
/// A marker type indicating that a `Cfg` can not be mutated.
pub struct Frozen;

impl Cfg<Mutable> {
    /// Construct a CFG for the empty language.
    pub fn new() -> Cfg<Mutable> {
        Cfg { phantom: ::std::marker::PhantomData, start: 0, rules: Vec::new(), max_nonterm: 0, max_term: 0, extra: ::typemap::TypeMap::new() }
    }

    /// Add a nonterminal, returning the new grammar symbol that can be used.
    pub fn add_nonterminal(&mut self) -> Symbol {
        let val = self.max_nonterm;
        self.max_nonterm += 1;
        Symbol::Nonterminal(val as u32)
    }

    /// Add a terminal, returning the new grammar symbol that can be used.
    pub fn add_terminal(&mut self) -> Symbol {
        let val = self.max_term;
        self.max_term += 1;
        Symbol::Terminal(val as u32)
    }

    /// Add a rule to the grammar, `lhs -> rhs[0] rhs[1] ...`, returning a `Rule`.
    pub fn add_rule<'a, L, R>(&mut self, lhs: L, rhs: &'a [R]) -> Rule where PackedSymbol: From<L> + From<&'a R>, {
        self.rules.push((lhs.into(), rhs.iter().map(PackedSymbol::from).collect()));
        Rule(self.rules.len() - 1)
    }

    /// Set the start rule.
    pub fn set_start(&mut self, r: Rule) {
        self.start = r.0;
    }

    /// Freeze this `Cfg`, preventing later mutations.
    pub fn freeze(self) -> Cfg<Frozen> {
        let Cfg { phantom: _phantom, rules, extra, start, max_nonterm, max_term } = self;
        Cfg {
            phantom: ::std::marker::PhantomData::<Frozen>,
            rules: rules,
            extra: extra,
            start: start,
            max_nonterm: max_nonterm,
            max_term: max_term
        }
    }
}

impl<T> Cfg<T> {
    /// Get a reference to the `extra` data.
    pub fn extra(&self) -> &::typemap::TypeMap {
        &self.extra
    }

    /// Get a mutable reference to the `extra` data.
    pub fn mut_extra(&mut self) -> &mut ::typemap::TypeMap {
        &mut self.extra
    }

    /// Get a rule from the grammar.
    pub fn get_rule(&self, r: Rule) -> Option<(PackedSymbol, &[PackedSymbol])> {
        self.rules.get(r.0).map(|&(s, ref r)| (s, &r[..]))
    }

    /// Number of terminals used by the grammar.
    ///
    /// Each integer from 0 to this number (exclusive) is a valid terminal.
    pub fn num_terminals(&self) -> u32 {
        self.max_term as u32
    }

    /// Number of nonterminals used by the grammar.
    ///
    /// Each integer from 0 to this number (exclusive) is a valid nonterminal.
    pub fn num_nonterminals(&self) -> u32 {
        self.max_nonterm as u32
    }

    /// Number of rules used by the grammar.
    ///
    /// Each `Rule(i)` from 0 to this number (exclusive) is a valid rule.
    pub fn num_rules(&self) -> usize {
        self.rules.len()
    }

    /// Iterator over all the rules in the grammar.
    pub fn rules<'a>(&'a self) -> ::std::slice::Iter<'a, (PackedSymbol, Vec<PackedSymbol>)> {
        self.rules.iter()
    }
}
