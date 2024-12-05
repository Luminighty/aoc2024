use std::{cmp::Ordering, collections::HashSet};

pub struct Rule(pub i32, pub i32);

pub type Update = Vec<i32>;

fn prohibit_from_rules(prohibit: &mut HashSet<i32>, rules: &Vec<Rule>, page: i32) {
    for rule in rules {
        if rule.1 == page {
            prohibit.insert(rule.0);
        }
    }
}

pub fn is_valid_update(rules: &Vec<Rule>, update: &Update) -> bool {
    let mut prohibit = HashSet::new();

    for page in update {
        if prohibit.contains(page) {
            return false;
        }
        prohibit_from_rules(&mut prohibit, rules, *page);
    }

    true
}

pub fn update_middle(update: &Update) -> i32 {
    update[update.len() / 2]
}

pub fn compare_update(rules: &Vec<Rule>, left: &i32, right: &i32) -> Ordering {
    for rule in rules {
        match (&rule.0, &rule.1) {
            (l, r) if l == left && r == right => return Ordering::Less,
            (l, r) if l == right && r == left => return Ordering::Greater,
            _ => {}
        };
    }
    Ordering::Equal
}
