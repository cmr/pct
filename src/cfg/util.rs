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
                uses.entry(lhs).or_insert(Vec::new()).push(rhs);
                let mut counts = HashMap::new();
                for sym in rhs {
                    *counts.entry(sym).or_insert(0) += 1;
                }
                rules.insert(rhs, (0, counts, rhs.len()));
            }
        }

        while let Some(lhs) = queue.pop_front() {
            for rule in uses.get(&lhs).unwrap_or(&Vec::new()) {
                let &mut (ref mut count, ref counts, number_symbols) = rules.get_mut(rule).unwrap();
                *count += *counts.get(&lhs).unwrap();
                if *count == number_symbols {
                    nullable.insert(lhs);
                    queue.push_back(lhs);
                }
            }
        }
    }

    cfg.mut_extra().insert::<Nullability>(nullable);
}
