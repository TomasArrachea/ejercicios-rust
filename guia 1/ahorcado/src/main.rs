use std::io;
use std::io::Read;
use std::fs::File;

static VIDAS: u8 = 3;

enum Error { //USAR ENUM EN EL CODIGO, consigna c.
	intentos_agotados,
	letra_erronea(char),
}

fn main() {
    println!("¡Bienvenido al ahorcado!\n\n");
	
	let mut intentos_restantes = VIDAS;
	let palabra = generar_palabra();
	let mut palabra_descubierta = inicializar_descubierta(palabra.len());
	let mut errores = vec![];

    while palabra != palabra_descubierta && intentos_restantes > 0 {
    	imprimir_descubierta(&palabra_descubierta);
		println!("Ingresa una letra: ");

		let guess = arriesgar();

		let mut esta = false;
		for (i, letra) in palabra.chars().enumerate(){
			if guess == letra {
	    		palabra_descubierta.remove(i);
	    		palabra_descubierta.insert(i, letra);
	    		esta = true;
			}
   		}
   	 	if esta == false {
   	 		intentos_restantes = intentos_restantes - 1;
   	 		errores.push(guess);
    	}
   		imprimir_intentos(&intentos_restantes);
   		imprimir_errores(&errores);
	}
}

fn imprimir_errores(errores: &Vec<char>){
	print!("Ya intentaste: ");
	for i in errores {
		print!("{} ", i);
	}
}
fn generar_palabra() -> String {
    let mut palabra = String::new();
    File::open("palabras.txt").expect("Failed to open palabras.txt").read_to_string(&mut palabra).unwrap();
    palabra
}

fn imprimir_descubierta(palabra_descubierta: &String){
	print!("\n\n\nLa palabra hasta el momento es: ");
	for letra in palabra_descubierta.chars() {
		print!("{} ",letra);
	}
	println!("");
}

fn imprimir_intentos(intentos: &u8){
   	println!("Le quedan {:?} intentos", intentos);
}

fn arriesgar() -> char {
	let mut guess = String::new();
	io::stdin().read_line(&mut guess).expect("Failed to read line");

	let guess: char = guess.trim().parse().expect("Se debe ingresar solo una letra");
	guess
}

fn inicializar_descubierta(len: usize) -> String {
	let mut palabra_descubierta = String::new();
	for _ in 0..len {
		palabra_descubierta.push('_'); 
	}
	palabra_descubierta
}