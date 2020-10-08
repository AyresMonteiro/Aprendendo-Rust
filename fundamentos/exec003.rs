// Crie um programa que leia dois números e mostre a soma entre eles.

use std::io;

fn scan_number() -> i32 {
  let mut num = String::new();

  io::stdin()
    .read_line(&mut num)
    .expect("Falha ao ler o número!");

  let num: i32 = num.trim().parse()
    .expect("Erro na conversão do número!");

  num
}

fn main() {
  println!("Insira um número:");
  let n1 = scan_number();
  println!("Insira outro número:");
  let n2 = scan_number();
  println!("A soma é {}", n1 + n2)
}