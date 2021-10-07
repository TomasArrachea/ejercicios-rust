use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::thread;
use std::sync::{Arc, Mutex};


static DIRECTORIO: &str = "textos";

fn main() {
    let palabras = leer_archivos(DIRECTORIO);
    let palabras = palabras.lock().unwrap();
    let mut palabras = Vec::from_iter(palabras.iter());
    
    palabras.sort_by_key(|n| n.1);
    palabras.reverse();

    for (palabra, i) in palabras {
        imprimir_palabra(palabra, i);
    }
}

fn leer_archivos(directorio: &str) -> Arc<Mutex<HashMap<String, i32>>> {
    let mut handles = Vec::new();
    let ptr_palabras = Arc::new(Mutex::new(HashMap::new()));

    for path in fs::read_dir(directorio).unwrap() {
        let mutex_clone = Arc::clone(&ptr_palabras);

        handles.push(thread::spawn(move || {
            let file = fs::File::open(path.unwrap().path()).unwrap();
            let reader = BufReader::new(file);

            for line in reader.lines() {
                cargar_palabras(line.unwrap(), &mut mutex_clone.lock().unwrap());
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap(); 
    }
    ptr_palabras
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

fn imprimir_palabra(palabra: &str, i: &i32) {
    println!("{} -> {}", palabra, i);
}
