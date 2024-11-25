fn main() {
    let n2: u32 = 253;
    //println!("raiz quadrada aproximada n2: {}", (n2 as f64).sqrt() as u32);
    if primo(n2){
        println!("{} é primo", n2);
    }else{
        println!("{} não é primo", n2);
    }
}
fn primo(n: u32) -> bool {
    for i in 2..(n as f64).sqrt() as u32 +1 {
        //println!("i: {}", i);
        if n % i == 0 {
            return false;
        }
    }return true;
}
