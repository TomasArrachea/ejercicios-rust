use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use crate::token::Token;

mod token;


fn main() {
	let palabras = leer_datos();
	let mut palabra_buscada = String::new();

	while palabra_buscada != "x" {
		println!("\nIngrese la palabra a buscar ('x' para salir): ");
		io::stdin().read_line(&mut palabra_buscada).expect("Failed to read line");

		match palabras.get(palabra_buscada.trim()) {
			Some(token) => {
				println!("\tLa palabra aparece en {}", token); 
			}
			None => {
				println!("No se encontró la palabra :(");
			}
		};
	}
	//falta tokenizar el string ingresado, buscar cada string de la frase y ordenar por importancia los resultados.
}

fn leer_datos() -> HashMap<String, Token> {
    let mut palabras = HashMap::new();
	let extension = ".txt";
	let mut i = 1;
	let mut nombre = i.to_string() + extension;

	let stop_words = cargar_stop_words();

	while let Ok(file) = File::open(&nombre) { //para cada documento
	    let reader = BufReader::new(file);	    

	    for line in reader.lines(){ //para cada linea del documento
        	cargar_tokens(line.unwrap(), &mut palabras, &nombre, &stop_words);
  		}
	    i += 1;
		nombre = i.to_string() + extension;
	}
	palabras
}

fn cargar_tokens(mut line: String, palabras: &mut HashMap<String, Token>, id: &str, stop_words: &[String]) {
	line.make_ascii_lowercase();

	for palabra in line.split_whitespace() { //para cada palabra en la linea
		if !es_stopword(palabra, stop_words){
	        if let Some(t) = palabras.get_mut(palabra) {
	            t.sumar_frecuencia();
	            t.agregar_id(id.to_string());
	        } else {
	    		palabras.insert(palabra.to_string(), Token::new(1, id.to_string()));
			}
		}
	}
}

fn cargar_stop_words() -> Vec<String> {
	let mut stop_words = Vec::new();
	let file_sw = File::open("stop_words.txt").expect("no se pudo abrir el archivo: stop_words.txt");
    let reader = BufReader::new(file_sw);	    
    for line in reader.lines(){ //para cada linea del documento
    	for p in line.unwrap().split_whitespace() { //para cada palabra en la linea
			stop_words.push(p.to_string())
		}
	}
	stop_words
}

fn es_stopword(palabra: &str, stop_words: &[String]) -> bool{
    let mut es_sw = false;
	for p in stop_words {
		if palabra == p {
			es_sw = true; 
		}
	}
	es_sw
}
