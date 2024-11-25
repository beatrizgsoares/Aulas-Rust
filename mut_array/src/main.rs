use std::io::stdin;

fn main() {
    let mut array_ref:[f64;5] = [1.0,2.0,3.0,4.0,5.0];
    let array_own:[f64;5] = [1.0,2.0,3.0,4.0,5.0];
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
            adicao_ref(&mut array_ref, n);
            let array_own = adicao_own(array_own, n);
            println!("Por referência: {:?}", array_ref);
            println!("Por ownership: {:?}", array_own);
        },"-" => {
            subtracao_ref(&mut array_ref, n);
            let array_own = subtracao_own(array_own, n);
            println!("Por referência: {:?}", array_ref);
            println!("Por ownership: {:?}", array_own);
        },"*" => {
            multiplicacao_ref(&mut array_ref, n);
            let array_own = multiplicacao_own(array_own, n);
            println!("Por referência: {:?}", array_ref);
            println!("Por ownership: {:?}", array_own);
        }, "/" => {
            divisao_ref(&mut array_ref, n);
            let array_own = divisao_own(array_own, n);
            println!("Por referência: {:?}", array_ref);
            println!("Por ownership: {:?}", array_own);
        },
        _ => println!("Operador inválido"),
    }
}

fn adicao_ref(array:&mut [f64], n:f64){
    for i in 0..array.len(){
        array[i]=array[i]+n;
    }
}
fn adicao_own(mut array:[f64;5], n:f64)->[f64;5]{
    for i in 0..array.len(){
        array[i]=array[i]+n;
    }array
}
fn subtracao_ref(array:&mut [f64], n:f64){
    for i in 0..array.len(){
        array[i]=array[i]-n;
    }
}
fn subtracao_own(mut array:[f64;5], n:f64)->[f64;5]{
    for i in 0..array.len(){
        array[i]=array[i]-n;
    }array
}
fn multiplicacao_ref(array:&mut [f64], n:f64){
    for i in 0..array.len(){
        array[i]=array[i]*n;
    }
}
fn multiplicacao_own(mut array:[f64;5], n:f64)->[f64;5]{
    for i in 0..array.len(){
        array[i]=array[i]*n;
    }array
}
fn divisao_ref(array:&mut [f64], n:f64){
    for i in 0..array.len(){
        array[i]=array[i]/n;
    }
}
fn divisao_own(mut array:[f64;5], n:f64)->[f64;5]{
    for i in 0..array.len(){
        array[i]=array[i]/n;
    }array
}