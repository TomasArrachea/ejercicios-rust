use std::fmt;
#[derive(Debug)]
pub struct Token {
	frecuencia: i32,
	ids: Vec<String>,
}

impl Token {
	pub fn new(frecuencia: i32, id: String) -> Token {
		Token {
			frecuencia,
			ids: vec![id],
		}
	}
	pub fn agregar_id(&mut self, id: String) {
		self.ids.push(id);
	}

	pub fn sumar_frecuencia(&mut self) {
		self.frecuencia += 1;
	}
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.ids)
	}
}