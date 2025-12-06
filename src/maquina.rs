use std::fmt;

#[derive(Debug)]
pub enum Direcao {
    E,
    D,
}

impl fmt::Display for Direcao {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let texto = match self {
            Direcao::E => "Esquerda",
            Direcao::D => "Direita",
        };

        write!(f, "{}", texto)
    }
}

#[derive(Debug)]
pub struct Transicao {
    origem: String,
    entrada: String,
    destino: String,
    saida: String,
    direcao: Direcao,
}

impl fmt::Display for Transicao {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}) -> ({}, {}, {})",
            self.origem, self.entrada, self.destino, self.saida, self.direcao
        )
    }
}

pub fn get_transition(entrada: &str) {
    let palavras: Vec<&str> = entrada.split_whitespace().collect();

    let direcao = match palavras[4] {
        "E" => Direcao::E,
        "D" => Direcao::D,
        _ => panic!("Direção desconhecida {}", palavras[4]),
    };

    let t = Transicao {
        origem: palavras[0].to_string(),
        entrada: palavras[1].to_string(),
        destino: palavras[2].to_string(),
        saida: palavras[3].to_string(),
        direcao: direcao,
    };

    println!("{}", t);
}

#[derive(Debug)]
pub struct Maquina {
    pub fita: String,
    pub pos: u32,
}
