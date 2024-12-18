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
    lista_produtos: Vec<Produto>,
}
#[derive(Debug)]
struct Prateleira{
    id: u32,
    lista_zonas: Vec<Zona>,
}
#[derive(Debug)]
struct Fileira{
    id: u32,
    lista_prateleiras: Vec<Prateleira>,
}
#[derive(Debug)]
struct Mercearia{
    lista_fileira: Vec<Fileira>,
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

    println!("---Mudar preço e nome de Produto---");
    queijo.mudar_preço(4.5);
    queijo.mudar_nome("Queijo da Serra");
    println!("{}", queijo);

    /*let mut zona_teste = Zona{id: 0, lista_produtos: vec![]};
    println!("{:?}", zona_teste);
    println!("{}", zona_teste.tem_produto("QUE"));
    zona_teste.inserir_produto(queijo.clone());
    println!("{:?}", zona_teste);
    println!("{}", zona_teste.tem_produto("QUE"));
    zona_teste.inserir_produto(queijo);*/

    println!("---Criar Mercearia e inserir produtos---");
    let mut mercearia = Mercearia::new(2, 3, 3);
    //mercearia.obter_zona(0, 1, 2).lista_produtos = vec![queijo.clone()];
    mercearia.inserir_produto(presunto, 0, 1, 2);
    mercearia.inserir_produto(queijo.clone(), 0, 1, 2);
    println!("{}", mercearia.obter_zona(0, 1, 2));
    //println!("{}", mercearia.obter_prateleira(0, 1));
    //println!("{}", mercearia.obter_fileira(0));
    println!("{}", mercearia);
    println!("---Tentativa de inserir produto repetido---");
    mercearia.inserir_produto(queijo, 1, 0, 1);

    println!("---Verificar se determinado Produto existe numa dada região da Mercearia---");
    println!("QUE existe na fila 0? {}", mercearia.obter_fileira(0).tem_produto("QUE").0);
    println!("QUE existe na prateleira 1 da fila 0? {}", mercearia.obter_prateleira(0, 1).tem_produto("QUE").0);
    println!("QUE existe na zona 2 da prateleira 1 da fila 0? {}", mercearia.obter_zona(0, 1, 2).tem_produto("QUE").0);
    println!("QUE existe na fila 1? {}", mercearia.obter_fileira(1).tem_produto("QUE").0);

    println!("---Remover Produto---");
    mercearia.remover_produto("QUE");
    println!("QUE existe na mercearia? {}", mercearia.tem_produto("QUE").0);
    println!("{}", mercearia.obter_zona(0, 1, 2));

    println!("---Mudar nome de Produto da Mercearia---");
    mercearia.mudar_nome("PRE", "Presunto Ibérico");
    println!("{}", mercearia.obter_zona(0, 1, 2));
    println!("---Mudar preço de Produto da Mercearia---");
    mercearia.mudar_preço("PRE", 5.99);
    println!("{}", mercearia.obter_zona(0, 1, 2));

    println!("---Mover Produto dentro da Mercearia---");
    mercearia.mover_produto("PRE", 1, 0, 1);
    println!("{}", mercearia.obter_zona(0, 1, 2));
    println!("{}", mercearia.obter_zona(1, 0, 1));

