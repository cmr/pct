#![cfg(test)]

use cfg::{ll1, Symbol, Cfg, EPSILON, END_OF_INPUT};
use cfg::util::{compute_first_of, Follow, compute_follow};

use cfg::bnf::{from_str, to_string};

#[test]
fn first_is_correct() {
    let mut cfg = Cfg::new();
    let s = cfg.add_nonterminal();
    let a = cfg.add_nonterminal();
    let b = cfg.add_nonterminal();
    let c = cfg.add_nonterminal();

    let d = cfg.add_terminal();
    let e = cfg.add_terminal();
    let f = cfg.add_terminal();

    // Construct the grammar:
    //
    // S -> A B C
    // A -> EPSILON | d
    // B -> e
    // C -> EPSILON | f

    let _rs = cfg.add_rule(s, &[a, b, c]);
    let _ra1 = cfg.add_rule(a, &[EPSILON]);
    let _ra2 = cfg.add_rule(a, &[d]);
    let _rb = cfg.add_rule(b, &[e]);
    let _rc1 = cfg.add_rule(c, &[EPSILON]);
    let _rc2 = cfg.add_rule(c, &[f]);

    assert_eq!(compute_first_of(&mut cfg, &[s]), vec![EPSILON, d.into(), e.into()].into_iter().collect());
    assert_eq!(compute_first_of(&mut cfg, &[a, c]), vec![EPSILON, d.into(), f.into()].into_iter().collect());
    assert!(compute_first_of::<_, Symbol>(&mut cfg, &[]).is_empty());
}

#[test]
fn from_str_is_correct() {
    let input = "S A B C
A
A d
B e
C
C f";
    let c = from_str(input);
    assert_eq!("S -> ABC
A -> ε
A -> d
B -> e
C -> ε
C -> f", to_string(&c));
}

#[test]
fn follow_is_correct() {
    let mut cfg = Cfg::new();
    let s = cfg.add_nonterminal();
    let a = cfg.add_nonterminal();
    let b = cfg.add_nonterminal();
    let c = cfg.add_nonterminal();

    let d = cfg.add_terminal();
    let e = cfg.add_terminal();
    let f = cfg.add_terminal();

    // Construct the grammar:
    //
    // S -> A B C
    // A -> EPSILON | d
    // B -> e
    // C -> EPSILON | f

    let _rs = cfg.add_rule(s, &[a, b, c]);
    let _ra1 = cfg.add_rule(a, &[EPSILON]);
    let _ra2 = cfg.add_rule(a, &[d]);
    let _rb = cfg.add_rule(b, &[e]);
    let _rc1 = cfg.add_rule(c, &[EPSILON]);
    let _rc2 = cfg.add_rule(c, &[f]);


    let mut cfg = cfg.freeze();
    compute_follow(&mut cfg);
    let follow = cfg.extra().get::<Follow>().unwrap();
    let expected = vec![vec![END_OF_INPUT].into_iter().collect(), vec![e.into()].into_iter().collect(), vec![END_OF_INPUT, f.into()].into_iter().collect(), vec![END_OF_INPUT].into_iter().collect()];
    assert_eq!(follow, &expected);
}

#[test]
fn can_make_ll1_table() {
    let mut cfg = Cfg::new();
    let s = cfg.add_nonterminal();
    let a = cfg.add_nonterminal();
    let b = cfg.add_nonterminal();
    let c = cfg.add_nonterminal();

    let d = cfg.add_terminal();
    let e = cfg.add_terminal();
    let f = cfg.add_terminal();

    // Construct the grammar:
    //
    // S -> A B C
    // A -> EPSILON | d
    // B -> e
    // C -> EPSILON | f

    let _rs = cfg.add_rule(s, &[a, b, c]);
    let _ra1 = cfg.add_rule(a, &[EPSILON]);
    let _ra2 = cfg.add_rule(a, &[d]);
    let _rb = cfg.add_rule(b, &[e]);
    let _rc1 = cfg.add_rule(c, &[EPSILON]);
    let _rc2 = cfg.add_rule(c, &[f]);

    let mut cfg = cfg.freeze();
    let _tab = ll1::generate_table(&mut cfg);
}
