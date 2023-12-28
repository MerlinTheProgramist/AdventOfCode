#![feature(map_try_insert)]

use std::collections::HashMap;
use std::ops::Range;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum Type {
    Gt,
    Lt,
}

#[derive(Debug, Clone)]
enum Resultat {
    Other(String),
    Next(),
    End(bool),
}
impl Resultat{
    pub fn from_str(s:&str) -> Self{
        match s.chars().next().unwrap() {
            'R' => Self::End(false),
            'A' => Self::End(true),
            _ => Self::Other(s.to_owned()),
        }
    }
}

#[derive(Debug)]
struct Rule {
    val_id: usize,
    cmp: Option<Type>,
    val: i32,
    result: Resultat,
}

type Part = [Range<i32>; 4];

struct Split{
    valid: Option<(Part, Resultat)>,
    invalid: Option<(Part, Resultat)>
}

impl Rule {
    pub fn apply(&self, part: &Part) -> Split {
        match self.cmp {
            None => Split{valid:Some((part.clone(), self.result.clone())), invalid:None},
            Some(cmp) => {
                let range = &part[self.val_id];
                let (invalid,valid) = match cmp {
                    Type::Gt => (range.start..(self.val+1).min(range.end), (self.val+1).max(range.start)..range.end),
                    Type::Lt => (self.val.max(range.start)..range.end, range.start..self.val.min(range.end)),
                };
                let invalid = if invalid.len()<=0{
                    None
                }else{
                    let mut new_part = part.clone();
                    new_part[self.val_id] = invalid;
                    Some((new_part, Resultat::Next()))
                };

                let valid = if valid.len()<=0{
                    None
                }else{
                    let mut new_part = part.clone();
                    new_part[self.val_id] = valid;
                    Some((new_part, self.result.clone()))
                };
                Split{valid, invalid}
            }
        }
    }
}




fn main() {
    let mut rules = HashMap::<String, Vec<Rule>>::new();

    let file = read_to_string("p.in").unwrap();
    let mut lines = file.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (name, line) = line[..line.len() - 1].split_once('{').unwrap();

        // println!("name: ({})",name);
        let ruleset = rules.try_insert(name.to_owned(), vec![]).unwrap();
        for rule in line.split(',') {
            ruleset.push(match rule.split_once(':'){
                None => { 
                    Rule{
                    val_id: 0,
                    cmp: None,
                    val: 0,
                    result: Resultat::from_str(rule)}},
                Some((line, res)) =>{ 
                    let mut line = line.chars();
                    let var = match line.next().unwrap() {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => panic!("undefined var type"),
                    };
                    let op = match line.next().unwrap() {
                        '>' => Type::Gt,
                        '<' => Type::Lt,
                        _ => panic!("unexpected operator"),
                    };
                    let val = line.collect::<String>().parse().unwrap();
                    println!("key: ({})",res);
                    Rule {
                        val_id: var,
                        cmp: Some(op),
                        val,
                        result: Resultat::from_str(res),
                    }
                    }
                });
            
        }
    }

    const N:Range<i32> = 1..4001;
    
    let mut ans = 0;
    let mut stack = Vec::from([([N;4], rules["in"].iter())]);
    while let Some((part,mut rule)) = stack.pop(){
        let splitted = rule.next().unwrap().apply(&part);
        println!("for rule: {:?} \nvalid: {:?} invalid: {:?}\n", rule, splitted.valid, splitted.invalid);
        for split in [splitted.invalid, splitted.valid]{
            if let Some((split_part, res)) = split{
                match res{
                    Resultat::End(accept) => 
                        if accept{
                            ans += split_part.iter().map(|x| x.len()).product::<usize>();    
                        },
                    Resultat::Other(other) => 
                        stack.push((split_part, rules[&other].iter())),
                    Resultat::Next() => 
                        stack.push((split_part, rule.clone())),
                }
            }        
        }
    }
    println!("{}", ans);    
}