    println!("---Adicionar quantidade de Produto da Mercearia---");
    mercearia.adicionar_quantidade("PRE", 5);
    println!("{}", mercearia.obter_zona(1, 0, 1));
    println!("---Remover quantidade de Produto da Mercearia---");
    mercearia.remover_quantidade("PRE", 24);
    println!("{}", mercearia.obter_zona(1, 0, 1));
    println!("---Tentativa de remover maior quantidade de Produto do que aquela que existe---");
    mercearia.remover_quantidade("PRE", 2);
    println!("{}", mercearia.obter_zona(1, 0, 1));
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
    fn tem_produto(&self, id_produto: &str) -> (bool, u32, usize){
        for (index_prod, produto) in self.lista_produtos.iter().enumerate() {
            if produto.identificador == id_produto {
                return (true, self.id, index_prod);
            }
        }(false, 0, 0)
    }
    fn inserir_produto(&mut self, prod: Produto){
        if self.tem_produto(&prod.identificador).0{
            println!("Este produto já existe. Operação cancelada.");
            return;
        }else{
            self.lista_produtos.push(prod);
        }
    }
    fn remover_produto_id(&mut self, id_prod: &str){
        for (i,produto) in self.lista_produtos.iter().enumerate() {
            if produto.identificador == id_prod {
                let prod_removido = self.lista_produtos.remove(i);
                println!("{} removido", prod_removido.nome);
                return;
            }
        }
    }
    fn remover_produto_index(&mut self, index_prod: usize) -> Produto{
        let prod_removido = self.lista_produtos.remove(index_prod);
        println!("{} removido", prod_removido.nome);
        prod_removido
    }
}
impl Prateleira {
    fn new (n_zona: u32, id: u32) -> Prateleira{
        let mut lista_zonas = vec![];
        for i in 0..n_zona{
            lista_zonas.push(Zona{id: i, lista_produtos: vec![]});
        }
        Prateleira{id, lista_zonas}
    }
    fn tem_produto(&self, id_prod: &str) -> (bool, u32, u32, usize){
        for zona in self.lista_zonas.iter() {
            let (existe, id_zona, index_prod) = zona.tem_produto(id_prod);
            if existe {
                return (true, self.id, id_zona, index_prod);
            }
        }(false, 0, 0, 0)
    }
}
impl Fileira{
    fn new (n_prateleira: u32, n_zona: u32, id: u32) -> Fileira{
        let mut lista_prateleiras = vec![];
        for i in 0..n_prateleira{
            lista_prateleiras.push(Prateleira::new(n_zona, i));
        }
        Fileira{id, lista_prateleiras}
    }
    fn tem_produto(&self, id_prod: &str) -> (bool, u32, u32, u32, usize){
        for prateleira in self.lista_prateleiras.iter() {
            let (existe, id_prateleira, id_zona, index_prod) = prateleira.tem_produto(id_prod);
            if existe {
                return (true, self.id, id_prateleira, id_zona, index_prod);
            }
        }(false, 0, 0, 0, 0)
    }
}
impl Mercearia {
    fn new(n_fileira: u32, n_prateleira: u32, n_zona: u32) -> Mercearia{
        let mut lista_fileira = vec![];
        for i in 0..n_fileira{
            lista_fileira.push(Fileira::new(n_prateleira, n_zona, i));
        }
        Mercearia{lista_fileira}
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
    fn inserir_produto(&mut self, prod: Produto, id_fileira: u32, id_prateleira: u32, id_zona: u32){
        let (existe, id_fil_existe, id_prat_existe, id_zona_existe, _index_prod) = self.tem_produto(&prod.identificador);
        if existe {
            println!("Produto {} já existe na Fila {}, Prateleira {}, Zona {}. Operação cancelada", prod.identificador, id_fil_existe, id_prat_existe, id_zona_existe);
            return;
        }else{
            self.obter_zona(id_fileira, id_prateleira, id_zona).inserir_produto(prod);
        }
    }
    fn tem_produto(&self, id_prod: &str) -> (bool, u32, u32, u32, usize){
        for fileira in self.lista_fileira.iter() {
            let (existe, id_fileira, id_prateleira, id_zona, index_prod) = fileira.tem_produto(id_prod);
            if existe{
                return (true, id_fileira, id_prateleira, id_zona, index_prod);
            }
        }(false, 0, 0, 0, 0)
    }
    fn remover_produto(&mut self, id_prod: &str){
        let (existe, id_fileira, id_prateleira, id_zona, index_prod) = self.tem_produto(id_prod);
        if existe{
            self.obter_zona(id_fileira, id_prateleira, id_zona).remover_produto_index(index_prod);
        }else{
            println!("Produto {} não existe na mercearia, por isso não pode ser removido.", id_prod)
        }
    }
    fn mudar_nome(&mut self, id_prod: &str, novo_nome: &str){
        let (existe, id_fileira, id_prateleira, id_zona, index_prod) = self.tem_produto(id_prod);
        if existe{
            self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos[index_prod].mudar_nome(novo_nome);
        }else{
            println!("Produto {} não existe na mercearia, por isso não pode mudar de nome.", id_prod);
        }
    }
    fn mudar_preço(&mut self, id_prod: &str, novo_preço: f64){
        let (existe, id_fileira, id_prateleira, id_zona, index_prod) = self.tem_produto(id_prod);
        if existe{
            self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos[index_prod].mudar_preço(novo_preço);
        }else{
            println!("Produto {} não existe na mercearia, por isso não pode mudar de preço.", id_prod);
        }
    }
    fn mover_produto(&mut self, id_prod: &str, nova_fileira: u32, nova_prateleira: u32, nova_zona: u32){
        let (existe, id_fileira, id_prateleira, id_zona, index_prod) = self.tem_produto(id_prod);
        if existe{
            let prod_removido = self.obter_zona(id_fileira, id_prateleira, id_zona).remover_produto_index(index_prod);
            self.obter_zona(nova_fileira, nova_prateleira, nova_zona).inserir_produto(prod_removido);
        }else{
            println!("Produto {} não existe na mercearia, por isso não pode ser movido.", id_prod);
        }
    }
    fn adicionar_quantidade(&mut self, id_prod: &str, adic: u32){
        let (existe, id_fileira, id_prateleira, id_zona, index_prod) = self.tem_produto(id_prod);
        if existe{
            self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos[index_prod].quantidade+=adic;
        }else{
            println!("Produto {} não existe na mercearia, por isso não pode ser adicionada quantidade.", id_prod);
        }
    }
    fn remover_quantidade(&mut self, id_prod: &str, remov: u32){
        let (existe, id_fileira, id_prateleira, id_zona, index_prod) = self.tem_produto(id_prod);
        if existe{
            let q = &mut self.obter_zona(id_fileira, id_prateleira, id_zona).lista_produtos[index_prod].quantidade;
            if remov <= *q{
                *q-=remov;
            }else{
                println!("Não se pode remover maior quantidade do que aquela que existe. Operação cancelada.");
                return;
            }
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
        for produto in &self.lista_produtos{
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