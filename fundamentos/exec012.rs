// Faça um algoritmo que leia o preço de um produto e mostre seu novo
// preço, com 5% de desconto.

use std::io;

fn main() {
  let mut preco = String::new();

  println!("Insira o preço do produto:");

  io::stdin()
    .read_line(&mut preco)
    .expect("Falha ao ler o preço!");

  let preco: f64 = preco.trim().parse()
    .expect("Falha ao converter o número!");
  
  println!("Preço: {:.2}", preco);
  println!("Preço com desconto: {:.2}", preco * 0.95)
}