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
    pub estado_atual: usize,
    pub char_leitura: char,
    pub estado_destino: usize,
    pub char_escrita: char,
    pub direcao: Direcao,
}

impl Transicao {
    pub fn new(
        estado_atual: usize,
        char_leitura: char,
        estado_destino: usize,
        char_escrita: char,
        dir: Direcao,
    ) -> Self {
        Transicao {
            estado_atual: estado_atual,
            char_leitura: char_leitura,
            estado_destino: estado_destino,
            char_escrita: char_escrita,
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

#[derive(Debug)]
pub struct Maquina {
    pub fita: Vec<char>,
    pub pos: usize,
    pub estado_atual: usize,
    pub transicoes: HashMap<(usize, char), Transicao>,
    pub qtd_estados: usize,
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
            estado_atual: 0,
            transicoes: HashMap::new(),
        }
    }

    pub fn passo_simulacao(&mut self) {
        let simbolo: char = self.fita[self.pos];
        let key: (usize, char) = (self.estado_atual, simbolo);

        match self.transicoes.get(&key) {
            Some(transicao) => {
                println!("Transição encontrada com sucesso: {}", transicao);
                self.estado_atual = transicao.estado_destino;
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
