//! LL(1) table generator.

use cfg::{Cfg, Rule, EPSILON, END_OF_INPUT, Frozen};
use cfg::util::{compute_follow, compute_first_of, Follow};

pub struct Table {
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
    Table { stuff: stuff }
}
