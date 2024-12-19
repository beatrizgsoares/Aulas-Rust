fn main() {
    let mut mensagem_ref = String::from("Ola mundo!");
    let mensagem_own = mensagem_ref.clone();

    println!("Mensagem inicial: {}", mensagem_ref);

    println!("-------Cifra de César--------");

    let chave:u8 = 27;
    let mensagem_own = cifra_cesar_own(mensagem_own, chave);
    cifra_cesar_ref(&mut mensagem_ref,chave);
    println!("Mensagem cifrada (ownership): {}", mensagem_own);
    println!("Mensagem cifrada (referência): {}", mensagem_ref);

    let mensagem_own = descifra_cesar_own(mensagem_own,chave);
    descifra_cesar_ref(&mut mensagem_ref,chave);
    println!("Mensagem descifrada (ownership): {}", mensagem_own);
    println!("Mensagem descifrada (referência): {}", mensagem_ref);


    println!("-------Cifra de Vigenère--------");

    let chave_vig = "umafrasequalquer";
    let mensagem_own = cifra_vigenere_own(mensagem_own, chave_vig);
    cifra_vigenere_ref(&mut mensagem_ref,chave_vig);
    println!("Mensagem cifrada (ownership): {}", mensagem_own);
    println!("Mensagem cifrada (referência): {}", mensagem_ref);

    let mensagem_own = descifra_vigenere_own(mensagem_own,chave_vig);
    descifra_vigenere_ref(&mut mensagem_ref,chave_vig);
    println!("Mensagem descifrada (ownership): {}", mensagem_own);
    println!("Mensagem descifrada (referência): {}", mensagem_ref);
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
fn cifra_cesar_own(s: String, n:u8) -> String {
    let mut s_cifrado = String::new();
    if !s.is_ascii(){
        println!("Valor não válido. Inserir apenas caracteres ASCII.");
        return "".to_string();
    }for valor in s.bytes(){
        let char_cifrado = mover(valor, n);
        s_cifrado.push(char_cifrado);
    }return s_cifrado;
}
fn descifra_cesar_own(s: String, n:u8) -> String {
    cifra_cesar_own(s, 26-n%26)
}
fn cifra_cesar_ref(s: &mut String, n:u8){
    if !s.is_ascii(){
        println!("Valor não válido. Inserir apenas caracteres ASCII.");
        return;
    }for (i,valor) in s.clone().bytes().enumerate(){
        s.replace_range(i..(i+1), mover(valor, n).to_string().as_str());
    }
}
fn descifra_cesar_ref(s: &mut String, n:u8){
    cifra_cesar_ref(s, 26-n%26);
}
fn cifra_vigenere_own(s: String, chave: &str) -> String{
    if !chave_valida(chave) {
        println!("Chave inválida. Inserir apenas valores de a-z ou A-Z");
        return "".to_string();
    }else if !s.is_ascii(){
        println!("Valor não válido. Inserir apenas caracteres ASCII.");
        return "".to_string();
    }
    let mut s_cifrado = String::new();
    for (i,valor) in s.bytes().enumerate(){
        let n = chave.as_bytes()[i];
        s_cifrado.push(mover(valor, n));
    }
    s_cifrado
}
fn cifra_vigenere_ref(s: &mut String, chave: &str){
    if !chave_valida(chave) {
        println!("Chave inválida. Inserir apenas valores de a-z ou A-Z");
        return;
    }else if !s.is_ascii(){
        println!("Valor não válido. Inserir apenas caracteres ASCII.");
        return;
    }for (i,valor) in s.clone().bytes().enumerate(){
        let n = chave.as_bytes()[i];
        s.replace_range(i..(i+1), mover(valor, n).to_string().as_str());
    }
}
fn descifra_vigenere_own(s: String, chave: &str) -> String{
    let mut s_descifrado = String::new();
    for (i,valor) in s.bytes().enumerate(){
        let n = chave.as_bytes()[i];
        s_descifrado.push(mover(valor, 26-n%26));
    }s_descifrado
}
fn descifra_vigenere_ref(s: &mut String, chave: &str){
    for (i,valor) in s.clone().bytes().enumerate(){
        let n = chave.as_bytes()[i];
        s.replace_range(i..(i+1), mover(valor, 26-n%26).to_string().as_str());
    }
}
fn chave_valida(chave: &str) -> bool{
    if !chave.is_ascii(){
        return false;
    }for valor in chave.bytes(){
        if !((valor>=65 && valor<=90) || (valor>=97 && valor<=122)){
            return false;
        }
    }true
}