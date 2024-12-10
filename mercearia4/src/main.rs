use std::collections::HashMap;
use std::fmt;
#[derive(Debug, Clone)]
struct Produto{
    identificador: String,
    nome: String,
    validade: String,
    preço: f64,
    quantidade: u32
}
#[derive(Debug, Clone)]
struct Livro{
    identificador: String,
    nome: String,
    autor: String,
    preço: f64,
    quantidade: u32
}
trait Artigo{
    fn identificador(&self) -> &String;
    fn nome(&mut self) -> &mut String;
    fn preço(&mut self) -> &mut f64;
    fn quantidade(&mut self) -> &mut u32;
}
#[derive(Debug)]
struct Zona<A>{
    id: usize,
    lista_artigos: HashMap<String, A>, //identificador de Artigo -> Artigo
}
#[derive(Debug)]
struct Prateleira<A>{
    id: usize,
    lista_zonas: Vec<Zona<A>>,
    artigos_por_zona: HashMap<String, usize>, //identificador de Produto -> id Zona
}
#[derive(Debug)]
struct Fileira<A>{
    id: usize,
    lista_prateleiras: Vec<Prateleira<A>>,
    artigos_por_prateleira: HashMap<String, usize>, //id. Produto -> id Prateleira
}
#[derive(Debug)]
struct Mercearia<A>{
    lista_fileira: Vec<Fileira<A>>,
    artigos_por_fileira: HashMap<String, usize>, //id. Produto -> id Fileira
}

fn main() {
    let presunto = Produto{
        identificador: String::from("PRE"),
        nome: String::from("Presunto"),
        validade: String::from("01-01-2025"),
        preço: 5.0,
        quantidade: 20
    };
    let queijo = Produto{
        identificador: String::from("QUE"),
        nome: String::from("Queijo"),
        validade: String::from("01-01-2025"),
        preço: 4.0,
        quantidade: 20
    };
    let livro1 = Livro{
        identificador: String::from("9781593278281"),
        nome: String::from("The Rust Programming Language"),
        autor: String::from("Klabnik"),
        preço: 3.99,
        quantidade: 10
    };

    /*let mut zona_teste = Zona{id: 0, lista_artigos: HashMap::new()};
    println!("{:?}", zona_teste);
    println!("{:?}", zona_teste.tem_artigo("QUE"));
    zona_teste.inserir_artigo(queijo.clone());
    println!("{:?}", zona_teste);
    println!("{:?}", zona_teste.tem_artigo("QUE"));
    zona_teste.inserir_artigo(queijo);

    let mut zona_teste2 = Zona{id: 0, lista_artigos: HashMap::new()};
    zona_teste2.inserir_artigo(livro1);
    println!("{:?}", zona_teste2);
    let livro_removido = zona_teste2.remover_artigo("9781593278281");
    println!("{:?}", livro_removido);
    println!("{:?}", zona_teste2);*/

    println!("----Criar Mercearia<Produto> e Mercearia<Livro> e adicionar produtos----");
    let mut mercearia: Mercearia<Produto> = Mercearia::new(2, 3, 3);
    let mut livraria: Mercearia<Livro> = Mercearia::new(2, 3, 3);
    println!("Resultado de tentar adicionar Queijo à mercearia: {:?}", mercearia.inserir_artigo(queijo.clone(), 1, 2, 0));
    println!("Resultado de tentar adicionar Presunto à mercearia: {:?}",mercearia.inserir_artigo(presunto, 0, 1, 2));
    println!("Resultado de voltar a tentar adicionar Queijo à mercearia: {:?}", mercearia.inserir_artigo(queijo, 1, 2, 0));
    println!("{}", mercearia);
    println!("Resultado de tentar adicionar livro1 à livraria: {:?}", livraria.inserir_artigo(livro1, 1, 2, 2));
    println!("{}", livraria);

    println!("----Procurar se artigo existe em certa região e se sim onde, a partir do identificador----");
    println!("Mercearia tem Queijo da Serra (QUE)? {:?}", mercearia.tem_artigo("QUE"));
    println!("Livraria tem The Rust Programming Language (9781593278281)? {:?}", livraria.tem_artigo("9781593278281"));

    println!("----Remover artigo----");
    println!("Resultado de tentar remover queijo: {:?}", mercearia.remover_artigo("QUE"));
    println!("Resultado de voltar a tentar remover queijo: {:?}", mercearia.remover_artigo("QUE"));
    println!("Mercearia tem Queijo da Serra (QUE)? {:?}", mercearia.tem_artigo("QUE"));

    println!("----Mudar nome de produto----");
    println!("Resultado de tentar mudar nome do Presunto: {:?}", mercearia.mudar_nome("PRE", "Presunto Ibérico"));
    println!("{}", mercearia.obter_zona(0, 1, 2));
    println!("Resultado de tentar mudar nome de Queijo: {:?}", mercearia.mudar_nome("QUE", "Queijo de Cabra"));

    println!("----Mudar preço de produto----");
    println!("Resultado de tentar mudar preço do livro1: {:?}", livraria.mudar_preço("9781593278281", 5.99));
    println!("{}", livraria.obter_zona(1, 2, 2));
    println!("Resultado de tentar mudar preço de Queijo: {:?}", mercearia.mudar_preço("QUE", 5.99));

    println!("----Mover produto----");
    println!("Resultado de tentar mover livro1: {:?}", livraria.mover_artigo("9781593278281", 1, 1, 1));();
    println!("Livro (9781593278281) existe? Onde? {:?}.", livraria.tem_artigo("9781593278281"));

    println!("----Adicionar quantidade----");
    println!("Resultado de tentar adicionar quantidade de Presunto: {:?}", mercearia.adicionar_quantidade("PRE", 5));
    println!("{}", mercearia.obter_zona(0, 1, 2));

    println!("----Remover quantidade----");
    println!("Resultado de tentar remover quantidade de Presunto: {:?}", mercearia.remover_quantidade("PRE", 20));
    println!("{}", mercearia.obter_zona(0, 1, 2));
    println!("Resultado de tentar remover quantidade de Presunto: {:?}",mercearia.remover_quantidade("PRE", 20));
    println!("{}", mercearia.obter_zona(0, 1, 2));
    println!("Resultado de tentar remover quantidade de Queijo: {:?}",mercearia.remover_quantidade("QUE", 20));
}

