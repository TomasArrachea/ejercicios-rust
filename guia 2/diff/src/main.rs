use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let lineas_1 = read_file_lines("1.txt");
	let lineas_2 = read_file_lines("2.txt");
	let grilla = lcs(&lineas_1, &lineas_2);
	print_diff(&grilla, lineas_1.len(), lineas_2.len(), &lineas_1, &lineas_2);
}

fn read_file_lines(ruta: &str) -> Vec<String> {
	let mut lineas = Vec::new();
	let file = File::open(ruta).expect("No se pudo abrir el archivo");
    let reader = BufReader::new(file);	    
    for line in reader.lines(){
    	let l = line.unwrap();
    	lineas.push(l);
	}
	lineas
}

//holiiiiiii te quiero
//Longest Common Subsequence
fn lcs(lineas_1: &[String] ,lineas_2: &[String]) -> Vec<Vec<i32>>{
	let m = lineas_1.len();
	let n = lineas_2.len();
	let mut grilla = vec![vec![0 ; n+1]; m+1]; //no hace falta inicializar todo en 0, solo la primer fila y columna
	for i in 0..m {
	    for j in 0..n {
	        if lineas_1[i] == lineas_2[j]{
	            grilla[i+1][j+1] = grilla[i][j] + 1;
	        } else {
	            grilla[i+1][j+1] = max(grilla[i+1][j], grilla[i][j+1]);
	        }
	    }
	}
	grilla
}

fn print_diff(grilla: &[Vec<i32>], i: usize, j: usize, x: &[String] , y: &[String]){
    if i > 0 && j > 0 && x[i-1] == y[j-1] {
        print_diff(grilla, i-1, j-1, x, y);
        println!("  {}", x[i-1]);
    }
    else if j > 0 && (i == 0 || grilla[i][j-1] >= grilla[i-1][j]) {
        print_diff(grilla, i, j-1, x, y);
        println!("+ {}", y[j-1]);
    }
    else if i > 0 && (j == 0 || grilla[i][j-1] < grilla[i-1][j]) {
        print_diff(grilla, i-1, j, x, y);
        println!("- {}", x[i-1]);
    }
    else {
        println!();
    }

}