use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq)]
struct Livro{
    titulo: String,
    autor: String,
    requisitado: bool,
    isbn: u64,
    palavras_chave: Vec<String>,
}
#[derive(Debug)]
struct Livraria{
    livros_por_isbn: HashMap<u64, Livro>,
    isbn_por_titulo: HashMap<String, u64>,
    isbn_por_autor: HashMap<String, HashSet<u64>>, //HashSet pois um autor pode ter vários livros
    isbn_por_palavra_chave: HashMap<String, HashSet<u64>>, //HashSet -> conjunto de isbns dos livros que usam determinada palavra-chave
}
fn main() {
    let mut livraria = Livraria::new();
    let livro1 = Livro{
        titulo:String::from("Os Maias"),
        autor:String::from("Eça de Queirós"),
        requisitado:false,
        isbn: 978_972_0_04957_5,
        palavras_chave: vec!["português".to_string(), "romance".to_string()],
    };
    let livro2 = Livro{
        titulo:String::from("Memorial do Convento"),
        autor:String::from("José Saramago"),
        requisitado:false,
        isbn: 978_972_0_04671_0,
        palavras_chave: vec!["português".to_string(), "romance".to_string()],
    };
    let livro3 = Livro{
        titulo:String::from("A Cidade e as Serras"),
        autor:String::from("Eça de Queirós"),
        requisitado:false,
        isbn: 978_972_38_2949_5,
        palavras_chave: vec!["português".to_string(), "romance".to_string()],
    };
    let livro4 = Livro{
        titulo: String::from("Guerra e Paz"),
        autor: String::from("Lev Tolstoi"),
        requisitado: false,
        isbn: 978_972_23_3334_4,
        palavras_chave: vec!["português".to_string(), "tradução".to_string(), "romance".to_string()],
    };
    let livro5 = Livro{
        titulo: String::from("Cem Anos de Solidão"),
        autor: String::from("Gabriel García Márquez"),
        requisitado: false,
        isbn: 978_972_20_5996_1,
        palavras_chave: vec!["português".to_string(), "tradução".to_string(), "romance".to_string()],
    };
    //println!("{}",livro3);

    println!("--------Adicionar Livros-------");
    livraria.adicionar_livro(livro1);
    livraria.adicionar_livro(livro2);
    livraria.adicionar_livro(livro3);
    livraria.adicionar_livro(livro4);
    livraria.adicionar_livro(livro5);
    println!("{}", livraria);

    println!("--------Remover Livro-------");
    let livro_removido = livraria.remover_livro_isbn(978_972_0_04671_0).unwrap();
    println!("{}", livraria);
    println!("Livro removido: {}", livro_removido);
    livraria.remover_livro_isbn(1);

    println!("--------Requisitar Livro-------");
    livraria.requisitar_livro(978_972_23_3334_4);
    println!("{}", livraria);
    livraria.requisitar_livro(1);

    println!("--------Devolver Livro-------");
    livraria.devolver_livro(978_972_23_3334_4);
    println!("{}", livraria);

    println!("--------Procurar Livro por ISBN-------");
    match livraria.procurar_por_isbn(978_972_0_04957_5) {
        None => {println!("Livro não encontrado.")}
        Some(ref_livro) => {println!("{}", ref_livro);}
    };
    match livraria.procurar_por_isbn(1) {
        None => {println!("Livro não encontrado.")}
        Some(ref_livro) => {println!("{}", ref_livro);}
    };

    println!("---------Procurar Livro por Título-------");
    match livraria.procurar_por_titulo("Memorial do Convento") {
        None => println!("Livro não encontrado."),
        Some(ref_livro) => {println!("{}", ref_livro);}
    }
    match livraria.procurar_por_titulo("Guerra e Paz") {
        None => println!("Livro não encontrado."),
        Some(ref_livro) => {println!("{}", ref_livro);}
    }

    println!("----------Procurar Livros por Autor-------");
    livraria.procurar_por_autor("Eça de Queirós");

    println!("----Procurar por união de palavras-chave (retorna todos os livros que têm pelo menos uma das palavras-chave dadas)----");
    let lista_isbns = livraria.procurar_por_palavras_chave_uniao(vec!["português".to_string()]);
    //println!("{:?}",lista_isbns);
    println!("Livros que têm a palavra-chave: 'português':");
    for isbn in lista_isbns{
        println!("{}", livraria.livros_por_isbn.get(&isbn).unwrap());
    }

    println!("----Procurar por interseção de palavras-chave (retorna todos os livros que têm todas palavras-chave dadas)----");
    let lista_isbns = livraria.procurar_por_palavras_chave_inter(vec!["português".to_string(), "tradução".to_string()]);
    //println!("{:?}",lista_isbns);
    println!("Livros que têm a palavra-chave: 'português' e 'tradução':");
    for isbn in lista_isbns{
        println!("{}", livraria.livros_por_isbn.get(&isbn).unwrap());
    }

    /*let a = HashSet::new();
    let b = HashSet::from([1,3,4]);
    let a: HashSet<_> = a.union(&b).cloned().collect();
    let c = HashSet::from([6,3,1]);
    let a: HashSet<_> = a.union(&c).cloned().collect();
    println!("{:?}",a);*/

}
impl Livraria {
    fn new () -> Livraria {
        Livraria{
            livros_por_isbn: HashMap::new(),
            isbn_por_titulo: HashMap::new(),
            isbn_por_autor: HashMap::new(),
            isbn_por_palavra_chave: HashMap::new()
        }
    }
    fn adicionar_livro(&mut self, livro:Livro) {
        self.isbn_por_titulo.insert(livro.titulo.clone(), livro.isbn);
        match self.isbn_por_autor.get_mut(&livro.autor){
            Some(h) => {h.insert(livro.isbn);}, //autor já existe na base de dados
            None => {self.isbn_por_autor.insert(livro.autor.clone(), HashSet::from([livro.isbn]));} //novo autor
        }
        for palavra_chave in livro.palavras_chave.iter(){
            match self.isbn_por_palavra_chave.get_mut(palavra_chave) {
                Some(h) => { h.insert(livro.isbn); }, //palavra-chave já existe na base de dados
                None => { self.isbn_por_palavra_chave.insert(palavra_chave.clone(), HashSet::from([livro.isbn])); } //nova palavra-chave
            }
        }
        self.livros_por_isbn.insert(livro.isbn, livro);
    }
    fn remover_livro_isbn(&mut self, isbn:u64) -> Option<Livro> {
        let livro = match self.livros_por_isbn.remove(&isbn){
            Some(livro) => livro,
            None => {println!("Livro não existe na livraria. Nada foi removido.");
                return None}
        };
        self.isbn_por_titulo.remove(&livro.titulo);
        self.isbn_por_autor.get_mut(&livro.autor).unwrap().remove(&isbn);
        for palavra_chave in livro.palavras_chave.iter(){
            self.isbn_por_palavra_chave.get_mut(palavra_chave).unwrap().remove(&isbn);
        }
        Some(livro)
    }
    fn requisitar_livro(&mut self, isbn:u64) {
        match self.livros_por_isbn.get_mut(&isbn){
            Some(livro) => {livro.requisitado = true},
            None => {println!("Livro não existe na livraria, por isso não pode ser requisitado.");}
        }
    }
    fn devolver_livro(&mut self, isbn:u64) {
        match self.livros_por_isbn.get_mut(&isbn){
            Some(livro) => {livro.requisitado = false},
            None => {println!("Livro não existe na base de dados da livraria, por isso não pode ser devolvido.");}
        }
    }
    fn procurar_por_isbn(&self, isbn:u64) -> Option<&Livro> {
        self.livros_por_isbn.get(&isbn)
    }
    fn procurar_por_titulo(&self, titulo:&str) -> Option<&Livro> {
        match self.isbn_por_titulo.get(titulo) {
            Some(isbn) => self.procurar_por_isbn(*isbn),
            None => None
        }
    }
    fn procurar_por_autor(&self, autor:&str) -> Vec<&Livro>{
        match self.isbn_por_autor.get(autor) {
            Some(isbn_list) => {
                println!("Livros de {}:", autor);
                let mut lista_ref_livros = vec![];
                for isbn in isbn_list.iter(){
                    let livro = self.procurar_por_isbn(*isbn).unwrap();
                    lista_ref_livros.push(livro);
                    println!("{}", livro);
                }lista_ref_livros
            }
            None => {println!("Livros deste autor não encontrados."); vec![]}
        }
    }
    fn procurar_por_palavras_chave_uniao(&self, palavras_chave:Vec<String>) -> HashSet<u64>{
        let mut lista_isbns = HashSet::new();
        for palavra in palavras_chave.iter(){
            lista_isbns = match self.isbn_por_palavra_chave.get(palavra){
                Some(lista_isbns_palavra) => lista_isbns.union(lista_isbns_palavra).cloned().collect(),
                None => lista_isbns,
            };
        }lista_isbns
    }
    fn procurar_por_palavras_chave_inter(&self, palavras_chave: Vec<String>) -> HashSet<u64>{
        let mut lista_isbns = HashSet::new();
        for palavra in palavras_chave.iter(){
            if lista_isbns.is_empty() {
                lista_isbns = match self.isbn_por_palavra_chave.get(palavra) {
                    None => {lista_isbns}
                    Some(lista_isbns_palavra) => {lista_isbns_palavra.clone()}
                };
            }else{
                lista_isbns = match self.isbn_por_palavra_chave.get(palavra){
                    None => {lista_isbns}
                    Some(lista_isbns_palavra) => {lista_isbns.intersection(lista_isbns_palavra).cloned().collect()}
                }
            }
        }lista_isbns
    }
}
impl fmt::Display for Livro{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{Título: {}, autor: {}, requisitado: {}, ISBN: {}, Palavras-chave: {:?}}}", self.titulo, self.autor, self.requisitado, self.isbn, self.palavras_chave)
    }
}
impl fmt::Display for Livraria {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Livraria: {{\n")?;
        for (_isbn,livro) in self.livros_por_isbn.iter() {
            write!(f, "{}\n", livro)?;
        }
        write!(f, "}}")
    }
}