impl Artigo for Produto {
    fn identificador(&self) -> &String {
        &self.identificador
    }
    fn nome(&mut self) -> &mut String {
        &mut self.nome
    }
    fn preço(&mut self) -> &mut f64 {
        &mut self.preço
    }
    fn quantidade(&mut self) -> &mut u32 {
        &mut self.quantidade
    }
}
impl Artigo for Livro {
    fn identificador(&self) -> &String {
        &self.identificador
    }
    fn nome(&mut self) -> &mut String {
        &mut self.nome
    }
    fn preço(&mut self) -> &mut f64 {
        &mut self.preço
    }
    fn quantidade(&mut self) -> &mut u32 {
        &mut self.quantidade
    }
}
impl <A: Artigo> Zona<A> {
    fn tem_artigo(&self, id_artigo: &str) -> bool{
        self.lista_artigos.contains_key(id_artigo)
    }
    fn inserir_artigo(&mut self, artigo: A){
        if self.tem_artigo(&artigo.identificador()){
            println!("Este artigo já existe. Operação cancelada.");
            return;
        }else{
            self.lista_artigos.insert(artigo.identificador().clone(), artigo);
        }
    }
    fn remover_artigo(&mut self, id_artigo: &str) -> Option<A>{
        match self.lista_artigos.remove_entry(id_artigo) {
            None => {println!("Artigo não existe nesta zona. Operação cancelada."); None}
            Some((_id, artigo)) => {Some(artigo)}
        }
    }
}
impl <A: Artigo> Prateleira<A> {
    fn new (n_zona: usize, id: usize) -> Prateleira<A>{
        let mut lista_zonas = vec![];
        for i in 0..n_zona{
            lista_zonas.push(Zona{id: i, lista_artigos: HashMap::new()});
        }
        Prateleira{id, lista_zonas, artigos_por_zona: HashMap::new()}
    }
    fn tem_artigo(&self, id_art: &str) -> Option<usize>{
        self.artigos_por_zona.get(id_art).copied()
    }
    fn remover_artigo(&mut self, id_art: &str) -> Option<A>{
        match self.artigos_por_zona.remove(id_art) {
            None => {println!("Artigo não existe nesta prateleira. Operação cancelada."); None}
            Some(id_zona) => {self.lista_zonas[id_zona].remover_artigo(id_art)}
        }
    }
}
impl <A: Artigo> Fileira<A>{
    fn new (n_prateleira: usize, n_zona: usize, id: usize) -> Fileira<A>{
        let mut lista_prateleiras = vec![];
        for i in 0..n_prateleira{
            lista_prateleiras.push(Prateleira::new(n_zona, i));
        }
        Fileira{id, lista_prateleiras, artigos_por_prateleira: HashMap::new()}
    }
    fn tem_artigo(&self, id_art: &str) -> Option<[usize; 2]>{
        match self.artigos_por_prateleira.get(id_art) {
            None => None,
            Some(&id_prateleira) => {
                let id_zona = self.lista_prateleiras[id_prateleira].tem_artigo(id_art).unwrap();
                Some([id_prateleira, id_zona])}
        }
    }
    fn remover_artigo(&mut self, id_art: &str) -> Option<A>{
        match self.artigos_por_prateleira.remove(id_art) {
            None => {println!("Produto não existe nesta fileira. Operação cancelada."); None}
            Some(id_prateleira) => {self.lista_prateleiras[id_prateleira].remover_artigo(id_art)}
        }
    }
}
impl <A: Artigo> Mercearia<A> {
    fn new(n_fileira: usize, n_prateleira: usize, n_zona: usize) -> Mercearia<A>{
        let mut lista_fileira = vec![];
        for i in 0..n_fileira{
            lista_fileira.push(Fileira::new(n_prateleira, n_zona, i));
        }
        Mercearia{lista_fileira, artigos_por_fileira: HashMap::new()}
    }
    fn obter_zona(&mut self, id_fileira: usize, id_prateleira: usize, id_zona: usize) -> &mut Zona<A>{
        &mut self.lista_fileira[id_fileira]
            .lista_prateleiras[id_prateleira]
            .lista_zonas[id_zona]
    }
    fn obter_prateleira(&mut self, id_fileira: usize, id_prateleira: usize) -> &mut Prateleira<A>{
        &mut self.lista_fileira[id_fileira]
            .lista_prateleiras[id_prateleira]
    }
    fn obter_fileira(&mut self, id_fileira: usize) -> &mut Fileira<A>{
        &mut self.lista_fileira[id_fileira]
    }
    fn tem_artigo(&self, id_art: &str) -> Option<[usize; 3]>{
        match self.artigos_por_fileira.get(id_art) {
            None => {None},
            Some(&id_fileira) => {
                let [id_prateleira, id_zona] = self.lista_fileira[id_fileira].tem_artigo(id_art).unwrap();
                Some([id_fileira, id_prateleira, id_zona])}
        }
    }
    fn inserir_artigo(&mut self, art: A, id_fileira: usize, id_prateleira: usize, id_zona: usize) -> Result<String,String>{
        match self.tem_artigo(&art.identificador()){
            None => {
                self.artigos_por_fileira.insert(art.identificador().clone(), id_fileira);
                self.obter_fileira(id_fileira).artigos_por_prateleira.insert(art.identificador().clone(), id_prateleira);
                self.obter_prateleira(id_fileira, id_prateleira).artigos_por_zona.insert(art.identificador().clone(), id_zona);
                self.obter_zona(id_fileira, id_prateleira, id_zona).inserir_artigo(art);
                Ok(String::from("Sucesso"))
            },
            Some([id_fil_existe, id_prat_existe, id_zona_existe]) => {
                Err(format!("Artigo já existe na mercearia na Fileira {}, Prateleira {}, Zona {}. Operação cancelada.", id_fil_existe, id_prat_existe, id_zona_existe))
            }
        }
    }
    fn remover_artigo(&mut self, id_prod: &str) -> Result<A, String>{
        match self.artigos_por_fileira.remove(id_prod) {
            None => {Err(String::from("Produto não existe nesta mercearia. Operação cancelada."))}
            Some(id_fileira) => {Ok(self.lista_fileira[id_fileira].remover_artigo(id_prod).unwrap())}
        }
    }
    fn mudar_nome(&mut self, id_art: &str, novo_nome: &str) -> Result<String, String>{
        match self.tem_artigo(id_art) {
            None => {Err(format!("Artigo {} não existe na mercearia, por isso não pode mudar de nome.", id_art))},
            Some([id_fileira, id_prateleira, id_zona]) => {
                let ref_art = self.obter_zona(id_fileira, id_prateleira, id_zona).lista_artigos.get_mut(id_art).unwrap();
                *ref_art.nome() = novo_nome.to_string();
                Ok(String::from("Sucesso"))
            }
        }
    }
    fn mudar_preço(&mut self, id_art: &str, novo_preço: f64) -> Result<String, String>{
        match self.tem_artigo(id_art) {
            None => {Err(format!("Produto {} não existe na mercearia, por isso não pode mudar de nome.", id_art))},
            Some([id_fileira, id_prateleira, id_zona]) => {
                let ref_art = self.obter_zona(id_fileira, id_prateleira, id_zona).lista_artigos.get_mut(id_art).unwrap();
                *ref_art.preço() = novo_preço;
                Ok(String::from("Sucesso"))
            }
        }
    }
    fn mover_artigo(&mut self, id_art: &str, nova_fileira: usize, nova_prateleira: usize, nova_zona: usize) -> Result<String, String>{
        match self.remover_artigo(id_art) {
            Err(_e) => {Err(format!("Produto {} não existe na mercearia, por isso não pode ser movido.", id_art))}
            Ok(prod) => {Ok(self.inserir_artigo(prod, nova_fileira, nova_prateleira, nova_zona)?)}
        }
    }
    fn adicionar_quantidade(&mut self, id_art: &str, adic: u32) -> Result<String, String>{
        match self.tem_artigo(id_art) {
            None => {Err(format!("Produto {} não existe na mercearia, por isso não pode ser adicionada quantidade.", id_art))},
            Some([id_fileira, id_prateleira, id_zona]) => {
                let ref_prod = self.obter_zona(id_fileira, id_prateleira, id_zona).lista_artigos.get_mut(id_art).unwrap();
                *ref_prod.quantidade() += adic;
                Ok(String::from("Sucesso"))
            }
        }
    }
    fn remover_quantidade(&mut self, id_art: &str, remov: u32) -> Result<String, String>{
        match self.tem_artigo(id_art) {
            None => { Err(format!("Produto {} não existe na mercearia, por isso não pode ser removida quantidade.", id_art)) },
            Some([id_fileira, id_prateleira, id_zona]) => {
                let ref_prod = self.obter_zona(id_fileira, id_prateleira, id_zona).lista_artigos.get_mut(id_art).unwrap();
                if remov <= *ref_prod.quantidade() {
                    *ref_prod.quantidade() -= remov;
                    Ok(String::from("Sucesso"))
                } else {
                    Err(String::from("Não se pode remover maior quantidade do que aquela que existe. Operação cancelada."))
                }
            }
        }
    }
}
impl fmt::Display for Produto{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} -> {}, val: {}, pr: {}€, qt: {}]", self.identificador, self.nome, self.validade, self.preço, self.quantidade)
    }
}
impl fmt::Display for Livro{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} -> {}, autor: {}, pr: {}€, qt: {}]", self.identificador, self.nome, self.autor, self.preço, self.quantidade)
    }
}
impl <A:Artigo + fmt::Display> fmt::Display for Zona<A>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Zona {}: {{", self.id)?;
        for (_key, produto) in &self.lista_artigos{
            write!(f, " {} ", produto)?;
        }write!(f,"}}")
    }
}
impl <A:Artigo + fmt::Display> fmt::Display for Prateleira<A>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Prateleira {}:", self.id)?;
        for zona in &self.lista_zonas{
            write!(f, "\n  {}", zona)?;
        }Ok(())
    }
}
impl <A:Artigo + fmt::Display> fmt::Display for Fileira<A>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fileira {}:", self.id)?;
        for prateleira in &self.lista_prateleiras{
            write!(f, "\n {}", prateleira)?;
        }Ok(())
    }
}
impl  <A:Artigo + fmt::Display> fmt::Display for Mercearia<A>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mercearia:")?;
        for fileira in &self.lista_fileira{
            write!(f, "\n{}", fileira)?;
        }Ok(())
    }
}
