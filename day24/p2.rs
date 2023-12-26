use std::fs::read_to_string;
use std::iter::zip;
use z3::ast::{Array, Ast, Real, Bool, Int, BV};
use z3::{Config,Context,Solver};


fn main() {
    let mut hailstones = Vec::new();
    for line in read_to_string("p.in").unwrap().lines() {
        let (pos,vel) = line.split_once('@').unwrap();
        let pos:Vec<i64> = pos.splitn(3, ", ").map(|x| x.trim().parse().unwrap()).collect();
        let vel:Vec<i64> = vel.splitn(3, ", ").map(|x| x.trim().parse().unwrap()).collect();
        
        hailstones.push((pos,vel));
    }

    solve(&hailstones);
}


fn solve(rays:&Vec<(Vec<i64>, Vec<i64>)>){
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // literals
    let zero = Int::from_u64(&ctx, 0);

    // constants
    let rpx = Int::new_const(&ctx, "rpx"); 
    let rpy = Int::new_const(&ctx, "rpy"); 
    let rpz = Int::new_const(&ctx, "rpz"); 

    let rvx = Int::new_const(&ctx, "rvx"); 
    let rvy = Int::new_const(&ctx, "rvy"); 
    let rvz = Int::new_const(&ctx, "rvz"); 

    let slv = Solver::new(&ctx);

    for (pos,dir) in rays{
        // literals

        // constants
        let t = Int::fresh_const(&ctx, "t");
        slv.assert(&t.ge(&zero));

        for ((p,v),(rp,rv)) in zip(zip(pos,dir), zip([&rpx, &rpy, &rpz],[&rvx,&rvy,&rvz])){
            let p = Int::from_i64(&ctx, *p);
            let v = Int::from_i64(&ctx, *v);
            slv.assert(&(&p+&v*&t-rv*&t)._eq(rp));
            // slv.assert(
            //     &Int::sub(&ctx,
            //         &[&Int::add(&ctx, 
            //            &[&p, 
            //              &Int::mul(&ctx, &[&v,&t]),
            //             ]),
            //           &Int::mul(&ctx, &[&rv,&t])
            //          ])
            //         ._eq(&rp));
        }
    }
    println!("{:?}",slv.check()); // this is neded
    for ass in slv.get_assertions(){
        println!("{:?}", ass);
    }

    let model = slv.get_model().expect("No sollution found, Sorry :( (Don't tell Santa)");
    let px = model.get_const_interp(&rpx).unwrap().as_i64().unwrap();
    let py = model.get_const_interp(&rpy).unwrap().as_i64().unwrap();
    let pz = model.get_const_interp(&rpz).unwrap().as_i64().unwrap();

    println!("{}", px+py+pz);
}
