// Desenvolva um programa que leia as duas notas de um aluno, calcule e
// mostre a sua média.

use std::io;

fn scan_float() -> f64 {
  let mut num = String::new();

  io::stdin()
    .read_line(&mut num)
    .expect("Falha ao ler o número!");

  let num: f64 = num.trim().parse()
    .expect("Falha ao converter o número!");

  num
}

fn main() {
  println!("Insira a primeira nota:");
  let n1 = scan_float();
  println!("Insira a segunda nota:");
  let n2 = scan_float();

  println!("As notas foram {} e {}\nA média é {}", n1, n2, (n1+n2)/2.0)
}