mod leitor;
mod maquina;

use maquina::{Direcao, Maquina, Transicao};

fn main() {
    let mut m1 = Maquina::new("aaaaa");

    let t: Transicao = Transicao::new(0, 'a', 0, 'b', Direcao::D);
    m1.transicoes
        .insert((t.estado_atual.clone(), t.char_leitura), t);

    m1.passo_simulacao();

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
