#[derive(Debug, Clone)]
struct Livro{
    titulo: String,
    autor: String,
    requisitado: bool
}
fn main() {
    let mut livraria:Vec<Livro> = vec![];

    let livro1 = Livro{
        titulo:String::from("Os Maias"),
        autor:String::from("Eça de Queirós"),
        requisitado:false,
    };
    let livro2 = Livro{
        titulo:String::from("Memorial do Convento"),
        autor:String::from("José Saramago"),
        requisitado:false,
    };

    adicionar_livro(&mut livraria, livro1);
    adicionar_livro(&mut livraria, livro2);
    println!("{:?}", livraria);

    remover_livro(&mut livraria, "Os Maias");
    println!("{:?}", livraria);

    requisitar_livro(&mut livraria, "Memorial do Convento");
    println!("{:?}", livraria);

    devolver_livro(&mut livraria, "Memorial do Convento");
    println!("{:?}", livraria);

}
fn adicionar_livro(livraria:&mut Vec<Livro>, livro:Livro) {
    livraria.push(livro);
}
fn remover_livro(livraria:&mut Vec<Livro>, titulo: &str) {
    for i in 0..livraria.len() {
        if livraria[i].titulo==titulo.to_string() {
            livraria.remove(i);
            return;
        }
    }
}
fn requisitar_livro(livraria:&mut Vec<Livro>, titulo: &str) {
    for i in 0..livraria.len() {
        if livraria[i].titulo==titulo.to_string() {
            livraria[i].requisitado = true;
            return;
        }
    }
}
fn devolver_livro(livraria:&mut Vec<Livro>, titulo: &str) {
    for i in 0..livraria.len() {
        if livraria[i].titulo==titulo.to_string() {
            livraria[i].requisitado = false;
            return;
        }
    }
}