use std::io;

fn convert_to_int(data_input: &String) -> i32 {
   let x = data_input.trim().parse::<i32>().unwrap();
   x
}

fn main() {

   //Ler numero 1
   let mut number1 = String::new();
   io::stdin().read_line(&mut number1).expect("Erro ao ler number1");   

   //Ler numero 2
   let mut number2 = String::new();
   io::stdin().read_line(&mut number2).expect("Erro ao ler number2");

   if number1 > number2 {
       println!("Numero 1({}) é maior que o numero 2({})", number1, number2);
    } 
    else {
       println!("Numero 1({}) é menor ou igual ao numero 2({})", number1, number2);
   }
}
