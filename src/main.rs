mod leitor;
mod maquina;

use maquina::{Direcao, Maquina, Transicao};

fn main() {
    let mut mt1: Maquina = match leitor::montar_maquina("assets/maquina.txt") {
        Ok(mt) => {
            println!("Sucesso ao criar a máquina\n\n");
            mt
        }
        Err(err) => {
            eprintln!("Não foi possível criar a máquina");
            eprintln!("Erro: {}", err);

            return;
        }
    };

    println!("Máquina encontrada: \n\n{}", mt1);
    mt1.passo_simulacao();
}
