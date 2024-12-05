use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
#[derive(Debug)]
enum Objeto{
    Livro {
        titulo: String,
        autor: String,
        requisitado: bool,
        isbn: u64,
        palavras_chave: Vec<String>,
    },
    AudioLivro {
        titulo: String,
        autor: String,
        requisitado: bool,
        isbn: u64,
        palavras_chave: Vec<String>,
        duracao: u32,
    },
    Estatua {
        titulo: String,
        autor: String,
        id: u64,
        palavras_chave: Vec<String>,
        dimensoes: [f32; 3],
    },
    Quadro {
        titulo: String,
        autor: String,
        id: u64,
        palavras_chave: Vec<String>,
    }
}
#[derive(Debug)]
struct Livraria{
    objeto_por_id: HashMap<u64, Objeto>,
    id_por_titulo: HashMap<String, HashSet<u64>>, //HashSet pois pode existir livro e audio livro
    id_por_autor: HashMap<String, HashSet<u64>>,
    id_por_palavra_chave: HashMap<String, HashSet<u64>>,
    id_por_tipo: HashMap<String, HashSet<u64>>,
}
fn main() {
    let mut livraria = Livraria::new();
    let livro1 = Objeto::Livro{
        titulo:String::from("Os Maias"),
        autor:String::from("Eça de Queirós"),
        requisitado:false,
        isbn: 978_972_0_04957_5,
        palavras_chave: vec!["português".to_string(), "romance".to_string()],
    };
    let livro2 = Objeto::Livro{
        titulo:String::from("Memorial do Convento"),
        autor:String::from("José Saramago"),
        requisitado:false,
        isbn: 978_972_0_04671_0,
        palavras_chave: vec!["português".to_string(), "romance".to_string()],
    };
    let livro3 = Objeto::Livro{
        titulo:String::from("A Cidade e as Serras"),
        autor:String::from("Eça de Queirós"),
        requisitado:false,
        isbn: 978_972_38_2949_5,
        palavras_chave: vec!["português".to_string(), "romance".to_string()],
    };
    let livro4 = Objeto::Livro{
        titulo: String::from("Guerra e Paz"),
        autor: String::from("Lev Tolstoi"),
        requisitado: false,
        isbn: 978_972_23_3334_4,
        palavras_chave: vec!["português".to_string(), "tradução".to_string(), "romance".to_string()],
    };
    let livro5 = Objeto::Livro{
        titulo: String::from("Hamlet"),
        autor: String::from("William Shakespeare"),
        requisitado: false,
        isbn: 978_972_20_5996_1,
        palavras_chave: vec!["português".to_string(), "tradução".to_string(), "teatro".to_string(), "renascença".to_string()],
    };
    let audio_livro1 = Objeto::AudioLivro{
        titulo:String::from("Os Maias"),
        autor:String::from("Eça de Queirós"),
        requisitado:false,
        isbn: 978_917_875992_7,
        palavras_chave: vec!["português".to_string(), "romance".to_string()],
        duracao:26,
    };
    let audio_livro2 = Objeto::AudioLivro{
        titulo:String::from("Fellowship of the Ring (Lord of the Rings, Book 1)"),
        autor:String::from("J.R.R. Tolkien"),
        requisitado:false,
        isbn: 978_078878981_6,
        palavras_chave: vec!["inglês".to_string(), "fantasia".to_string()],
        duracao:19,
    };
    let estatua1 = Objeto::Estatua {
        titulo: String::from("David"),
        autor: String::from("Miguel Ângelo"),
        id: 100,
        palavras_chave: vec!["renascença".to_string()],
        dimensoes: [1.5, 1.5, 5.0],
    };
    let estatua2 = Objeto::Estatua {
        titulo: String::from("O Pensador"),
        autor: String::from("Auguste Rodin"),
        id: 101,
        palavras_chave: vec!["realismo".to_string()],
        dimensoes: [1.5, 1.5, 3.0],
    };
    let quadro1 = Objeto::Quadro {
        titulo: String::from("A Persistência da Memória"),
        autor: String::from("Salvador Dali"),
        id: 200,
        palavras_chave: vec!["surrealismo".to_string()],
    };
    let quadro2 = Objeto::Quadro {
        titulo: String::from("Impressão, Nascer do Sol"),
        autor: String::from("Claude Monet"),
        id: 201,
        palavras_chave: vec!["impressionismo".to_string()],
    };
    let quadro_id_repetido = Objeto::Quadro {
        titulo: String::from("Guernica"),
        autor: String::from("Pablo Picasso"),
        id: 200,
        palavras_chave: vec!["cubismo".to_string()],
    };

    println!("---Tentar aceder aos campos dimensoes e duracao de Objetos---");
    println!("Duração do livro1: {:?}", livro1.duracao());
    println!("Duração do audio_livro1: {:?}", audio_livro1.duracao());
    println!("Dimensões do livro1: {:?}", livro1.dimensoes());
    println!("Dimensões da estatua1: {:?}", estatua1.dimensoes());

    println!("--------Adicionar Objetos (Livros, Audio Livros, Estátuas e Quadros)-------");
    livraria.adicionar(livro1);
    livraria.adicionar(livro2);
    livraria.adicionar(livro3);
    livraria.adicionar(livro4);
    livraria.adicionar(livro5);
    livraria.adicionar(audio_livro1);
    livraria.adicionar(audio_livro2);
    livraria.adicionar(estatua1);
    livraria.adicionar(estatua2);
    livraria.adicionar(quadro1);
    livraria.adicionar(quadro2);
    livraria.adicionar(quadro_id_repetido); //adicionar um objeto com o id repetido (não vai ser permitido)
    println!("{}", livraria);
    //println!("{:?}", livraria.id_por_titulo);
    //println!("{:?}", livraria.id_por_autor);
    //println!("{:?}", livraria.id_por_palavra_chave);
    //println!("{:?}", livraria.id_por_tipo);

    println!("--------Remover Livro-------");
    let livro_removido = livraria.remover(978_972_0_04671_0).unwrap(); //Memorial do convento
    println!("{}", livraria);
    println!("Livro removido: {}", livro_removido);
    livraria.remover(1); //Tentativa de remover objeto que não existe

    println!("--------Requisitar Livro-------");
    livraria.requisitar_livro(978_917_875992_7); //Audio Livro "Os Maias"
    println!("{}", livraria.procurar_por_id(978_917_875992_7).unwrap());
    livraria.requisitar_livro(1); //Tentativa de requisitar objeto que não existe
    livraria.requisitar_livro(100); //Tentativa de requisitar Estátua

    println!("--------Devolver Livro-------");
    livraria.devolver_livro(978_917_875992_7); //Audio Livro "Os Maias"
    println!("{}", livraria.procurar_por_id(978_917_875992_7).unwrap());

    println!("--------Procurar Objeto por ID-------");
    match livraria.procurar_por_id(101) {
        None => {println!("Objeto não encontrado.")}
        Some(ref_obj) => {println!("{}", ref_obj);}
    };
    match livraria.procurar_por_id(1) {
        None => {println!("Objeto não encontrado.")}
        Some(ref_obj) => {println!("{}", ref_obj);}
    };

    println!("---------Procurar Objeto por Título-------");
    livraria.procurar_por_titulo("Os Maias");
    livraria.procurar_por_titulo("David");
    livraria.procurar_por_titulo("Não existe");

    println!("----------Procurar Objeto por Autor-------");
    livraria.procurar_por_autor("Eça de Queirós");
    livraria.procurar_por_autor("Zé Ninguém");

    println!("----------Procurar Objetos por Tipo-------");
    livraria.procurar_por_tipo("Estatua");
    livraria.procurar_por_tipo("Ebook");

    println!("----Procurar por união de palavras-chave (retorna todos os objetos que têm pelo menos uma das palavras-chave dadas)----");
    let lista_ids = livraria.procurar_por_palavras_chave_uniao(vec!["português".to_string()]);
    //println!("{:?}",lista_ids);
    println!("Objetos que têm a palavra-chave: 'português':");
    for isbn in lista_ids{
        println!("{}", livraria.objeto_por_id.get(&isbn).unwrap());
    }
    let lista_ids = livraria.procurar_por_palavras_chave_uniao(vec!["renascença".to_string(), "surrealismo".to_string()]);
    //println!("{:?}",lista_ids);
    println!("Objetos que têm a palavra-chave: 'renascença' e/ou 'surrealismo':");
    for isbn in lista_ids{
        println!("{}", livraria.objeto_por_id.get(&isbn).unwrap());
    }

    println!("----Procurar por interseção de palavras-chave (retorna todos os objetos que têm todas palavras-chave dadas)----");
    let lista_ids = livraria.procurar_por_palavras_chave_inter(vec!["português".to_string(), "tradução".to_string()]);
    //println!("{:?}",lista_isbns);
    println!("Objetos que têm a palavra-chave: 'português' e 'tradução':");
    for id in lista_ids{
        println!("{}", livraria.objeto_por_id.get(&id).unwrap());
    }
    let lista_ids = livraria.procurar_por_palavras_chave_inter(vec!["português".to_string(), "romance".to_string()]);
    //println!("{:?}",lista_isbns);
    println!("Objetos que têm a palavra-chave: 'português' e 'romance':");
    for id in lista_ids{
        println!("{}", livraria.objeto_por_id.get(&id).unwrap());
    }

}
impl Objeto{
    fn id(&self) -> u64{
        match self{
            Objeto::Livro {isbn, .. } => *isbn,
            Objeto::AudioLivro {isbn, .. } => *isbn,
            Objeto::Estatua {id, .. } => *id,
            Objeto::Quadro {id, .. } => *id,
        }
    }
    fn titulo(&self) -> &String{
        match self{
            Objeto::Livro {titulo, .. } => titulo,
            Objeto::AudioLivro {titulo, .. } => titulo,
            Objeto::Estatua {titulo, .. } => titulo,
            Objeto::Quadro {titulo, .. } => titulo,
        }
    }
    fn autor(&self) -> &String{
        match self{
            Objeto::Livro {autor, .. } => autor,
            Objeto::AudioLivro {autor, .. } => autor,
            Objeto::Estatua {autor, .. } => autor,
            Objeto::Quadro {autor, .. } => autor,
        }
    }
    fn palavras_chave(&self) -> &Vec<String>{
        match self{
            Objeto::Livro {palavras_chave, .. } => palavras_chave,
            Objeto::AudioLivro {palavras_chave, .. } => palavras_chave,
            Objeto::Estatua {palavras_chave, .. } => palavras_chave,
            Objeto::Quadro {palavras_chave, .. } => palavras_chave,
        }
    }
    fn duracao(&self) -> Result<u32, String> { //esta função só "funciona" com AudioLivro
        match self{
            Objeto::AudioLivro {duracao, ..} => {Ok(*duracao)},
            Objeto::Livro{..} | Objeto::Estatua{..} | Objeto::Quadro{..} => {Err(String::from("Este objeto não tem o campo duração implementado"))},
        }
    }
    fn dimensoes(&self) -> Result<&[f32; 3], String> { //esta função só "funciona" com Estátua
        match self{
            Objeto::Estatua {dimensoes, ..} => {Ok(dimensoes)},
            Objeto::Livro{..} | Objeto::AudioLivro{..} | Objeto::Quadro{..} => {Err(String::from("Este objeto não tem o campo dimensões implementado"))}
        }
    }
    fn tipo(&self) -> String {
        match self {
            Objeto::Livro {..} => "Livro".to_string(),
            Objeto::AudioLivro {..} => "AudioLivro".to_string(),
            Objeto::Quadro {..} => "Quadro".to_string(),
            Objeto::Estatua {..} => "Estatua".to_string(),
        }
    }
}
impl Livraria{
    fn new() -> Livraria{
        Livraria{
            objeto_por_id: HashMap::new(),
            id_por_titulo: HashMap::new(),
            id_por_autor: HashMap::new(),
            id_por_palavra_chave: HashMap::new(),
            id_por_tipo: HashMap::from(
                [("Livro".to_string(), HashSet::new()),
                    ("AudioLivro".to_string(), HashSet::new()),
                    ("Quadro".to_string(), HashSet::new()),
                    ("Estatua".to_string(), HashSet::new())]
            )
        }
    }
    fn adicionar(&mut self, objeto: Objeto){
        if self.objeto_por_id.contains_key(&objeto.id()){
            println!("Não pode haver dois objetos com o mesmo ID. Operação cancelada.");
            return
        }
        self.id_por_tipo.get_mut(&objeto.tipo()).unwrap().insert(objeto.id());
        match self.id_por_titulo.get_mut(objeto.titulo()){
            Some(lista) => {lista.insert(objeto.id());} //titulo já existe na base de dados
            None => {self.id_por_titulo.insert(objeto.titulo().clone(), HashSet::from([objeto.id()]));}
        }
        match self.id_por_autor.get_mut(objeto.autor()){
            Some(lista) => {lista.insert(objeto.id());} //autor já existe na base de dados
            None => {self.id_por_autor.insert(objeto.autor().clone(), HashSet::from([objeto.id()]));}
        }
        for palavra in objeto.palavras_chave(){
            match self.id_por_palavra_chave.get_mut(palavra){
                Some(hashset) => {hashset.insert(objeto.id());} //palavra-chave já existe
                None => {self.id_por_palavra_chave.insert(palavra.clone(), HashSet::from([objeto.id()]));}
            }
        }
        self.objeto_por_id.insert(objeto.id(), objeto);
    }
    fn remover(&mut self, id: u64) -> Option<Objeto>{
        let objeto = match self.objeto_por_id.remove(&id){
            Some(objeto) => objeto,
            None => {println!("Objeto com id {} não existe na livraria. Nada foi removido.", id); return None}
        };
        self.id_por_tipo.get_mut(&objeto.tipo()).unwrap().remove(&id);
        self.id_por_titulo.get_mut(objeto.titulo()).unwrap().remove(&id);
        self.id_por_autor.get_mut(objeto.autor()).unwrap().remove(&id);
        for palavra in objeto.palavras_chave(){
            self.id_por_palavra_chave.get_mut(palavra).unwrap().remove(&id);
        }
        Some(objeto)
    }
    fn requisitar_livro(&mut self, isbn: u64) {
        match self.objeto_por_id.get_mut(&isbn) {
            Some(Objeto::Livro {requisitado, ..}) => {*requisitado=true}
            Some(Objeto::AudioLivro {requisitado, ..}) => {*requisitado=true}
            None => {println!("Objecto com id {} não existe na livraria, por isso não pode ser requisitado", isbn)}
            _ => {println!("Este tipo de objeto não pode ser requisitado. Operação cancelada")}
        }
    }
    fn devolver_livro(&mut self, isbn: u64) {
        match self.objeto_por_id.get_mut(&isbn) {
            Some(Objeto::Livro {requisitado, ..}) => {*requisitado=false}
            Some(Objeto::AudioLivro {requisitado, ..}) => {*requisitado=false}
            None => {println!("Objecto com id {} não existe na livraria, por isso não pode ser requisitado", isbn)}
            _ => {println!("Este tipo de objeto não pode ser requisitado. Operação cancelada")}
        }
    }
    fn procurar_por_id(&self, id: u64) -> Option<&Objeto> {
        self.objeto_por_id.get(&id)
    }
    fn procurar_por_titulo(&self, titulo:&str) -> Option<Vec<&Objeto>> {
        match self.id_por_titulo.get(titulo){
            Some(lista) => {
                println!("Objetos com o título {}:", titulo);
                let mut lista_ref_obj = vec![];
                for id in lista{
                    let obj = self.objeto_por_id.get(id).unwrap();
                    lista_ref_obj.push(obj);
                    println!("{}", obj)
                }Some(lista_ref_obj)
            }
            None => {println!("Não existem objetos com este título"); None}
        }
    }
    fn procurar_por_autor(&self, autor:&str) -> Option<Vec<&Objeto>> {
        match self.id_por_autor.get(autor){
            Some(lista) => {
                println!("Objetos do autor {}:", autor);
                let mut lista_ref_obj = vec![];
                for id in lista{
                    let obj = self.objeto_por_id.get(id).unwrap();
                    lista_ref_obj.push(obj);
                    println!("{}", obj)
                }Some(lista_ref_obj)
            }
            None => {println!("Não existem objetos para este autor"); None}
        }
    }
    fn procurar_por_tipo(&self, tipo:&str) -> Option<Vec<&Objeto>> {
        match self.id_por_tipo.get(tipo){
            Some(lista) => {
                println!("Objetos do tipo {}:", tipo);
                let mut lista_ref_obj = vec![];
                for id in lista{
                    let obj = self.objeto_por_id.get(id).unwrap();
                    lista_ref_obj.push(obj);
                    println!("{}", obj)
                }Some(lista_ref_obj)
            }
            None => {println!("Este tipo não existe na livraria"); None}
        }
    }
    fn procurar_por_palavras_chave_uniao(&self, palavras_chave:Vec<String>) -> HashSet<u64>{
        let mut lista_ids = HashSet::new();
        for palavra in palavras_chave{
            lista_ids = match self.id_por_palavra_chave.get(&palavra) {
                Some(lista_ids_palavra) => lista_ids.union(lista_ids_palavra).cloned().collect(),
                None => lista_ids
            };
        }lista_ids
    }
    fn procurar_por_palavras_chave_inter(&self, palavras_chave: Vec<String>) -> HashSet<u64> {
        let mut lista_ids = HashSet::new();
        for palavra in palavras_chave{
            if lista_ids.is_empty(){
                lista_ids = match self.id_por_palavra_chave.get(&palavra) {
                    None => lista_ids,
                    Some(lista_ids_palavra) => lista_ids_palavra.clone(),
                }
            }else {
                lista_ids = match self.id_por_palavra_chave.get(&palavra){
                    None => {lista_ids}
                    Some(lista_ids_palavra) => {lista_ids.intersection(lista_ids_palavra).cloned().collect()}
                }
            }
        }lista_ids
    }
}
impl fmt::Display for Objeto{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Objeto::Livro {titulo, autor, requisitado, isbn, palavras_chave} => {
                write!(f, "Livro: {{Título: {}, autor: {}, requisitado: {}, ISBN: {}, Palavras-chave: {:?}}}", titulo, autor, requisitado, isbn, palavras_chave)
            }
            Objeto::AudioLivro {titulo, autor, requisitado, isbn, palavras_chave, duracao} => {
                write!(f, "Audio Livro: {{Título: {}, autor: {}, requisitado: {}, ISBN: {}, Palavras-chave: {:?}, duração: {}h}}", titulo, autor, requisitado, isbn, palavras_chave, duracao)
            }
            Objeto::Estatua {titulo, autor, id, palavras_chave, dimensoes} => {
                write!(f, "Estatua: {{Título: {}, autor: {}, id: {}, Palavras-chave: {:?}, dimensões: {:?}}}", titulo, autor, id, palavras_chave, dimensoes)
            }
            Objeto::Quadro {titulo, autor, id, palavras_chave} => {
                write!(f, "Quadro: {{Título: {}, autor: {}, id: {}, Palavras-chave: {:?}}}", titulo, autor, id, palavras_chave)
            }
        }

    }
}
impl fmt::Display for Livraria {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Livraria: {{\n")?;
        for (_isbn,objeto) in self.objeto_por_id.iter() {
            write!(f, "{}\n", objeto)?;
        }
        write!(f, "}}")
    }
}