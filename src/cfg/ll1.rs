//! LL(1) table generator.

use cfg::{Cfg, Rule, EPSILON, END_OF_INPUT, Frozen, Token, Symbol, PackedSymbol};
use cfg::util::{compute_follow, compute_first_of, Follow};

pub struct Table {
    start: Symbol,
    rules: Vec<(PackedSymbol, Vec<PackedSymbol>)>,
    stuff: Vec<Vec<Option<Rule>>>
}

pub fn generate_table(cfg: &mut Cfg<Frozen>) -> Table {
    compute_follow(cfg);
    let follow = cfg.extra().get::<Follow>().unwrap();
    println!("{:?}", follow);

    let mut stuff = vec![vec![None; cfg.max_term as usize]; cfg.max_nonterm as usize];
    for (i, &(lhs, ref rhs)) in cfg.rules().enumerate() {
        println!("Processing rule {:?}: {:?} -> {:?}", i, lhs, rhs);
        let first = compute_first_of(cfg, rhs);
        for term in &first {
            let slot = &mut stuff[lhs.to_index()][term.to_index()];
            if slot.is_some() {
                panic!("Conflict in LL(1) table generation! Sorry boss.");
            }
            *slot = Some(Rule(i));
        }
        if first.contains(&EPSILON) {
            let follow = &follow[lhs.to_index()];
            for b in follow {
                let slot = &mut stuff[lhs.to_index()][b.to_index()];
                if slot.is_some() {
                    panic!("Conflict in LL(1) table generation! Sorry boss.");
                }
                *slot = Some(Rule(i));
            }
            /*
            if follow.contains(&END_OF_INPUT) {
                let slot = &mut stuff[lhs.to_index()][END_OF_INPUT.to_index()];
                if slot.is_some() {
                    panic!("Conflict in LL(1) table generation! Sorry boss.");
                }
                *slot = Some(Rule(i));
            }
            */
        }
    }
    Table { rules: cfg.rules.clone(), start: cfg.rules[cfg.start].0.into(), stuff: stuff }
}

/// Parse a string, returning the rules applied to derive the string.
pub fn parse(tab: &Table, mut s: Vec<&Token>) -> Vec<Rule> {
    let mut derivation = Vec::new();
    s.push(&END_OF_INPUT);
    let mut stack = vec![END_OF_INPUT.into(), tab.start];
    let mut idx = 0;
    let mut a = s[idx];
    while (*stack.last().unwrap() != END_OF_INPUT.into()) {
        if (*stack.last().unwrap() == a.to_terminal().into()) { stack.pop(); a = s[idx+1]; idx += 1; }
        else if (stack.last().unwrap().is_terminal()) { panic!("Parse error!"); }
        else if (tab.stuff[stack.last().unwrap().to_index()][a.to_terminal().to_index()].is_none()) { panic!("Parse error!") }
        else { // it's a nonterminal, and not an error!
            let rule_idx = tab.stuff[stack.last().unwrap().to_index()][a.to_terminal().to_index()].unwrap().0;
            let &(lhs, ref rhs) = &tab.rules[rule_idx];
            derivation.push(Rule(rule_idx));
            stack.pop();
            for sym in rhs.iter().rev() {
                stack.push(sym.into());
            }
        }
    }
    derivation
}
