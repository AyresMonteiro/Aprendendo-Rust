// Faça um programa que leia o comprimento do cateto oposto e do cateto
// adjacente de um triângulo retângulo, calcule e mostre o comprimento da
// hipotenusa.

use std::io;

fn ask_number() -> f64 {
  let mut num = String::new();

  io::stdin()
    .read_line(&mut num)
    .expect("Erro ao ler número!");

  let num: f64 = num.trim().parse()
    .expect("Erro ao converter número!");

  num
}

fn main() {
  println!("Insira o valor do cateto oposto:");
  let cat_op = ask_number();
  println!("Insira o valor do cateto adjacente:");
  let cat_adj = ask_number();

  println!("A hipotenusa desse triângulo é {}.", (cat_op.powf(2_f64) + cat_adj.powf(2_f64)).sqrt());
}
