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
