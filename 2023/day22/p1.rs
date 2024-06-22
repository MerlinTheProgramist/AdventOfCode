use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::collections::HashSet;

const N:usize = 350;
const BLOCKS:usize = 1400;

type Block = (Vec<usize>, Vec<usize>);

fn main() {
    let mut blocks = Vec::new();
    for line in read_to_string("p.in").unwrap().lines(){
        let (pos1, pos2) = line.split_once('~').unwrap();
        let pos1:Vec<_> = pos1.splitn(3,',').map(|x| x.parse::<usize>().unwrap()).collect();
        let pos2:Vec<_> = pos2.splitn(3,',').map(|x| x.parse::<usize>().unwrap()).collect();
        
        blocks.push((pos1, pos2));
    }
    blocks.sort_by(|a,b| a.0[2].cmp(&b.0[2]));
    // println!("{:?}", inputs.iter().map(|(a,b)| a[2].min(b[2])).collect::<Vec<_>>());
    
    for index in 0..blocks.len() {
        let block = &blocks[index];
        let mut max_z = 1;
        for check in blocks[..index].iter(){
            if intersects(block, check){
                max_z = max_z.max(check.1[2] + 1)
            }
        } 
        let block = &mut blocks[index];
        block.1[2] -= block.0[2] - max_z;
        block.0[2] = max_z;
    }
    blocks.sort_by(|a,b| a.0[2].cmp(&b.0[2]));
    
    println!("{:?}", blocks);

    let mut k_supports_v = vec![HashSet::<usize>::new(); blocks.len()];
    let mut v_supports_k = vec![HashSet::<usize>::new(); blocks.len()];

    for (j,upper) in blocks.iter().enumerate(){
        for (i, lower) in blocks[..j].iter().enumerate(){
            if intersects(upper,lower) &&  upper.0[2] == lower.1[2] + 1{
                k_supports_v[i].insert(j);
                v_supports_k[j].insert(i);
            } 
        } 
    }    

    let mut total = 0;
    // check if every block that it supports is also supported by other block
    for i in 0..blocks.len(){
        if k_supports_v[i].iter().all(|&j| v_supports_k[j].len()>1){
            total += 1;
        }
    }
    println!("{}", total);
}

#[inline(always)]
fn intersects(a:&Block, b:&Block) -> bool{
    a.0[0].max(b.0[0]) <= a.1[0].min(b.1[0]) && 
    a.0[1].max(b.0[1]) <= a.1[1].min(b.1[1])
}
