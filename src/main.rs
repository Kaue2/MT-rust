mod leitor;
mod maquina;

use maquina::{Direcao, Maquina, Transicao};

fn main() {
    let result = leitor::montar_maquina("assets/maquina.txt");

    match result {
        Ok(_) => {
            println!("Tudo certo")
        }
        Err(err) => {
            println!("Erro: {}", err)
        }
    }
}
