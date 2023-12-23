#![feature(map_try_insert)]
use std::collections::{VecDeque,HashMap};
use std::fs::read_to_string;
use std::collections::hash_map::Entry;

const TARGET:&str= "rx";

fn gcd(a: u64, b: u64) ->u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(v: &Vec<u64>) -> u64 {
    let mut ans = v[0];
    for n in v.iter().skip(1) {
        ans = (*n * ans) / gcd(*n, ans);
    }
    return ans;
}


#[derive(Debug, Clone)]
enum ModuleFunction {
    Conjunction { states: HashMap<String, bool> },
    FlipFlop { state: bool },
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
        
    let feed = modules.iter().find_map(|(name,module)| if module.outputs.contains(&TARGET.into()){Some(name)}else{None}).unwrap().clone();

    let mut cycle_lenghts = HashMap::<String, u64>::new();
    let mut seen = HashMap::<String, u64>::new();
    for (name, module) in modules.iter(){
        if module.outputs.contains(&feed.clone()){
            seen.insert(name.into(),0);
        }
    }

    for press in 1.. {
        // origin, target, pulse
        let mut q = VecDeque::new();
        for target in &broadcast_targets {
            q.push_back(("broadcast".to_owned(), target.clone(), false));
        }
        while let Some((origin, target, pulse)) = q.pop_front() {
            if let Some(module) = modules.get_mut(&target) {

                if target == feed && pulse == true{
                    let count = seen.entry(origin.clone()).or_insert(0);
                    *count += 1;
                    // first time we see, update in dictionary
                    match cycle_lenghts.entry(origin.clone()){
                        Entry::Vacant(v) => _ = v.insert(press),
                        Entry::Occupied(o) => {
                            assert!(press == *o.get() * *count)
                        }
                    }
                    if seen.values().all(|&x| x>0){
                        println!("{}", lcm(&cycle_lenghts.values().map(|&x| x).collect::<Vec<u64>>()));
                        return;
                    }
                }
                    
                if let Some(res) = module.function.apply(&origin, pulse) {
                    for x in &module.outputs {
                        q.push_back((target.clone(), x.clone(), res));
                    }
                }
            }
        }
    }
}
