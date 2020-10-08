// Faça um programa que leia um número inteiro qualquer e mostre na tela
// a sua tabuada.

use std::io;

fn main() {
  let mut num = String::new();

  println!("Digite um número:");

  io::stdin()
    .read_line(&mut num)
    .expect("Falha ao ler o número!");
  
  let num: f64 = num.trim().parse()
    .expect("Falha ao converter o número!");

  println!("Tabuada:");
  for i in 1..11 {
    println!("{} x {} = {}", num, i, num * (i as f64));
  }
}