// Escreva um programa que leia um valor em metros e o exiba convertido
// em centímetros e milímetros.

use std::io;

fn main() {
  let mut num = String::new();

  println!("Digite um valor em metros!");

  io::stdin()
    .read_line(&mut num)
    .expect("Falha ao ler o número!");
  
  let num: f64 = num.trim().parse()
    .expect("Falha ao converter o número!");
  
  println!("O valor que você digitou é {} m", num);
  println!("O valor que você digitou em centímetros é {} cm", num*100.0);
  println!("O valor que você digitou em milímetros é {} mm", num*1000.0)
}