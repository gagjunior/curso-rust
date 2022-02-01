use std::io;

fn convert_to_int(data_input: &String) -> i32 {
   let x = data_input.trim().parse::<i32>().unwrap();
   x
}

fn main() {
   println!("Digite o primeiro numero:");

   let mut number1 = String::new();

   io::stdin().read_line(&mut number1).expect("Falha ao ler entrada");

   println!("Digite o segundo numero:");

   let mut number2 = String::new();

   io::stdin()
      .read_line(&mut number2)
      .expect("Falha ao ler entrada");

   if convert_to_int(&number1) > convert_to_int(&number2) {
      println!("O numero {} eh maior que o numero {}", number1, number2);
   } else {
      println!("O numero {} eh menor ou igual ao numero {}", number1, number2);
   }
}
