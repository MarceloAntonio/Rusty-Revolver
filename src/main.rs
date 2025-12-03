
use std::process::Command;
use rand::Rng;
use std::io::{self, Write};

fn clear(){

    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(["/c", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}

fn  numero_aleatorio () -> i32{
    rand::thread_rng().gen_range(1..7)
}


fn main() {
    clear();
    


loop {
    clear();
    let municao = numero_aleatorio();
    let camara = numero_aleatorio(); 

    if municao == camara{
        println!("Não deu sorte :(");
        println!("A camara que parou é a {}",camara);
        println!("Munição estava na camara {}",municao);
        break;

    }
    else {
        println!("Deu sorte :)");
        println!("A camara que parou é a {}",camara);
        println!("Munição estava na camara {}",municao);
    }
    print!("\nVocê deseja continuar? \n(s) - Sim \n(n) - Não\nResposta > ");
    io::stdout().flush().unwrap();
    let mut parar = String::new();


    io::stdin().read_line(&mut parar).expect("Erro ao ler");

    let resposta = parar.trim().to_lowercase();
    if resposta == "n" || resposta == "no" || resposta == "não"{
        println!("\n\nParando");
        break;
    }
}

}

