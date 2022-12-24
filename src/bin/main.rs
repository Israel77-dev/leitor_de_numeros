use std::env;

use leitor_de_numeros::parser::parser;

fn main() {
    let args: Vec<_> = env::args().collect();

    for arg in args[1..].iter() {
        match arg.parse::<u128>() {
            Ok(n) => println!(
                "{}",
                parser::ler_numero(n).unwrap_or("Número inválido".to_owned())
            ),
            Err(_) => panic!("Argumento inválido: {}", arg),
        }
    }
}
