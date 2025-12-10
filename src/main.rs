mod maquina;

use maquina::Maquina;

fn main() {
    let _m1 = Maquina::new("aaaaa");

    let msg = String::from("Q1 a Q2 b E");
    maquina::get_transition(&msg);
}
