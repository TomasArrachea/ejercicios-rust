use std::io;

const VIDAS: u8 = 3;

fn main() {
	let mut intentos_restantes = VIDAS;
	let palabra = "elegante";//generar_palabra();
	let mut palabra_descubierta = String::new();
	let mut guess = String::new();

    println!("Bienvenido al ahorcado de FIUBA!\n\n");

    let mut i = 0;
	palabra_descubierta.push('_'); 
	while palabra.len() > i  { //INICIALIZAR PALABRA_DESCUBIERTA 
		palabra_descubierta.push('_'); 
		i +=1;
	}

    while palabra != palabra_descubierta {
		println!("\nLa palabra hasta el momento es: {:}", palabra_descubierta);

		println!("Ingresa una letra:");
		io::stdin() 							//ARRIESGAR
		    .read_line(&mut guess)
		    .expect("Failed to read line");

		let guess: char = match guess.trim().parse(){
	        Ok(num) => num,
	        Err(_) => {
	        	println!("Error. Ingresa una letra!");
	        	continue;
	        }
		};

		for (i, letra) in palabra.chars().enumerate(){
	    	if guess == letra {
	    		palabra_descubierta.insert(i, letra);
	    	} else {
	    		intentos_restantes -= 1;
	    	}
   		}
   		 
   		println!("Le quedan {:?} intentos", intentos_restantes);
	}
}

// fn generar_palabra() -> &'static str {
// 	"elefante" //LEER DESDE UN ARCHIVO TXT
// }

// fn imprimir_incognitas(palabra: &str){
// }

// fn imprimir_intentos(intentos: u8){
// }

// fn arriesgar(guess) {}
// fn llenar_guiones() {}