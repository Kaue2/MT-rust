use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use crate::maquina::{Direcao, Maquina, Transicao};

fn get_estados(reader: &mut BufReader<File>) -> io::Result<Vec<String>> {
    let mut buffer = String::new();
    let mut estados = Vec::new();

    loop {
        buffer.clear();
        let bytes_lidos = reader.read_line(&mut buffer)?;
        if bytes_lidos == 0 {
            break;
        }

        let linha = buffer.trim();
        if linha.is_empty() {
            break;
        }

        for parte in linha.split(',') {
            let content = parte.trim();

            if !content.is_empty() {
                println!("Estado encontrado: {}", content);
                estados.push(content.to_string());
            }
        }
    }

    Ok(estados)
}

fn atribuir_estados(maquina: &mut Maquina, reader: &mut BufReader<File>) -> io::Result<()> {
    let estados = get_estados(reader)?;

    for e in estados {
        maquina.mapa_nomes.insert(e, maquina.qtd_estados);
        maquina.qtd_estados += 1;
    }

    Ok(())
}

fn get_estados_finais(reader: &mut BufReader<File>) -> io::Result<Vec<String>> {
    let mut buffer = String::new();
    let mut estados = Vec::new();

    loop {
        buffer.clear();

        let bytes_lidos = reader.read_line(&mut buffer)?;
        if bytes_lidos == 0 {
            break;
        }

        let linha = buffer.trim();
        if linha.is_empty() {
            break;
        }

        for parte in linha.split(',') {
            let content = parte.trim();
            if !content.is_empty() {
                estados.push(content.to_string());
            }
        }
    }

    Ok(estados)
}

fn atribuir_q_aceita(maquina: &mut Maquina, reader: &mut BufReader<File>) -> io::Result<()> {
    maquina.estados_finais.resize(maquina.qtd_estados, false);
    let estados = get_estados_finais(reader)?;

    for e in estados {
        match maquina.mapa_nomes.get(&e) {
            Some(&valor) => {
                maquina.estados_finais[valor] = true;
            }
            None => {
                println!("{}", e);
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Estado final não existente",
                ));
            }
        }
    }

    Ok(())
}

fn atribuir_transicoes() {}

fn atribuir_fita() {}

fn atribuir_resultado() {}

pub fn montar_maquina(path: &str) -> std::io::Result<()> {
    let mut maquina = Maquina::new("");
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
                atribuir_estados(&mut maquina, &mut reader)?;
            }
            "q_aceita:" => {
                atribuir_q_aceita(&mut maquina, &mut reader)?;
            }
            "transicoes:" => {
                atribuir_transicoes();
            }
            "fita_inicial:" => {
                atribuir_fita();
            }
            "esperado:" => {
                atribuir_resultado();
            }
            _ => {}
        }
    }

    Ok(())
}
