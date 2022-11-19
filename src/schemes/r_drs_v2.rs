use rand::{seq::SliceRandom,thread_rng, Rng};
use random_choice::random_choice;

pub struct Rdrs2 {
    pub sk: Vec<Vec<i128>>,
    pub pk: Vec<Vec<i128>>,
    pub d:i128,
}

impl Rdrs2 {
    pub fn new(n:usize, zn:usize,d:i128,rounds:usize) -> Rdrs2{
        let keypair:(Vec<Vec<i128>>,Vec<Vec<i128>>) = Rdrs2::keygen(n,zn,d,rounds);
        return Rdrs2 {
            sk: keypair.0,
            pk: keypair.1,
            d:  d,
        }
    }

    fn kraemerbits(n:usize, zn:usize, d:i128) -> Vec<i128>{
        let mut v:Vec<i128> = vec![0;n];
        let mut x:Vec<i128> = vec![0;n-zn+1];
        v[0] = d;
        for i in 0..n {
            x[i] = rand::thread_rng().gen_range(0..n) as i128;
        }
        x.sort();
        for i in 1..(n-zn) {
            v[i] = &x[i] - &x[i+1];
        }
        for i in (n-zn+1)..n {
            v[i] = 0;
        }
        v
    }

    fn keygen(n:usize,zn:usize,d:i128,rounds:usize) -> (Vec<Vec<i128>>, Vec<Vec<i128>>) {
        let mut sk: Vec<Vec<i128>> = vec![vec![0;n];n];
        let mut pk: Vec<Vec<i128>> = vec![vec![0;n];n];
        let choices = vec![-1,1];
        let weights : [f64;2] = [0.5,0.5];

        // Secret key
        for i in 0..n {
            let t = Self::kraemerbits(n,zn,d);
            sk[i][i] = t[0].clone();
            for j in 1..n {
                let sgn = random_choice().random_choice_f64(&choices, &weights, 1)[0];
                let c = t[j] * sgn;
                sk[i][(i+j)%n] = c;
            }
        }

        // Public key
        pk = sk.clone();
        let mut y:Vec<usize> = vec![0;n];
        for i in 0..n {
            y[i] = 1;
        }
        y.shuffle(&mut thread_rng());
        for _ in 0..rounds {
            for j in 0..n {
                pk.swap(j,y[j]);
            }

            let sgn = random_choice().random_choice_f64(&choices, &weights, 1)[0];
            for j in (0..n-1).step_by(2) {
                for k in 0..n {
                    pk[j][k] = pk[j][k].clone() + sgn * pk[j][k].clone();
                }
                for k in 0..n {
                    pk[j+1][k] = pk[j+1][k].clone() + sgn * pk[j][k].clone();
                }
            }
        }
        for j in 0..n {
            pk.swap(j, y[j]);
        }
        (sk,pk)
    }

    pub fn sign(&self,n:usize,m:&Vec<i128>,d:i128)-> Vec<i128> {
        let mut w = vec![0;n];
        for i in 0..n {
            w[i] = m[i];
        }
        let mut k:Vec<i128> = vec![0;n];
        let mut i: usize = rand::thread_rng().gen_range(0..n);
        let mut wmax:i128 = w.iter().max_by_key(|x| x.clone().abs()).unwrap().clone().abs();

        while wmax >= d {
            let q = w[i].clone() / d;
            k[i] = k[i].clone() + q;
            for j in 0..n {
                w[j] = w[j].clone() - q * self.sk[i][j];
            }
            i = (i+1) % n;
            wmax = w.iter().max_by_key(|x| x.clone().abs()).unwrap().clone().abs();
        }
        w
    }

}