// Faça um programa que leia a largura e a altura de uma parede em metros,
// calcule a sua área e a quantidade de tinta necessária para pintá-la, sabendo
// que cada litro de tinta pinta uma área de 2m2.

use std::io;

fn read_float() -> f64 {
  let mut num = String::new();

  io::stdin()
    .read_line(&mut num)
    .expect("Falha ao ler o número!");

  let num: f64 = num.trim().parse()
    .expect("Falha ao converter o número!");
  
  num
}

fn main() {
  println!("Insira a largura da parede em metros:");
  let largura = read_float();
  println!("Insira a altura da parede em metros:");
  let altura = read_float();

  println!("Largura: {} m", largura);
  println!("Altura: {} m", altura);
  println!("Área: {} m^2", largura * altura);
  println!("Litros de tinta necessários: {} L", largura * altura / 2.0);
}