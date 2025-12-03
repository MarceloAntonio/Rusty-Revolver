
# Rusty-Revolver

Uma implementação em **Rust** que simula a lógica probabilística de uma Roleta Russa. O projeto utiliza geração de números pseudoaleatórios para determinar o encerramento ou continuidade da execução.

## 📋 Funcionamento

O programa opera em um loop de execução baseado na probabilidade de 1/6 (aproximadamente 16,67%):

1.  **Geração:** A cada iteração, dois números inteiros entre 1 e 6 são gerados.
2.  **Condição de Parada:**
    * Se os dois números forem **iguais**: O programa encerra a execução (simulando o disparo).
    * Se os números forem **diferentes**: O fluxo continua.
3.  **Interação:** O sistema solicita um input do usuário (`s` para continuar, `n` para sair).

## 🛠️ Dependências e Módulos

O código fonte utiliza as seguintes bibliotecas da linguagem Rust:

* `std::process::Command`: Para manipulação de comandos do sistema operacional.
* `rand::Rng`: Crate externa para geração de números aleatórios (`rand`).
* `std::io::{self, Write}`: Para manipulação de entrada/saída e buffer.

## 🚀 Instalação e Execução

Siga os passos abaixo para compilar e executar o projeto.

### 1. Clonar o repositório

```bash
git clone https://github.com/marceloantonio/Rusty-Revolver
````

### 2\. Acessar o diretório

```bash
cd Rusty-Revolver
```

### 3\. Executar

**Windows**
Caso utilize o binário pré-compilado:

```cmd
RustProject.exe
```

**Linux / macOS (ou via Cargo)**
Para compilar e rodar diretamente do código fonte:

```bash
cargo run
```

-----

## ⚙️ Instalação do Rust

Caso não tenha o ambiente Rust configurado (necessário para o comando `cargo`), execute o comando abaixo em seu terminal (Linux/macOS):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Para Windows, faça o download do instalador oficial em [rustup.rs](https://rustup.rs).
