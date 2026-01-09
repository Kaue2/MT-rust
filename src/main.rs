mod leitor;
mod maquina;

use maquina::{Maquina, MaquinaErro};

#[derive(Debug)]
struct ProgramaErro;

fn main() -> Result<(), ProgramaErro> {
    let mut mt1: Maquina = match leitor::montar_maquina("assets/maquina.txt") {
        Ok(mt) => {
            println!("Sucesso ao criar a máquina\n\n");
            mt
        }
        Err(err) => {
            eprintln!("Não foi possível criar a máquina");
            eprintln!("Erro: {}", err);

            return Err(ProgramaErro);
        }
    };

    println!("Máquina encontrada: \n\n{}", mt1);
    let result = match mt1.simular_maquina() {
        Ok(true) => true,
        Ok(false) => true,
        Err(MaquinaErro) => false,
    };

    if result != true {
        return Err(ProgramaErro);
    }

    Ok(())
}
