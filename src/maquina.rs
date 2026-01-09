use std::collections::HashMap;
use std::fmt;

struct TransicaoErro;
pub struct MaquinaErro;

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
    pub mapa_nomes: HashMap<String, usize>,
    pub estados_finais: Vec<bool>,
    pub resultado: Vec<char>,
}

impl fmt::Display for Maquina {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fita_str: String = self.fita.iter().collect();
        let res_str: String = self.resultado.iter().collect();

        write!(
            f,
            "Posição atual: {}\nFita: {}\nResultado esperado: {}\nEstado atual: {}\n\n",
            self.pos, fita_str, res_str, self.estado_atual
        )
    }
}

impl Maquina {
    pub fn new(fita: &str) -> Self {
        Maquina {
            fita: fita.chars().collect(),
            pos: 0,
            estado_atual: 0,
            transicoes: HashMap::new(),
            qtd_estados: 0,
            mapa_nomes: HashMap::new(),
            estados_finais: Vec::new(),
            resultado: Vec::new(),
        }
    }

    fn passo_simulacao(&mut self) -> Result<bool, TransicaoErro> {
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
                            eprintln!("Erro: não é possível ir para esquerda, limite atingido.");
                            return Err(TransicaoErro);
                        }
                    }
                }

                println!("{}", self); // self é uma instância de máquina
                return Ok(false);
            }
            None => {
                eprintln!(
                    "Aviso: encerrando máquina nenhuma transição encontrada para: ({}, {})",
                    key.0, key.1
                );
                return Ok(true);
            }
        }
    }

    /* *
     * função para simular uma máquina
     * retorna sucesso se a máquina parar
     */
    pub fn simular_maquina(&mut self) -> Result<bool, MaquinaErro> {
        loop {
            match self.passo_simulacao() {
                Ok(true) => {
                    if self.estados_finais[self.estado_atual] == true {
                        println!(
                            "Simulação encerrada! \nDecisão da máquina: aceito \n{}",
                            self
                        );
                        return Ok(true);
                    } else {
                        println!(
                            "Simulação encerrada! \nDecisão da máquina: rejeito \n{}",
                            self
                        )
                    }
                }
                Ok(false) => {
                    continue;
                }
                Err(TransicaoErro) => {
                    return Err(MaquinaErro);
                }
            }
        }
    }
}
