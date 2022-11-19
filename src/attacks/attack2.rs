use rand::Rng;
use crate::{schemes::r_drs_v2::Rdrs2, lattice_reduction::lll::lll};
use rayon::prelude::*;
use random_choice::random_choice;

pub fn attack2(n:usize,drs:&Rdrs2) -> Vec<Vec<i128>> {
    let mut llmatrix:Vec<Vec<i128>> = vec![vec![0;n];n-1];
    let mut number_of_loops:usize = 0;
    let mut ll_size:usize = llmatrix.len();
    let mut n_of_d:usize = 0;
    let choices:Vec<i128> = vec![-1,1];
    let weights:[f64;2] = [0.5,0.5];
    const TWO:i128 = 2;
    let delta:i128 = TWO.pow(28);
    let mut total_n_of_sig:usize  = 0;  

    while ll_size < n || n_of_d != n {
        number_of_loops+=1;
        let mut message:Vec<i128> = vec![0;n];
        for i in 0..n {
            let sgn = random_choice().random_choice_f64(&choices, &weights, 1)[0];
            message[i] = rand::thread_rng().gen_range(0..delta) * sgn;
        }
        let w0:Vec<i128> = drs.sign(n, &message, drs.d);

        let custom_number:usize = 1;// custom number of signatures per message(1...log(n))

        let mut mtx:Vec<Vec<i128>> = vec![vec![0;n];0];
        for i in 0..custom_number{
            let mut wmatrix:Vec<Vec<i128>> = vec![vec![0;n];n];
            wmatrix.par_iter_mut().for_each(|wrow| {
                let sig = drs.sign(n, &message, drs.d);
                *wrow = sig;
            });
            wmatrix.sort();
            wmatrix.dedup();
            wmatrix.iter().for_each(|sig| mtx.push(sig.to_vec()));
        }

        mtx.sort();
        mtx.dedup();

        

        for wi in &mtx {
            let mut diff:Vec<i128> = vec![0;n];
            for i in 0..n {
                diff[i] = &w0[i] - &wi[i];
            }
            llmatrix.push(diff);
        }

        llmatrix = lll(&llmatrix);


        n_of_d = 0;
        ll_size = llmatrix.len();
        if ll_size >= n {
            for i in 0..llmatrix.len() {
                if &llmatrix[i][i].abs() == &drs.d {
                    let mut sum_of_off_diag:i128 = - drs.d;
                for j in 0..n {
                    sum_of_off_diag += &llmatrix[i][j].abs();
                }
                if sum_of_off_diag >= drs.d {
                    n_of_d+=1;
                    println!("{:?}",llmatrix[i]);
                }
                }
            }
        }

        println!(
            "\n+++++++++ {} +++++++++ ({}  /  {})",
            number_of_loops,
            llmatrix.len(),
            n
        );

        ll_size = llmatrix.len();
        total_n_of_sig += mtx.len();
        println!("total number of different signatures collected:  {:?}  (number of rows in LL before reduction: {})  {}",total_n_of_sig,llmatrix.len(),n_of_d);

    }

    


    llmatrix
}