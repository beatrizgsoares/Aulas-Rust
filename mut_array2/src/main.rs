use std::io::stdin;

fn main() {
    let mut array_ref:[f64;5] = [1.0,2.0,3.0,4.0,5.0];
    let mut operacao = String::new();
    let mut n=String::new();

    println!("Número: ");
    stdin()
        .read_line(&mut n)
        .expect("Did not enter a correct string");
    let n:f64 = n.trim().parse()
        .expect("Valor não é um número válido");

    println!("Escolhe função: (+,-,* ou /)");
    stdin()
        .read_line(&mut operacao)
        .expect("Did not enter a correct string");

    match operacao.trim() {
        "+" => {
            adição(&mut array_ref, n);
            println!("Por referência: {:?}", array_ref);
        },"-" => {
            subtração(&mut array_ref, n);
            println!("Por referência: {:?}", array_ref);
        },"*" => {
            multiplicação(&mut array_ref, n);
            println!("Por referência: {:?}", array_ref);
        }, "/" => {
            divisão(&mut array_ref, n);
            println!("Por referência: {:?}", array_ref);
        },
        _ => println!("Operador inválido"),
    }
}
fn adição(array: &mut [f64], n: f64){
    array.into_iter()
        .for_each(|x| *x=*x+n);
}
fn subtração(array: &mut [f64], n: f64){
    array.into_iter()
        .for_each(|x| *x=*x-n);
}
fn multiplicação(array: &mut [f64], n: f64){
    array.into_iter()
        .for_each(|x| *x=*x*n);
}
fn divisão(array: &mut [f64], n: f64){
    array.into_iter()
        .for_each(|x| *x=*x/n);
}