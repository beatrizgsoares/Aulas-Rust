fn main() {
    let mut s_ref=String::from("😀 Olá mundo 👋");
    let s_own=String::from("😀 Olá mundo 👋");
    println!("{}", s_ref);
    println!("{}", s_own);

    let s_own = remover_owned(s_own, "Olá");
    remover_ref(& mut s_ref, "Olá");
    println!("{}", s_own);
    println!("{}", s_ref);

    let s_own = adicionar_owned(s_own, "Bom dia", 2);
    adicionar_ref(&mut s_ref, "Bom dia", 2);
    println!("{}", s_own);
    println!("{}", s_ref);

    let s_own = substituir_own(s_own, "mundo 👋", "galáxia 🖖");
    substituir_ref(&mut s_ref, "mundo 👋", "galáxia 🖖");
    println!("{}", s_own);
    println!("{}", s_ref);

    let s_own = maiuscula_letra_owned(s_own,"á");
    maiuscula_letra_ref(&mut s_ref,"á");
    println!("{}", s_own);
    println!("{}", s_ref);

    let s_own = maiuscula_total_own(s_own);
    maiuscula_total_ref(&mut s_ref);
    println!("{}", s_own);
    println!("{}", s_ref);

    let s_own = minuscula_letra_own(s_own, "A");
    minuscula_letra_ref(&mut s_ref,"A");
    println!("{}", s_own);
    println!("{}", s_ref);

    let s_own = minuscula_total_own(s_own);
    minuscula_total_ref(&mut s_ref);
    println!("{}", s_own);
    println!("{}", s_ref);

}
fn remover_owned(s: String, sub: &str) -> String {
    s.replace(sub, "")
}
fn remover_ref(s: &mut String, sub:&str) {
    let i = s.find(sub).expect("String a substituir não encontrada");
    let comp = sub.chars().count();
    //println!("Comprimento: {}", comp);
    let mut n = 0;
    while n<comp {
        s.remove(i);
        n+=1;
    }
}
fn adicionar_owned(mut s: String, adic:&str, ind:usize) -> String {
    //Para funcionar com caractéres maiores que 1 byte (ex. á):
    let (ind_corrigido, _) = s.char_indices().nth(ind).unwrap();
    s.insert_str(ind_corrigido, adic);
    s
}
fn adicionar_ref(s: &mut String, adic: &str, ind: usize) {
    let (ind_corrigido, _) = s.char_indices().nth(ind).unwrap();
    s.insert_str(ind_corrigido, adic);
}
fn substituir_own(s: String, sub:&str, adic:&str) -> String {
    s.replace(sub, adic)
}
fn substituir_ref(s: &mut String, sub:&str, adic:&str) {
    let i = s.find(sub).expect("String a substituir não encontrada");
    //println!("{}", i);
    s.insert_str(i, adic);
    //println!("{}", s);
    remover_ref(s, sub);
}
fn maiuscula_letra_owned(s: String, letra: &str) -> String {
    s.replace(letra, letra.to_uppercase().as_str())
}
fn maiuscula_letra_ref(s: &mut String, letra: &str) {
    for (i, c) in s.clone().match_indices(letra) {
        //println!("{} {}", i, c);
        s.replace_range(i..i+c.len(), letra.to_uppercase().as_str());
    }
}
fn maiuscula_total_own(s: String) -> String {
    s.to_uppercase()
}
fn maiuscula_total_ref(s: &mut String) {
    for (i, c) in s.clone().char_indices() {
        //println!("{}, {}", i, c);
        s.replace_range(i..i+c.len_utf8(), c.to_uppercase().to_string().as_str());
    }
}
fn minuscula_letra_own(s: String, letra: &str) -> String {
    s.replace(letra, letra.to_lowercase().as_str())
}
fn minuscula_letra_ref(s: &mut String, letra: &str){
    for (i, c) in s.clone().match_indices(letra) {
        //println!("{} {}", i, c);
        s.replace_range(i..i+c.len(), letra.to_lowercase().as_str());
    }
}
fn minuscula_total_own(s: String) -> String {
    s.to_lowercase()
}
fn minuscula_total_ref(s: &mut String) {
    for (i, c) in s.clone().char_indices() {
        s.replace_range(i..i+c.len_utf8(), c.to_lowercase().to_string().as_str());
    }
}