use std::io::stdin;

fn main() {
    let mut operacao = String::new();
    let mut a=String::new();
    let mut b=String::new();

    println!("Primeiro valor: ");
    stdin()
        .read_line(&mut a)
        .expect("Did not enter a correct string");
    let a:f64 = a.trim().parse()
        .expect("Primeiro valor não é número válido");

    println!("Segundo valor: ");
    stdin()
        .read_line(&mut b)
        .expect("Did not enter a correct string");
    let b:f64 = b.trim().parse()
        .expect("Segundo valor não é número válido");

    println!("Escolhe função: (+,-,* ou /)");
    stdin()
        .read_line(&mut operacao)
        .expect("Did not enter a correct string");

    //let a:f64 = a.trim().parse().unwrap();
    //let b:f64 = b.trim().parse().unwrap();
    //let mut operacao:String = operacao.trim().parse().unwrap();

    match operacao.trim(){
        "+" => println!("{a}+{b}={}",a+b),
        "-" => println!("{a}-{b}={}",a-b),
        "/" => println!("{a}/{b}={}",a/b),
        "*" => println!("{a}*{b}={}",a*b),
        _ => println!("Operador inválido"),
    }
}