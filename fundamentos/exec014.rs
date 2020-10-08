// Escreva um programa que converta uma temperatura digitada em °C para °F.

use std::io;

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
  celsius * 1.8 + 32.0
}

fn main() {
  let mut celsius = String::new();

  println!("Insira um número:");

  io::stdin()
    .read_line(&mut celsius)
    .expect("Falha ao ler temperatura!");
  
  let celsius: f64 = celsius.trim().parse()
    .expect("Erro ao converter valor!");
  
  let fahrenheit = celsius_to_fahrenheit(celsius);

  println!("A temperatura que você inseriu foi: {} °C", celsius);
  println!("A temperatura correspondente é: {} °F", fahrenheit)
}