use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::maquina::{Direcao, Maquina, Transicao};

fn pegar_estados(path: &str) {}
fn pegar_q_aceita(path: &str) {}
fn pegar_transicoes(path: &str) {}

pub fn pegar_fita(path: &str) -> std::io::Result<()> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    // tenta ler o buf
    // leu? ok
    // tem alguma coisa? então continua no loop
    while reader.read_line(&mut buffer)? > 0 {
        println!("conteúdo encontrado {}", buffer);

        buffer.clear();
    }

    Ok(())
}

fn pegar_resultado(path: &str) {}

pub fn montar_maquina(path: &str) {}
