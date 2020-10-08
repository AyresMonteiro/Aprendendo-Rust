// Escreva um programa que pergunte a quantidade de km percorridos por um carro
// alugado e a quantidade de dias pelos quais ele foi alugado. Calcule o preço a
// pagar, sabendo que o carro custa R$60,00 por dia e R$0,15 por km rodado.

use std::io;

fn ask_float() -> f64 {
  let mut num = String::new();

  io::stdin()
    .read_line(&mut num)
    .expect("Erro ao ler número!");

  let num = num.trim().parse()
    .expect("Falha ao converter número!");

  num
}

fn main() {
  println!("Insira o número de dias de aluguel:");
  let dias = ask_float();

  println!("Insira o número de km rodados:");
  let km = ask_float();

  println!("O preço a se pagar é: R${:.2}", dias * 60.0 + km * 0.15);
}