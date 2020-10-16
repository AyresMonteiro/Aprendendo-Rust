// Crie um programa que leia um número real qualquer pelo teclado e mostre
// na tela a sua porção inteira.

use std::io;

fn main() {
  let mut num = String::new();

  println!("Insira um número:");

  io::stdin()
    .read_line(&mut num)
    .expect("Erro ao ler número!");

  let num: f64 = num.trim().parse()
    .expect("Erro ao converter número!");

  println!("float: {}\nint: {}", num, num.trunc());
}
