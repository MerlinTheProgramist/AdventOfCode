use std::fs::read_to_string;
use std::ops::RangeInclusive;

type Num = i128;

// const CROSS_RANGE:RangeInclusive<Num> = 7..=27;
const CROSS_RANGE:RangeInclusive<Num> = 200000000000000..=400000000000000;


#[derive(Debug)]
struct Vec3{x:Num,y:Num,z:Num}

impl std::ops::Add for Vec3{
    type Output = Self;
    fn add(self, other:Self) -> Self{
        Self{
            x: self.x+other.x,
            y: self.y+other.y,
            z: self.z+other.z,
        }
    }
}
impl std::ops::Sub for Vec3{
    type Output = Self;
    fn sub(self, other:Self) -> Self{
        Self{
            x: self.x-other.x,
            y: self.y-other.y,
            z: self.z-other.z,
        }
    }
}

fn main() {
    let mut hailstones = Vec::new();
    for line in read_to_string("p.in").unwrap().lines() {
        let (pos,vel) = line.split_once('@').unwrap();
        let pos:Vec<Num> = pos.splitn(3, ", ").map(|x| x.trim().parse().unwrap()).collect();
        let vel:Vec<Num> = vel.splitn(3, ", ").map(|x| x.trim().parse().unwrap()).collect();
        
        hailstones.push((
            Vec3{x:pos[0], y:pos[1], z:pos[2]},
            Vec3{x:pos[0]+vel[0], y: pos[1]+vel[1], z:pos[2]+vel[2]},
        ));
    }

    let mut ans = 0;
    for i in 1..hailstones.len(){
        let s1 = &hailstones[i];
        for j in 0..i{
            let s2 = &hailstones[j];

            let div = (s1.0.x-s1.1.x)*(s2.0.y-s2.1.y) - (s1.0.y-s1.1.y)*(s2.0.x-s2.1.x);

            if div == 0{
                continue;
            }
            
            let px = ((s1.0.x*s1.1.y - s1.0.y*s1.1.x)*(s2.0.x-s2.1.x)
                        - (s1.0.x-s1.1.x)*(s2.0.x*s2.1.y - s2.0.y*s2.1.x))/div;
            let py = ((s1.0.x*s1.1.y - s1.0.y*s1.1.x)*(s2.0.y-s2.1.y)
                        - (s1.0.y-s1.1.y)*(s2.0.x*s2.1.y - s2.0.y*s2.1.x))/div;

            // if p 
            if (((px-s1.0.x)>0) ^ ((s1.1.x-s1.0.x)>0)) 
            || (((px-s2.0.x)>0) ^ ((s2.1.x-s2.0.x)>0))
            || (((py-s1.0.y)>0) ^ ((s1.1.y-s1.0.y)>0)) 
            || (((py-s2.0.y)>0) ^ ((s2.1.y-s2.0.y)>0))
            {
                continue;
            }  
            
            if CROSS_RANGE.contains(&px) && CROSS_RANGE.contains(&py){
                ans += 1;
                println!("{:?}\n{:?}\n", s1, s2);
            }
        }
    }
    
    println!("{}", ans);
}
