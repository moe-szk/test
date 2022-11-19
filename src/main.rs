mod attacks;
use std::time::Instant;

use attacks::attack2;

mod schemes;
use schemes::r_drs_v2::{self, Rdrs2};

mod lattice_reduction;
use lattice_reduction::{lll,bkz};

use crate::attacks::attack2::attack2;


fn main() {
    let param:(usize,usize,i128,usize) = (20,1,20,2);
    let drs = Rdrs2::new(param.0,param.1,param.2,param.3);

    println!("Params   n:{:?},zn:{:?},d:{:?}\n",param.0,param.1,param.2);
    let start_time = Instant::now();
    let mut recovered_mtx = attack2(param.0, &drs);
    let elapsed_time = start_time.elapsed();

    println!("Reslt");
    for row in recovered_mtx {
        println!("{:?}",row);
    }

    println!("Execution time: {:?}",elapsed_time);
}
