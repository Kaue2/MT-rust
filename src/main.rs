mod maquina;

use maquina::Maquina;

fn main() {
    let m1 = Maquina {
        fita: String::from("Ola máquina"),
        pos: 0,
    };

    let msg = String::from("Q1 a Q2 b E");
    maquina::get_transition(&msg);
}
