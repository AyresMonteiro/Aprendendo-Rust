// Faça um programa que leia um número inteiro e mostre na tela o seu
// sucessor e o seu antecessor.

use std::io;

fn main() {
  let mut num = String::new();

  println!("Insira um número:");

  io::stdin()
    .read_line(&mut num)
    .expect("Falha ao ler número!");
  
  let num: i64 = num.trim().parse()
    .expect("Falha ao converter número!");

  println!("Número digitado: {}", num);
  println!("Antecessor: {}", num - 1);
  println!("Sucessor: {}", num + 1)
}