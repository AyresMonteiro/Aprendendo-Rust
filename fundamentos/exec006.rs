// Crie um algoritmo que leia um número e mostre o seu dobro, triplo e raiz
// quadrada.

use std::io;

fn main() {
  let mut num = String::new();

  println!("Digite um número:");

  io::stdin()
    .read_line(&mut num)
    .expect("Falha ao ler número!");

  let num: f64 = num.trim().parse()
    .expect("Falha ao converter o número!");
  
  println!("O número que você digitou é: {}", num);
  println!("O dobro é: {}", num * 2.0);
  println!("O triplo é: {}", num * 3.0);
  println!("A raiz quadrada é: {}", num.sqrt());
}