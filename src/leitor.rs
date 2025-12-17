use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

use crate::maquina::{Direcao, Maquina, Transicao};

fn get_estados(reader: &mut BufReader<File>) -> io::Result<Vec<String>> {
    let mut buffer = String::new();
    let mut estados: Vec<String> = Vec::new();

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
    let mut estados: Vec<String> = Vec::new();

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

fn get_transicoes(
    maquina: &mut Maquina,
    reader: &mut BufReader<File>,
) -> io::Result<Vec<Transicao>> {
    let mut buffer = String::new();
    let mut transicoes: Vec<Transicao> = Vec::new();

    loop {
        buffer.clear();
        let bytes_lidos = reader.read_line(&mut buffer)?;
        if bytes_lidos == 0 {
            break;
        }

        if buffer.clone().trim().is_empty() {
            break;
        }

        let partes: Vec<&str> = buffer.split_whitespace().collect();

        let dir = match partes[4] {
            "E" => Direcao::E,
            "D" => Direcao::D,
            _ => panic!("Direção desconhecida: {}", partes[4]),
        };

        let estado_atual = match maquina.mapa_nomes.get(partes[0]) {
            Some(&valor) => valor,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Erro: não foi possível encontrar o estado atual no mapa de nomes",
                ));
            }
        };

        let estado_destino = match maquina.mapa_nomes.get(partes[2]) {
            Some(&valor) => valor,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Erro: não foi possível encontrar o estado de destino no mapa de nomes",
                ));
            }
        };

        transicoes.push(Transicao {
            estado_atual: estado_atual,
            char_leitura: partes[1]
                .parse()
                .expect("Erro: não foi possível colocar char leitura"),
            estado_destino: estado_destino,
            char_escrita: partes[3]
                .parse()
                .expect("Erro: não foi possível colocar char escrita"),
            direcao: dir,
        });
    }

    Ok(transicoes)
}

fn atribuir_transicoes(maquina: &mut Maquina, reader: &mut BufReader<File>) -> io::Result<()> {
    let transicoes: Vec<Transicao> = get_transicoes(maquina, reader)?;

    for transicao in transicoes {
        let key = (transicao.estado_atual, transicao.char_leitura);
        maquina.transicoes.insert(key, transicao);
    }

    Ok(())
}

fn get_fita(reader: &mut BufReader<File>) -> io::Result<String> {
    let mut buffer = String::new();
    let mut fita = String::new();

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

        fita = fita + linha;
    }

    Ok(fita)
}

fn atribuir_fita(maquina: &mut Maquina, reader: &mut BufReader<File>) -> io::Result<()> {
    let fita = get_fita(reader)?;
    maquina.fita = fita.chars().collect();

    Ok(())
}

fn atribuir_resultado(maquina: &mut Maquina, reader: &mut BufReader<File>) -> io::Result<()> {
    let resultado = get_fita(reader)?;
    maquina.resultado = resultado.chars().collect();

    Ok(())
}

pub fn montar_maquina(path: &str) -> std::io::Result<Maquina> {
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
                atribuir_transicoes(&mut maquina, &mut reader)?;
            }
            "fita_inicial:" => {
                atribuir_fita(&mut maquina, &mut reader)?;
            }
            "esperado:" => {
                atribuir_resultado(&mut maquina, &mut reader)?;
            }
            _ => {
                println!("linha encontrada: {}", linha);
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("O arquivo lido não respeita o formato necessesário"),
                ));
            }
        }
    }

    Ok(maquina)
}
