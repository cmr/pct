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

/* This bit of code used to be the body of compute_nullability. It is interesting in that it also
 * computes rules which cannot derive any strings, and thus never appear in a derivation
 * (*separate* from reachability by the start rule). For now, it will remain.
 *
 *  {
 *      let mut rules_to_process = Vec::new();
 *      let mut rules_to_process_2 = Vec::new();
 *
 *      for &(lhs, ref rhs) in cfg.rules() {
 *          if rhs.is_empty() {
 *              nullable.insert(lhs, true);
 *          } else if rhs.iter().any(|x| x.is_terminal()) {
 *              nullable.insert(lhs, false);
 *          } else {
 *              rules_to_process.push((lhs, rhs));
 *          }
 *      }
 *
 *      while !rules_to_process.is_empty() {
 *          if rules_to_process_2 == rules_to_process {
 *              // we haven't made any progress; mark the remaining nonterminals as not-nullable.
 *              // I suspect that what this means is that these productions cannot be applied to derive
 *              // any string (interminable recursion).
 *              for &(lhs, _) in &rules_to_process {
 *                  assert!(nullable.insert(lhs, false).unwrap_or(false) == false);
 *              }
 *          }
 *          rules_to_process_2.clear();
 *          for &(lhs, rhs) in &rules_to_process {
 *              if nullable.contains_key(&lhs) {
 *                  continue;
 *              } else if rhs.iter().any(|x| nullable.get(x) == Some(&true)) {
 *                  let mut all_null = true;
 *                  let mut has_unknown = false;
 *                  for sym in rhs {
 *                      match nullable.get(sym) {
 *                          Some(&true) => { },
 *                          Some(&false) => { all_null = false; break; },
 *                          None => { all_null = false; has_unknown = true; break; },
 *                      }
 *                  }
 *                  if all_null {
 *                      assert!(nullable.insert(lhs, true) == None);
 *                  } else if !has_unknown {
 *                      assert!(nullable.insert(lhs, false) == None);
 *                  }
 *              } else {
 *                  rules_to_process_2.push((lhs, rhs));
 *              }
 *          }
 *      }
 *  }
 */
