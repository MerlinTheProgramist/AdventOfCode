#![feature(map_try_insert)]
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::default::Default;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum ModuleFunction {
    Conjunction { states: HashMap<String, bool> },
    FlipFlop { state: bool },
    None,
}
impl ModuleFunction {
    pub fn apply(&mut self, from: &String, pulse: bool) -> Option<bool> {
        match self {
            Self::Conjunction { states } => {
                states.insert(from.clone(), pulse);
                Some(!states.values().all(|&x| x))
            }
            Self::FlipFlop { state } => {
                if !pulse {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            }
            Self::None => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    outputs: Vec<String>,
    function: ModuleFunction,
}

fn main() {
    let mut broadcast_targets = Vec::new();
    let mut modules = HashMap::<String, Module>::new();
    for line in read_to_string("p.in").unwrap().lines() {
        let (src, outputss) = line.split_once(" -> ").unwrap();

        let targets = if src == "broadcaster" {
            &mut broadcast_targets
        } else {
            &mut modules
                .try_insert(
                    src[1..].to_owned(),
                    Module {
                        function: match &src[..1] {
                            "%" => ModuleFunction::FlipFlop { state: false },
                            "&" => ModuleFunction::Conjunction {
                                states: HashMap::new(),
                            },
                            _ => panic!("Undefined module type. In case of emergency please contant your nearest elf consultant or raindeer mechanic"),
                        },
                        outputs: vec![]
                    },
                )
                .unwrap().outputs
        };
        for outputs in outputss.split(", ") {
            targets.push(outputs.to_owned());
        }
    }

    for (name, module) in modules.clone().into_iter() {
        for output in module.outputs.iter() {
            if let Some(src) = &mut modules.get_mut(output) {
                if let ModuleFunction::Conjunction { ref mut states } = src.function {
                    states.insert(name.clone(), false);
                }
            }
        }
    }

    // applying the button clicks until it will cycle

    println!("targets: {:?}", broadcast_targets);
    println!("modules: {:?}", modules);

    let mut lo = 0;
    let mut hi = 0;

    for _ in 0..1000 {
        // origin, target, pulse
        lo += 1;
        let mut q = Vec::new();
        for target in &broadcast_targets {
            q.push(("broadcast".to_owned(), target.clone(), false));
        }
        while let Some((origin, target, pulse)) = q.pop() {
            if pulse {
                hi += 1;
            } else {
                lo += 1;
            }
            if let Some(module) = modules.get_mut(&target) {
                if let Some(res) = module.function.apply(&origin, pulse) {
                    for x in &module.outputs {
                        q.push((target.clone(), x.clone(), res));
                    }
                }
            }
        }
    }
    println!("{}", lo * hi);
}
