use std::fs::read_to_string;

const N: usize = 256;

#[derive(Default)]
struct Item {
    key: String,
    val: u8,
}

struct HashMap {
    objects: [Vec<Item>; N],
}

impl HashMap {
    pub fn insert(&mut self, key: &String, val: u8) {
        let mut items = &mut self.objects[Self::hash(&key)];
        for item in items.iter_mut() {
            if item.key == *key {
                item.val = val;
                return;
            }
        }
        items.push(Item {
            key: key.clone(),
            val,
        });
    }
    pub fn remove(&mut self, key: &String) {
        self.objects[Self::hash(&key)].retain(|x| x.key != *key);
    }

    fn hash(s: &String) -> usize {
        s.as_bytes()
            .iter()
            .fold(0, |acc, &c| ((acc + c as usize) * 17) % N)
    }
    fn get_power(&self) -> u32 {
        let mut res = 0;
        for (b, vec) in self.objects.iter().enumerate() {
            for (pos, item) in vec.iter().enumerate() {
                res += (b + 1) * (pos + 1) * (item.val as usize);
            }
        }
        return res as u32;
    }
}

fn main() {
    let mut map = HashMap {
        objects: std::array::from_fn(|_| vec![]),
    };

    for s in read_to_string("p.in").unwrap().trim().split(',') {
        if s.as_bytes()[s.len() - 2] == b'=' {
            let val = s.chars().last().unwrap().to_digit(10).unwrap() as u8;
            map.insert(&s[..s.len() - 2].to_string(), val);
        } else if *s.as_bytes().last().unwrap() == b'-' {
            map.remove(&s[..s.len() - 1].to_string());
        }
    }
    println!("{}", map.get_power());
}
