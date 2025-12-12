use std::collections::HashMap;
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
    pub estado_atual: String,
    pub char_leitura: char,
    pub estado_destino: String,
    pub char_escrita: char,
    pub direcao: Direcao,
}

impl Transicao {
    pub fn new(ea: String, cl: char, ed: String, ce: char, dir: Direcao) -> Self {
        Transicao {
            estado_atual: ea,
            char_leitura: cl,
            estado_destino: ed,
            char_escrita: ce,
            direcao: dir,
        }
    }
}

impl fmt::Display for Transicao {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}) -> ({}, {}, {})",
            self.estado_atual,
            self.char_leitura,
            self.estado_destino,
            self.char_escrita,
            self.direcao
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
        estado_atual: palavras[0].to_string(),
        char_leitura: palavras[1].parse().expect("Símbolo de leitura inválido"),
        estado_destino: palavras[2].to_string(),
        char_escrita: palavras[3].parse().expect("Símbolo de escrita inválido"),
        direcao: direcao,
    };

    println!("{}", t);
}

#[derive(Debug)]
pub struct Maquina {
    pub fita: Vec<char>,
    pub pos: usize,
    pub estado_atual: String,
    pub transicoes: HashMap<(String, char), Transicao>,
}

impl fmt::Display for Maquina {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Posição atual: {}\nFita: \n\n", self.pos);

        for char in &self.fita {
            // Equivalente a colocar '?' depois da função
            match write!(f, "{}", char) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

impl Maquina {
    pub fn new(fita: &str) -> Self {
        Maquina {
            fita: fita.chars().collect(),
            pos: 0,
            estado_atual: String::from("q0"),
            transicoes: HashMap::new(),
        }
    }

    pub fn passo_simulacao(&mut self) {
        let simbolo: char = self.fita[self.pos];
        let key: (String, char) = (self.estado_atual.clone(), simbolo);

        match self.transicoes.get(&key) {
            Some(transicao) => {
                println!("Transição encontrada com sucesso: {}", transicao);
                self.estado_atual = transicao.estado_destino.clone();
                self.fita[self.pos] = transicao.char_escrita.clone();

                match transicao.direcao {
                    Direcao::D => self.pos += 1,
                    Direcao::E => {
                        if self.pos > 0 {
                            self.pos -= 1;
                        } else {
                            panic!("Erro: não é possível ir para esquerda, limite atingido.")
                        }
                    }
                }

                println!("{}", self); // self é uma instância de máquina
            }
            None => {
                panic!(
                    "Erro: nenhuma transição encontrada para: ({}, {})",
                    key.0, key.1
                )
            }
        }
    }
}
