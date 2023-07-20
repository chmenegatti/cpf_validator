use std::io;

fn main() {
  println!("Digite um CPF (apenas os dígitos):");
  
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Falha ao ler a entrada.");
  let cpf = input.trim();
  
  if validar_cpf(cpf) {
    println!("CPF válido!");
  } else {
    println!("CPF inválido!");
  }
}

fn validar_cpf(cpf: &str) -> bool {
  // Remover caracteres não numéricos do CPF
  let cpf_clean: String = cpf.chars().filter(|c| c.is_numeric()).collect();
  
  if cpf_clean.len() != 11 {
    return false;
  }
  
  let cpf_digits: Vec<u32> = cpf_clean
  .chars()
  .map(|c| c.to_digit(10).unwrap())
  .collect();
  
  let mut sum1: u32 = 0;
  let mut sum2: u32 = 0;
  
  for i in 0..9 {
    sum1 += cpf_digits[i] * (10 - i as u32);
    sum2 += cpf_digits[i] * (11 - i as u32);
  }
  
  let mod1 = (sum1 % 11) % 10;
  let mod2 = (sum2 + 2 * mod1) % 11 % 10;
  
  mod1 == cpf_digits[9] && mod2 == cpf_digits[10]
}
