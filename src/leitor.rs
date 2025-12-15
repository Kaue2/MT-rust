use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::maquina::{Direcao, Maquina, Transicao};

fn pegar_estados() {}

fn pegar_q_aceita() {}

fn pegar_transicoes() {}

fn pegar_fita() {}

fn pegar_resultado() {}

pub fn montar_maquina(path: &str) -> std::io::Result<()> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        let bytes_lidos = reader.read_line(&mut buffer)?;
        if bytes_lidos == 0 {
            break;
        }

        let linha = buffer.trim();
        if linha.is_empty() {
            continue;
        }

        match linha {
            "estados:" => {
                pegar_estados();
            }
            "q_aceita:" => {
                pegar_q_aceita();
            }
            "transicoes:" => {
                pegar_transicoes();
            }
            "fita_inicial:" => pegar_fita(),
            "esperado:" => {
                pegar_resultado();
            }
            _ => {}
        }
    }

    Ok(())
}
