// Faça um programa que leia algo pelo teclado e mostre na tela o seu tipo
// primitivo e todas as informações possíveis sobre ele.

use std::io;

fn main() {
  let mut alguma_coisa = String::new();

  println!("Digite alguma coisa:");

  io::stdin()
    .read_line(&mut alguma_coisa)
    .expect("Falha ao ler alguma coisa!");

  let alguma_coisa = alguma_coisa.trim();

  println!("Tipo do valor \"{}\": string.", alguma_coisa);
  println!("É da codificação ASCII? {}", alguma_coisa.is_ascii());
  println!("É uma variável vazia? {}", alguma_coisa.is_empty())
}