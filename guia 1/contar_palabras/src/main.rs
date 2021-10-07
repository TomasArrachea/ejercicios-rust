use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

static ARCHIVO : &str = "texto.txt";

fn main() {
    let palabras = leer_archivo(ARCHIVO);

    let mut palabras = Vec::from_iter(palabras.iter());
    palabras.sort_by_key(|n| n.1);
    palabras.reverse();

    for (palabra, i) in palabras {
        imprimir_palabra(palabra, i);
    }
}

fn leer_archivo(archivo: &str) -> HashMap<String, i32> {
    let file = File::open(archivo).unwrap();
    let reader = BufReader::new(file);
    let mut palabras: HashMap<String, i32> = HashMap::new();

    for line in reader.lines() {
        cargar_palabras(line.unwrap(), &mut palabras);
    }
    palabras
}

fn cargar_palabras(mut line: String, palabras: &mut HashMap<String, i32>) {
    line.make_ascii_lowercase();

    for p in line.split_whitespace() {
        if let Some(v) = palabras.get_mut(p) {
            *v += 1;
        } else {
            palabras.insert(String::from(p), 1);
        }
    }
}

fn imprimir_palabra(palabra: &String, i: &i32) {
    println!("{} -> {}", palabra, i);
}
