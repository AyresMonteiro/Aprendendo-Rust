// Faça um algoritmo que leia o salário de um funcionário e mostre seu novo
// salário, com 15% de aumento.

use std::io;

fn main() {
  let mut salario = String::new();

  println!("Insira o valor do salário:");

  io::stdin()
    .read_line(&mut salario)
    .expect("Erro ao ler o salário!");

  let salario: f64 = salario.trim().parse()
    .expect("Erro ao converter o salário!");

  println!("O salário era: R${:.2}", salario);
  println!("Aumentou para: R${:.2}", salario * 1.15)
}