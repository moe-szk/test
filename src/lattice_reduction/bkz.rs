use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::process::{Command};

pub fn bkz(mtx: &Vec<Vec<i128>>) -> Vec<Vec<i128>> {
    let n = mtx[0].len();
    let m = mtx.len();

    // commands
    let mut lll = Command::new("./bkz");

    // clear input_mtx.txt and output_mtx.txt
    Command::new("cp").args(["empty.txt","input_mtx.txt"]).status().expect("Error: clear input_mtx.txt");
    Command::new("cp").args(["empty.txt","output_mtx.txt"]).status().expect("Error: clear output_mtx.txt");
    
    // make input matrix file
    let mtx_str:String = mtx_to_mtx_str(mtx);
    let mut inputfile = File::create("input_mtx.txt").expect("Error: Open input_mtx.txt");
    inputfile.write_all(mtx_str.as_bytes()).expect("Error: Write input_mtx.txt");

    // lll command
    lll.spawn().unwrap().wait().expect("Error in lll algorithm");

    // output_mtx.txt -> mtx
    let mut reduced_mtx = output_to_mtx(n, m);
    remove_zero_vector(&mut reduced_mtx);

    // output
    reduced_mtx
}

fn output_to_mtx(n: usize, m: usize) -> Vec<Vec<i128>> {
    // let mtx_str = std::str::from_utf8(&output.stdout).unwrap();
    // let mut mtx: Vec<Vec<i128>> = vec![vec![i128::from(0); n]; m];

    // open the file
    let mut f = BufReader::new(File::open("output_mtx.txt").unwrap());

    let mut s = String::new();

    let mtx: Vec<Vec<i128>> = f.lines()
        .map(|l|{ 
            l.unwrap().replace("[", "").replace("]", "").split_whitespace()
             .map(|number| number.parse::<i128>().unwrap() )
             .collect()
            })
        .collect();

    mtx
}

fn mtx_to_mtx_str(mtx: &Vec<Vec<i128>>) -> String {
    let n = mtx[0].len();
    let m = mtx.len();
    let mut mtx_str: String = format!("{}  {}\n",m,n);
    for i in 0..m {
        for j in 0..n {
            let elem_str = mtx[i][j].to_string();
            mtx_str.push_str(&elem_str);
            mtx_str.push(' ');
        }
        mtx_str.push('\n');
    }
    mtx_str
}

fn remove_zero_vector(mtx: &mut Vec<Vec<i128>>) {
    let mut remove_list: Vec<usize> = vec![0; 0];
    let n = mtx[0].len();
    let m = mtx.len();
    let zero_vec: Vec<i128> = vec![0; n];

    for i in 0..m {
        if mtx[i]
            .iter()
            .zip(&zero_vec)
            .filter(|&(a, b)| a == b)
            .count()
            == n || mtx[i].len() == 0
        {
            // if mtx[i] == zero_vec {
            remove_list.push(i);
        }
    }
    let mut n_of_removes = 0;
    for index in remove_list {
        mtx.remove(index - n_of_removes);
        n_of_removes += 1;
    }
}
