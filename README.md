# MT-rust: Simulador de Máquina de Turing

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Status](https://img.shields.io/badge/status-active-success.svg)

**MT-rust** é um simulador de Máquina de Turing de alta performance desenvolvido em **Rust**. O objetivo do projeto é fornecer uma ferramenta CLI robusta para definição, execução e análise de autômatos, aproveitando a segurança de memória e a concorrência do Rust.

## 🚀 Funcionalidades

### Atuais
* Simulação de Máquina de Turing Determinística (DTM) padrão.
* Leitura de configuração de autômatos via arquivo de texto.
* Visualização passo-a-passo da fita e do estado interno.
* Execução eficiente com baixo overhead de memória.

### 🚧 Roadmap & Funcionalidades em Desenvolvimento
O projeto está evoluindo para suportar cenários mais complexos de computação teórica:

* **Suporte a Múltiplas Máquinas:** 
    * Capacidade de definir e executar múltiplas Máquinas de Turing distintas a partir de um único arquivo de setup.
    * Execução sequencial de diferentes autômatos.
    
* **Máquinas de Turing Não-Determinísticas (NDTM) com Threading:**
    * Implementação de não-determinismo real utilizando o sistema de *threads* do Rust.
    * Quando o autômato encontra múltiplas transições possíveis para um mesmo estado/símbolo, o processo se ramifica (spawns threads), explorando caminhos de computação paralelamente.

## 📦 Instalação

Certifique-se de ter o [Rust e o Cargo instalados](https://www.rust-lang.org/tools/install).

```bash
# Clone o repositório
git clone [https://github.com/Kaue2/MT-rust.git](https://github.com/Kaue2/MT-rust.git)

# Entre no diretório
cd MT-rust

# Compile o projeto em modo release (recomendado para performance)
cargo build --release
