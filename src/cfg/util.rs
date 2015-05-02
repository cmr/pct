use std::collections::{HashMap, HashSet, VecDeque};
use super::{Cfg, PackedSymbol};

pub struct Nullability;
impl ::typemap::Key for Nullability { type Value = HashSet<PackedSymbol>; }

/// Computes the nullability relation N : Nonterminal -> bool.
///
/// A nonterminal is said to be nullable if it can derive the empty string in 1 or more steps.
///
/// This algorithm is currently O(n^2), but for common cases will terminate very quickly.
/// Pathological input is incredibly rare and not worth considering.
pub fn compute_nullability<T>(cfg: &mut Cfg<T>) {
    let mut nullable = HashSet::new();
    {
        let mut queue = VecDeque::new();
        let mut uses = HashMap::new();
        // maps from rhs to (num_null, counts, number_symbols).
        let mut rules = HashMap::new();
        for &(lhs, ref rhs) in cfg.rules() {
            if rhs.is_empty() {
                nullable.insert(lhs);
                queue.push_back(lhs);
            } else {
                let mut counts = HashMap::new();
                for sym in rhs {
                    if sym.is_nonterminal() {
                        *counts.entry(sym).or_insert(0) += 1;
                        uses.entry(sym).or_insert(HashSet::new()).insert(rhs);
                    }
                }
                rules.insert((lhs, rhs), (0, counts));
            }
        }

        while let Some(lhs) = queue.pop_front() {
            for &rule in uses.get(&lhs).unwrap_or(&HashSet::new()) {
                let &mut (ref mut count, ref counts) = rules.get_mut(&(lhs, rule)).unwrap();
                *count += *counts.get(&lhs).unwrap();
                if *count == rule.len() {
                    nullable.insert(lhs);
                    queue.push_back(lhs);
                }
            }
        }
    }

    cfg.mut_extra().insert::<Nullability>(nullable);
}

/*
pub struct First;
impl ::typemap::Key for First { type Value = HashMap<Vec<PackedSymbol>, HashSet<PackedSymbol>>; }

pub fn compute_first_of_memoized<T>(cfg: &mut Cfg<T>, seq: Vec<PackedSymbol>) -> &HashSet<PackedSymbol> {
    let sigh = seq.clone();

    match cfg.mut_extra().entry::<First>().unwrap().entry(seq).or_insert_with(|| compute_first_of(cfg, &sigh)) {
}
*/

// Returns true if we should keep looking in following symbols.
fn compute_first_of_symbol<T>(cfg: &Cfg<T>, set: &mut HashSet<PackedSymbol>, sym: PackedSymbol) -> bool {
    if sym.is_terminal() {
        set.insert(sym);
        if sym == super::EPSILON {
            true
        } else {
            false
        }
    } else {
        let mut first_eps = false;
        for &(lhs, ref rhs) in cfg.rules() {
            if lhs == sym {
                let mut hs = HashSet::new();
                let mut all_eps = true;
                for sym in rhs {
                    if !compute_first_of_symbol(cfg, &mut hs, *sym) {
                        all_eps = false;
                        set.extend(hs.drain());
                        break;
                    }
                    set.extend(hs.drain());
                }
                if all_eps {
                    set.insert(super::EPSILON);
                    first_eps = true;
                }
            }
        }
        first_eps
    }
}

pub fn compute_first_of<'a, T, R>(cfg: &mut Cfg<T>, seq: &'a [R]) -> HashSet<PackedSymbol> where PackedSymbol: From<&'a R> {
    let mut first = HashSet::new();
    for sym in seq {
        if !compute_first_of_symbol(cfg, &mut first, sym.into()) {
            println!("Ok, giving up after FIRST of {:?}", PackedSymbol::from(sym));
            break;
        }
    }
    first
}

pub struct Follow;
impl ::typemap::Key for Follow { type Value = Vec<HashSet<PackedSymbol>>; }

/// Computes the FOLLOW relation F : Nonterminal -> HashSet<Terminal>
pub fn compute_follow(cfg: &mut Cfg<super::Frozen>) {
    let mut follow = vec![HashSet::with_capacity(cfg.max_term as usize); cfg.max_nonterm as usize];
    let mut stable_relation = Vec::new();
    {
        follow[cfg.rules[cfg.start].0.to_index()].insert(super::END_OF_INPUT); // Add $ to to FOLLOW(S)
        for &(lhs, ref rhs) in cfg.rules() {
            for loc in 0..rhs.len() {
                if rhs[loc].is_nonterminal() {
                    let first = compute_first_of(cfg, rhs[loc+1..]);
                    if first.remove(&super::EPSILON) || loc == rhs.len()-1 {
                        stable_relation.push((lhs, rhs[loc])):
                    }
                    follow[rhs[loc]].extend(first);
                }
            }
        }
        // so fast. wow. very efficient.
        let mut stable = false;
        while !stable {
            let old = follow.clone();
            for &(from, to) in &stable_relation {
                follow[to].extend(follow[from])
            }
        }
    }
    cfg.mut_extra().insert::<Follow>(follow);
}
