// Faça um programa que leia o nome de uma pessoa e mostre uma mensagem 
// de boas-vindas.

use std::io;

fn main() {
  let mut name = String::new();
  println!("Insira seu nome:");

  io::stdin()
    .read_line(&mut name)
    .expect("Falha ao ler o nome!");

  let name = name.trim();

  println!("Olá {}, seja bem-vindo(a)!", name)
}