// Crie um programa que leia quantos reais uma pessoa tem na carteira e
// mostre quantos dólares ela pode comprar.

use std::io;

fn main() {
  // Cotação do dólar no dia 08/10/2020
  let cotacao: f64 = 5.61;

  let mut reais = String::new();

  println!("Insira um valor em reais:");
  io::stdin()
    .read_line(&mut reais)
    .expect("Erro ao ler o valor!");
  
  let reais: f64 = reais.trim().parse()
    .expect("Falha ao converter valor!");

  println!("Com R${:.2} você pode comprar US${:.2}.", reais, reais/cotacao);
}