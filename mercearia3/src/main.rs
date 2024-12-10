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
#[derive(Debug)]
struct Zona{
    id: usize,
    lista_produtos: HashMap<String, Produto>, //identificador de Produto -> Produto
}
#[derive(Debug)]
struct Prateleira{
    id: usize,
    lista_zonas: Vec<Zona>,
    produtos_por_zona: HashMap<String, usize>, //identificador de Produto -> id Zona
}
#[derive(Debug)]
struct Fileira{
    id: usize,
    lista_prateleiras: Vec<Prateleira>,
    produtos_por_prateleira: HashMap<String, usize>, //id. Produto -> id Prateleira
}
#[derive(Debug)]
struct Mercearia{
    lista_fileira: Vec<Fileira>,
    produtos_por_fileira: HashMap<String, usize>, //id. Produto -> id Fileira
}

fn main() {
    let presunto = Produto{
        identificador: String::from("PRE"),
        nome: String::from("Presunto"),
        validade: String::from("01-01-2025"),
        preço: 5.0,
        quantidade: 20
    };
    let mut queijo = Produto{
        identificador: String::from("QUE"),
        nome: String::from("Queijo"),
        validade: String::from("01-01-2025"),
        preço: 4.0,
        quantidade: 20
    };

    println!("----Mudar nome e preço de produto----");
    queijo.mudar_preço(4.5);
    queijo.mudar_nome("Queijo da Serra");
    println!("{:?}", queijo);

    /*let mut zona_teste = Zona{id: 0, lista_produtos: HashMap::new()};
    println!("{:?}", zona_teste);
    println!("{:?}", zona_teste.tem_produto("QUE"));
    zona_teste.inserir_produto(queijo.clone());
    println!("{:?}", zona_teste);
    println!("{:?}", zona_teste.tem_produto("QUE"));
    zona_teste.inserir_produto(queijo);*/

    println!("----Criar Mercearia e adicionar produtos----");
    let mut mercearia = Mercearia::new(2, 3, 3);
    println!("Resultado de tentar adicionar Queijo: {:?}", mercearia.inserir_produto(queijo.clone(), 1, 2, 0));
    println!("Resultado de tentar adicionar Presunto: {:?}",mercearia.inserir_produto(presunto, 0, 1, 2));
    println!("Resultado de voltar a tentar adicionar Queijo: {:?}", mercearia.inserir_produto(queijo, 1, 2, 0));
    println!("{}", mercearia);

    println!("----Procurar se produto existe em certa região e se sim onde, a partir do identificador----");
    println!("Mercearia tem Queijo da Serra (QUE)? {:?}", mercearia.tem_produto("QUE"));
    println!("Fileira 0 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_fileira(0).tem_produto("QUE"));
    println!("Fileira 1 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_fileira(1).tem_produto("QUE"));
    println!("Prateleira 1 da Fileira 1 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_prateleira(1, 1).tem_produto("QUE"));
    println!("Prateleira 2 da Fileira 1 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_prateleira(1, 2).tem_produto("QUE"));
    println!("Zona 1 da Prateleira 2 da Fileira 1 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_zona(1, 2, 1).tem_produto("QUE"));
    println!("Zona 0 da Prateleira 2 da Fileira 1 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_zona(1, 2, 0).tem_produto("QUE"));

    println!("----Remover produto----");
    println!("Resultado de tentar remover queijo: {:?}", mercearia.remover_produto("QUE"));
    println!("Resultado de voltar a tentar remover queijo: {:?}", mercearia.remover_produto("QUE"));
    println!("Mercearia tem Queijo da Serra (QUE)? {:?}", mercearia.tem_produto("QUE"));

    println!("----Mudar nome de produto----");
    println!("Resultado de tentar mudar nome do Presunto: {:?}", mercearia.mudar_nome("PRE", "Presunto Ibérico"));
    println!("{}", mercearia.obter_zona(0, 1, 2));
    println!("Resultado de tentar mudar nome de Queijo: {:?}", mercearia.mudar_nome("QUE", "Queijo de Cabra"));

    println!("----Mudar preço de produto----");
    println!("Resultado de tentar mudar preço do Presunto: {:?}", mercearia.mudar_preço("PRE", 5.99));
    println!("{}", mercearia.obter_zona(0, 1, 2));
    println!("Resultado de tentar mudar preço de Queijo: {:?}", mercearia.mudar_preço("QUE", 5.99));

    println!("----Mover produto----");
    println!("Resultado de tentar mover Presunto: {:?}", mercearia.mover_produto("PRE", 1, 1, 1));();
    println!("Presunto (PRE) existe? Onde? {:?}.", mercearia.tem_produto("PRE"));

    println!("----Adicionar quantidade----");
    println!("Resultado de tentar adicionar quantidade de Presunto: {:?}", mercearia.adicionar_quantidade("PRE", 5));
    println!("{}", mercearia.obter_zona(1, 1, 1));

    println!("----Remover quantidade----");
    println!("Resultado de tentar remover quantidade de Presunto: {:?}", mercearia.remover_quantidade("PRE", 20));
    println!("{}", mercearia.obter_zona(1, 1, 1));
    println!("Resultado de tentar remover quantidade de Presunto: {:?}",mercearia.remover_quantidade("PRE", 20));
    println!("{}", mercearia.obter_zona(1, 1, 1));
    println!("Resultado de tentar remover quantidade de Queijo: {:?}",mercearia.remover_quantidade("QUE", 20));
}

impl Produto{
    fn mudar_preço(&mut self, novo_preço: f64){
        self.preço = novo_preço;
    }
    fn mudar_nome(&mut self, novo_nome: &str){
        self.nome = novo_nome.to_string();
    }
}
impl Zona {
    fn tem_produto(&self, id_produto: &str) -> bool{
        self.lista_produtos.contains_key(id_produto)
    }
    fn inserir_produto(&mut self, prod: Produto){
        if self.tem_produto(&prod.identificador){
            println!("Este produto já existe. Operação cancelada.");
            return;
        }else{
            self.lista_produtos.insert(prod.identificador.clone(), prod);
        }
    }
    fn remover_produto(&mut self, id_prod: &str) -> Option<Produto>{
        match self.lista_produtos.remove_entry(id_prod) {
            None => {println!("Produto não existe nesta zona. Operação cancelada."); None}
            Some((_id, prod)) => {Some(prod)}
        }
    }
}
impl Prateleira {
    fn new (n_zona: usize, id: usize) -> Prateleira{
        let mut lista_zonas = vec![];
        for i in 0..n_zona{
            lista_zonas.push(Zona{id: i, lista_produtos: HashMap::new()});
        }
        Prateleira{id, lista_zonas, produtos_por_zona: HashMap::new()}
    }
    fn tem_produto(&self, id_prod: &str) -> Option<usize>{
        self.produtos_por_zona.get(id_prod).copied()
    }
    fn remover_produto(&mut self, id_prod: &str) -> Option<Produto>{
        match self.produtos_por_zona.remove(id_prod) {
            None => {println!("Produto não existe nesta prateleira. Operação cancelada."); None}
            Some(id_zona) => {self.lista_zonas[id_zona].remover_produto(id_prod)}
        }
    }
}
impl Fileira{
    fn new (n_prateleira: usize, n_zona: usize, id: usize) -> Fileira{
        let mut lista_prateleiras = vec![];
        for i in 0..n_prateleira{
            lista_prateleiras.push(Prateleira::new(n_zona, i));
        }
        Fileira{id, lista_prateleiras, produtos_por_prateleira: HashMap::new()}
    }
    fn tem_produto(&self, id_prod: &str) -> Option<[usize; 2]>{
        match self.produtos_por_prateleira.get(id_prod) {
            None => None,
            Some(&id_prateleira) => {
                let id_zona = self.lista_prateleiras[id_prateleira].tem_produto(id_prod).unwrap();
                Some([id_prateleira, id_zona])}
        }
    }
    fn remover_produto(&mut self, id_prod: &str) -> Option<Produto>{
        match self.produtos_por_prateleira.remove(id_prod) {
            None => {println!("Produto não existe nesta fileira. Operação cancelada."); None}
            Some(id_prateleira) => {self.lista_prateleiras[id_prateleira].remover_produto(id_prod)}
        }
    }
}
impl Mercearia {
    fn new(n_fileira: usize, n_prateleira: usize, n_zona: usize) -> Mercearia{
        let mut lista_fileira = vec![];
        for i in 0..n_fileira{
            lista_fileira.push(Fileira::new(n_prateleira, n_zona, i));
        }
        Mercearia{lista_fileira, produtos_por_fileira: HashMap::new()}
    }
    fn obter_zona(&mut self, id_fileira: usize, id_prateleira: usize, id_zona: usize) -> &mut Zona{
        &mut self.lista_fileira[id_fileira]
            .lista_prateleiras[id_prateleira]
            .lista_zonas[id_zona]
    }
    fn obter_prateleira(&mut self, id_fileira: usize, id_prateleira: usize) -> &mut Prateleira{
        &mut self.lista_fileira[id_fileira]
            .lista_prateleiras[id_prateleira]
    }
    fn obter_fileira(&mut self, id_fileira: usize) -> &mut Fileira{
        &mut self.lista_fileira[id_fileira]
    }
    fn tem_produto(&self, id_prod: &str) -> Option<[usize; 3]>{
        match self.produtos_por_fileira.get(id_prod) {
            None => {None},
            Some(&id_fileira) => {
                let [id_prateleira, id_zona] = self.lista_fileira[id_fileira].tem_produto(id_prod).unwrap();
                Some([id_fileira, id_prateleira, id_zona])}
        }
    }
    fn inserir_produto(&mut self, prod: Produto, id_fileira: usize, id_prateleira: usize, id_zona: usize) -> Result<String,String>{
        match self.tem_produto(&prod.identificador){
            None => {
                self.produtos_por_fileira.insert(prod.identificador.clone(), id_fileira);
                self.obter_fileira(id_fileira).produtos_por_prateleira.insert(prod.identificador.clone(), id_prateleira);
                self.obter_prateleira(id_fileira, id_prateleira).produtos_por_zona.insert(prod.identificador.clone(), id_zona);
                self.obter_zona(id_fileira, id_prateleira, id_zona).inserir_produto(prod);
                Ok(String::from("Sucesso"))
            },
            Some([id_fil_existe, id_prat_existe, id_zona_existe]) => {
                Err(format!("Produto já existe na mercearia na Fileira {}, Prateleira {}, Zona {}. Operação cancelada.", id_fil_existe, id_prat_existe, id_zona_existe))
            }
        }
    }
    fn remover_produto(&mut self, id_prod: &str) -> Result<Produto, String>{
        match self.produtos_por_fileira.remove(id_prod) {
            None => {Err(String::from("Produto não existe nesta mercearia. Operação cancelada."))}
            Some(id_fileira) => {Ok(self.lista_fileira[id_fileira].remover_produto(id_prod).unwrap())}
        }
    }
    fn mudar_nome(&mut self, id_prod: &str, novo_nome: &str) -> Result<String, String>{
        match self.tem_produto(id_prod) {
            None => {Err(format!("Produto {} não existe na mercearia, por isso não pode mudar de nome.", id_prod))},
            Some([id_fileira, id_prateleira, id_zona]) => {
                let ref_prod = self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos.get_mut(id_prod).unwrap();
                ref_prod.mudar_nome(novo_nome);
                Ok(String::from("Sucesso"))
            }
        }
    }
    fn mudar_preço(&mut self, id_prod: &str, novo_preço: f64) -> Result<String, String>{
        match self.tem_produto(id_prod) {
            None => {Err(format!("Produto {} não existe na mercearia, por isso não pode mudar de nome.", id_prod))},
            Some([id_fileira, id_prateleira, id_zona]) => {
                let ref_prod = self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos.get_mut(id_prod).unwrap();
                ref_prod.mudar_preço(novo_preço);
                Ok(String::from("Sucesso"))
            }
        }
    }
    fn mover_produto(&mut self, id_prod: &str, nova_fileira: usize, nova_prateleira: usize, nova_zona: usize) -> Result<String, String>{
        match self.remover_produto(id_prod) {
            Err(_e) => {Err(format!("Produto {} não existe na mercearia, por isso não pode ser movido.", id_prod))}
            Ok(prod) => {Ok(self.inserir_produto(prod, nova_fileira, nova_prateleira, nova_zona)?)}
        }
    }
    fn adicionar_quantidade(&mut self, id_prod: &str, adic: u32) -> Result<String, String>{
        match self.tem_produto(id_prod) {
            None => {Err(format!("Produto {} não existe na mercearia, por isso não pode ser adicionada quantidade.", id_prod))},
            Some([id_fileira, id_prateleira, id_zona]) => {
                let ref_prod = self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos.get_mut(id_prod).unwrap();
                ref_prod.quantidade += adic;
                Ok(String::from("Sucesso"))
            }
        }
    }
    fn remover_quantidade(&mut self, id_prod: &str, remov: u32) -> Result<String, String>{
        match self.tem_produto(id_prod) {
            None => { Err(format!("Produto {} não existe na mercearia, por isso não pode ser removida quantidade.", id_prod)) },
            Some([id_fileira, id_prateleira, id_zona]) => {
                let ref_prod = self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos.get_mut(id_prod).unwrap();
                if remov <= ref_prod.quantidade {
                    ref_prod.quantidade -= remov;
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
impl fmt::Display for Zona{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Zona {}: {{", self.id)?;
        for (_key, produto) in &self.lista_produtos{
            write!(f, " {} ", produto)?;
        }write!(f,"}}")
    }
}
impl fmt::Display for Prateleira{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Prateleira {}:", self.id)?;
        for zona in &self.lista_zonas{
            write!(f, "\n  {}", zona)?;
        }Ok(())
    }
}
impl fmt::Display for Fileira{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fileira {}:", self.id)?;
        for prateleira in &self.lista_prateleiras{
            write!(f, "\n {}", prateleira)?;
        }Ok(())
    }
}
impl fmt::Display for Mercearia{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mercearia:")?;
        for fileira in &self.lista_fileira{
            write!(f, "\n{}", fileira)?;
        }Ok(())
    }
}
