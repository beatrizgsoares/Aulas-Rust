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
    id: u32,
    lista_produtos: HashMap<String, Produto>, //identificador de Produto -> Produto
}
#[derive(Debug)]
struct Prateleira{
    id: u32,
    lista_zonas: Vec<Zona>,
    produtos_por_zona: HashMap<String, u32>, //identificador de Produto -> id Zona
}
#[derive(Debug)]
struct Fileira{
    id: u32,
    lista_prateleiras: Vec<Prateleira>,
    produtos_por_prateleira: HashMap<String, u32>, //id. Produto -> id Prateleira
}
#[derive(Debug)]
struct Mercearia{
    lista_fileira: Vec<Fileira>,
    produtos_por_fileira: HashMap<String, u32>, //id. Produto -> id Fileira
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
    mercearia.inserir_produto(queijo, 1, 2, 0);
    mercearia.inserir_produto(presunto, 0, 1, 2);
    println!("{}", mercearia);

    println!("----Procurar se produto existe em certa região, a partir do identificador----");
    println!("Mercearia tem Queijo da Serra (QUE)? {:?}", mercearia.tem_produto("QUE").0);
    println!("Fileira 0 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_fileira(0).tem_produto("QUE").0);
    println!("Fileira 1 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_fileira(1).tem_produto("QUE").0);
    println!("Prateleira 1 da Fileira 1 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_prateleira(1, 1).tem_produto("QUE").0);
    println!("Prateleira 2 da Fileira 1 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_prateleira(1, 2).tem_produto("QUE").0);
    println!("Zona 1 da Prateleira 2 da Fileira 1 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_zona(1, 2, 1).tem_produto("QUE"));
    println!("Zona 0 da Prateleira 2 da Fileira 1 tem Queijo da Serra (QUE)? {:?}", mercearia.obter_zona(1, 2, 0).tem_produto("QUE"));

    println!("----Procurar onde produto existe na mercearia----");
    let (existe, id_fileira, id_prateleira, id_zona) = mercearia.tem_produto("QUE");
    println!("Queijo da Serra (QUE) existe? {:?}. Se sim existe na Fileira {}, Prateleira {}, Zona {}", existe, id_fileira, id_prateleira, id_zona);
    let info_pre = mercearia.tem_produto("PRE");
    println!("Presunto (PRE) existe? {:?}. Se sim existe na Fileira {}, Prateleira {}, Zona {}", info_pre.0, info_pre.1, info_pre.2, info_pre.3);

    println!("----Remover produto----");
    mercearia.remover_produto("QUE");
    println!("Mercearia tem Queijo da Serra (QUE)? {:?}", mercearia.tem_produto("QUE").0);

    println!("----Mudar nome de produto----");
    mercearia.mudar_nome("PRE", "Presunto Ibérico");
    println!("{}", mercearia.obter_zona(0, 1, 2));

    println!("----Mudar preço de produto----");
    mercearia.mudar_preço("PRE", 5.99);
    println!("{}", mercearia.obter_zona(0, 1, 2));

    println!("----Mover produto----");
    mercearia.mover_produto("PRE", 1, 1, 1);
    let info_pre = mercearia.tem_produto("PRE");
    println!("Presunto (PRE) existe? {:?}. Se sim existe na Fileira {}, Prateleira {}, Zona {}", info_pre.0, info_pre.1, info_pre.2, info_pre.3);

    println!("----Adicionar quantidade----");
    mercearia.adicionar_quantidade("PRE", 5);
    println!("{}", mercearia.obter_zona(1, 1, 1));

    println!("----Remover quantidade----");
    mercearia.remover_quantidade("PRE", 20);
    println!("{}", mercearia.obter_zona(1, 1, 1));
    mercearia.remover_quantidade("PRE", 20);
    println!("{}", mercearia.obter_zona(1, 1, 1));
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
    fn new (n_zona: u32, id: u32) -> Prateleira{
        let mut lista_zonas = vec![];
        for i in 0..n_zona{
            lista_zonas.push(Zona{id: i, lista_produtos: HashMap::new()});
        }
        Prateleira{id, lista_zonas, produtos_por_zona: HashMap::new()}
    }
    fn tem_produto(&self, id_prod: &str) -> (bool, u32){
        match self.produtos_por_zona.get_key_value(id_prod) {
            None => {(false,0)},
            Some((_id_prod,&id_zona)) => {(true, id_zona)}
        }
    }
    fn remover_produto(&mut self, id_prod: &str) -> Option<Produto>{
        match self.produtos_por_zona.remove(id_prod) {
            None => {println!("Produto não existe nesta prateleira. Operação cancelada."); None}
            Some(id_zona) => {self.lista_zonas[id_zona as usize].remover_produto(id_prod)}
        }
    }
}
impl Fileira{
    fn new (n_prateleira: u32, n_zona: u32, id: u32) -> Fileira{
        let mut lista_prateleiras = vec![];
        for i in 0..n_prateleira{
            lista_prateleiras.push(Prateleira::new(n_zona, i));
        }
        Fileira{id, lista_prateleiras, produtos_por_prateleira: HashMap::new()}
    }
    fn tem_produto(&self, id_prod: &str) -> (bool, u32, u32){
        match self.produtos_por_prateleira.get_key_value(id_prod) {
            None => {(false,0,0)},
            Some((_id_prod,&id_prateleira)) => {
                let id_zona = self.lista_prateleiras[id_prateleira as usize].tem_produto(id_prod).1;
                (true, id_prateleira, id_zona)}
        }
    }
    fn remover_produto(&mut self, id_prod: &str) -> Option<Produto>{
        match self.produtos_por_prateleira.remove(id_prod) {
            None => {println!("Produto não existe nesta fileira. Operação cancelada."); None}
            Some(id_prateleira) => {self.lista_prateleiras[id_prateleira as usize].remover_produto(id_prod)}
        }
    }
}
impl Mercearia {
    fn new(n_fileira: u32, n_prateleira: u32, n_zona: u32) -> Mercearia{
        let mut lista_fileira = vec![];
        for i in 0..n_fileira{
            lista_fileira.push(Fileira::new(n_prateleira, n_zona, i));
        }
        Mercearia{lista_fileira, produtos_por_fileira: HashMap::new()}
    }
    fn obter_zona(&mut self, id_fileira: u32, id_prateleira: u32, id_zona: u32) -> &mut Zona{
        &mut self.lista_fileira[id_fileira as usize]
            .lista_prateleiras[id_prateleira as usize]
            .lista_zonas[id_zona as usize]
    }
    fn obter_prateleira(&mut self, id_fileira: u32, id_prateleira: u32) -> &mut Prateleira{
        &mut self.lista_fileira[id_fileira as usize]
            .lista_prateleiras[id_prateleira as usize]
    }
    fn obter_fileira(&mut self, id_fileira: u32) -> &mut Fileira{
        &mut self.lista_fileira[id_fileira as usize]
    }
    fn tem_produto(&self, id_prod: &str) -> (bool, u32, u32, u32){
        match self.produtos_por_fileira.get_key_value(id_prod) {
            None => {(false,0,0,0)},
            Some((_id_prod,&id_fileira)) => {
                let (_lixo, id_prateleira, id_zona) = self.lista_fileira[id_fileira as usize].tem_produto(id_prod);
                (true, id_fileira, id_prateleira, id_zona)}
        }
    }
    fn inserir_produto(&mut self, prod: Produto, id_fileira: u32, id_prateleira: u32, id_zona: u32){
        let (existe, id_fil_existe, id_prat_existe, id_zona_existe) = self.tem_produto(&prod.identificador);
        if existe {
            println!("Produto {} já existe na Fila {}, Prateleira {}, Zona {}. Operação cancelada", prod.identificador, id_fil_existe, id_prat_existe, id_zona_existe);
            return;
        }else{
            self.produtos_por_fileira.insert(prod.identificador.clone(), id_fileira);
            self.obter_fileira(id_fileira).produtos_por_prateleira.insert(prod.identificador.clone(), id_prateleira);
            self.obter_prateleira(id_fileira, id_prateleira).produtos_por_zona.insert(prod.identificador.clone(), id_zona);
            self.obter_zona(id_fileira, id_prateleira, id_zona).inserir_produto(prod);
        }
    }
    fn remover_produto(&mut self, id_prod: &str) -> Option<Produto>{
        match self.produtos_por_fileira.remove(id_prod) {
            None => {println!("Produto não existe nesta mercearia. Operação cancelada."); None}
            Some(id_fileira) => {self.lista_fileira[id_fileira as usize].remover_produto(id_prod)}
        }
    }
    fn mudar_nome(&mut self, id_prod: &str, novo_nome: &str){
        let (existe, id_fileira, id_prateleira, id_zona) = self.tem_produto(id_prod);
        if existe{
            match self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos.get_mut(id_prod) {
                None => {}
                Some(ref_prod) => {ref_prod.mudar_nome(novo_nome);}
            };
        }else{
            println!("Produto {} não existe na mercearia, por isso não pode mudar de nome.", id_prod);
        }
    }
    fn mudar_preço(&mut self, id_prod: &str, novo_preço: f64){
        let (existe, id_fileira, id_prateleira, id_zona) = self.tem_produto(id_prod);
        if existe{
            match self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos.get_mut(id_prod) {
                None => {}
                Some(ref_prod) => {ref_prod.mudar_preço(novo_preço);}
            };
        }else{
            println!("Produto {} não existe na mercearia, por isso não pode mudar de preço.", id_prod);
        }
    }
    fn mover_produto(&mut self, id_prod: &str, nova_fileira: u32, nova_prateleira: u32, nova_zona: u32){
        match self.remover_produto(id_prod) {
            None => {println!("Produto {} não existe na mercearia, por isso não pode ser movido.", id_prod)}
            Some(prod) => {self.inserir_produto(prod, nova_fileira, nova_prateleira, nova_zona);}
        }
    }
    fn adicionar_quantidade(&mut self, id_prod: &str, adic: u32){
        let (existe, id_fileira, id_prateleira, id_zona) = self.tem_produto(id_prod);
        if existe{
            match self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos.get_mut(id_prod) {
                None => {}
                Some(ref_prod) => {ref_prod.quantidade += adic;}
            };
        }else{
            println!("Produto {} não existe na mercearia, por isso não pode ser adicionada quantidade.", id_prod);
        }
    }
    fn remover_quantidade(&mut self, id_prod: &str, remov: u32){
        let (existe, id_fileira, id_prateleira, id_zona) = self.tem_produto(id_prod);
        if existe{
            match self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos.get_mut(id_prod) {
                None => {}
                Some(ref_prod) => {
                    if remov<=ref_prod.quantidade {
                        ref_prod.quantidade -= remov;
                    }else{
                        println!("Não se pode remover maior quantidade do que aquela que existe. Operação cancelada.");
                        return;
                    }}
            };
        }else{
            println!("Produto {} não existe na mercearia, por isso não pode ser removida quantidade.", id_prod);
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