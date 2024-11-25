fn main() {
    let chave:u8 = 27;
    let mut mensagem_ref = String::from("Ola mundo");
    mensagem_ref.replace_range(0..1, "w");
    //cifra_cesar_ref(&mut mensagem_ref,chave);
    println!("{}", mensagem_ref);
    /*let mensagem_descifrada = descifra_cesar(&mensagem_cifrada,chave);
    println!("{}", mensagem_descifrada);*/

    let chave_vig = "abcd ";
    if !chave_valida(chave_vig) {
        println!("Chave inválida. Inserir apenas valores de a-z ou A-Z");
    }
}
fn cifra_cesar_own(s: String, n:u8) -> String {
    let mut s_cifrado = String::new();
    if !s.is_ascii(){
        println!("Valor não válido. Inserir apenas caracteres ASCII.");
        return "".to_string();
    }for valor in s.bytes(){
        //println!("{}", valor);
        if valor>=65 && valor<=90{
            let char_cifrado = (65+(valor+n-65)%26) as char;
            //println!("{}", char_cifrado);
            s_cifrado.push(char_cifrado);
        }else if valor>=97 && valor<=122{
            let char_cifrado = (97+(valor+n-97)%26) as char;
            s_cifrado.push(char_cifrado);
        }else{
            s_cifrado.push(valor as char);
        }
    }return s_cifrado;
}
fn descifra_cesar_own(s: String, n:u8) -> String {
    cifra_cesar_own(s, 26-n%26)
}
fn cifra_cesar_ref(s: &mut str, n:u8){
    if !s.is_ascii(){
        println!("Valor não válido. Inserir apenas caracteres ASCII.");
        return;
    }for (i,valor) in s.bytes().enumerate(){
        s.replace(i..(i+1), mover(valor, n));
    }
}
fn cifra_vigenere(s: &str, chave: &str) -> String{
    let mut s_cifrado = String::new();
    if !s.is_ascii(){
        println!("Valor não válido. Inserir apenas caracteres ASCII.");
        return "".to_string();
    }else if !chave_valida(chave){
        println!("Chave não é válida. Só pode tomar valores de A-Z ou a-z.");
        return "".to_string();
    }/*for (i,valor) in s.bytes().enumerate(){
        if chave[i]
    }*/
    return s_cifrado;
}
fn chave_valida(chave: &str) -> bool{
    if !chave.is_ascii(){
        return false;
    }for valor in chave.bytes(){
        if !((valor>=65 && valor<=90) || (valor>=97 && valor<=122)){
            return false;
        }
    }return true;
}
fn mover(valor: u8, n: u8)-> char{
    if valor>=65 && valor<=90{
        return (65+(valor+n-65)%26) as char;
    }else if valor>=97 && valor<=122{
        return (97+(valor+n-97)%26) as char;
    }else{
        return valor as char;
    }
}