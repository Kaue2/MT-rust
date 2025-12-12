mod maquina;

use maquina::{Direcao, Maquina, Transicao};

fn main() {
    let mut m1 = Maquina::new("aaaaa");

    let t: Transicao = Transicao::new(String::from("q0"), 'a', String::from("q0"), 'b', Direcao::D);
    m1.transicoes
        .insert((t.estado_atual.clone(), t.char_leitura), t);

    let msg = String::from("Q1 a Q2 b E");
    maquina::get_transition(&msg);

    m1.passo_simulacao();
}
