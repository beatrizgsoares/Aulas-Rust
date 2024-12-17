use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use chrono::prelude::*;
#[derive(Debug, Clone)]
pub enum Qualidade{
    Fragil{
        validade: NaiveDate,
        max: u32,
    },
    Oversized{
        n_zonas: usize,
    },
    Normal
}
pub trait Item{
    fn identificador(&self) -> &u32;
    fn nome(&self) -> &String;
    fn quantidade(&self) -> &u32;
    fn qualidade(&self) -> &Qualidade;
}
#[derive(Debug)]
pub struct Zona{
    id: usize,
    item: Option<Box<dyn Item>>,
    timestamp: DateTime<Local>,
}
#[derive(Debug)]
pub struct Nivel{
    id: usize,
    lista_zonas: Vec<Zona>,
}
#[derive(Debug)]
pub struct Prateleira{
    id: usize,
    lista_niveis: Vec<Nivel>,
}
#[derive(Debug)]
pub struct Fileira{
    id: usize,
    lista_prateleiras: Vec<Prateleira>,
}
#[derive(Debug)]
pub struct Armazem{
    lista_fileiras: Vec<Fileira>,
    itens_por_coordenadas: HashMap<u32, HashSet<[usize; 4]>>, //id Item -> lista de coordenadas onde existe [idFil, idPrat, idNiv, idZona]
    nomes_por_identificador: HashMap<String, u32>, //nome do item -> id Item
    round_robin: RoundRobin,
    validades: Vec<NaiveDate>,
}
#[derive(Debug)]
pub struct RoundRobin{
    pub atual: [usize; 4], //[n_zona, n_nivel, n_prateleira, n_fileira]
    pub maximos: [usize; 4],
}
impl Zona{
    pub fn new (id: usize) -> Zona{
        Zona{id, item: None, timestamp: Local.with_ymd_and_hms(0,1,1,0,0,0).unwrap()}
    }
    pub fn tem_item(&self, identificador: u32) -> bool{
        match &self.item{
            None => false,
            Some(item) => *item.identificador()==identificador
        }
    }
    fn inserir_item(&mut self, item_box: Box<dyn Item>) -> Result<String, String> {
        match self.item{
            None => {self.item = Some(item_box); self.timestamp = Local::now(); Ok("Sucesso".to_string())},
            Some(_) => Err("Já existe um item nesta zona!".to_string()),
        }
    }
    fn remover_item(&mut self) -> Result<Box<dyn Item>, String> {
        match self.item.take(){
            Some(item_box)  => {
                self.timestamp = Local.with_ymd_and_hms(0,1,1,0,0,0).unwrap();
                Ok(item_box)
            }
            None => Err("Não existe item nesta zona!".to_string())
        }
    }
}
impl Nivel{
    pub fn new (id: usize, n_zonas: usize) -> Nivel{
        let mut lista_zonas = vec![];
        for i in 0..n_zonas{
            lista_zonas.push(Zona::new(i));
        }
        Nivel{id, lista_zonas}
    }
}
impl Prateleira{
    pub fn new(id: usize, n_niveis: usize, n_zonas: usize) -> Prateleira{
        let mut lista_niveis = vec![];
        for i in 0..n_niveis{
            lista_niveis.push(Nivel::new(i, n_zonas));
        }
        Prateleira{id, lista_niveis}
    }
}
impl Fileira{
    pub fn new(id: usize, n_prateleiras: usize, n_niveis: usize, n_zonas: usize) -> Fileira{
        let mut lista_prateleiras = vec![];
        for i in 0..n_prateleiras{
            lista_prateleiras.push(Prateleira::new(i, n_niveis, n_zonas));
        }
        Fileira{id, lista_prateleiras}
    }
}
impl Armazem{
    pub fn new(n_fileiras: usize, n_prateleiras: usize, n_niveis: usize, n_zonas: usize) -> Armazem{
        let mut lista_fileiras = vec![];
        for i in 0..n_fileiras{
            lista_fileiras.push(Fileira::new(i, n_prateleiras, n_niveis, n_zonas));
        }
        Armazem{lista_fileiras,
            itens_por_coordenadas: HashMap::new(),
            nomes_por_identificador: HashMap::new(),
            round_robin: RoundRobin{atual: [0; 4], maximos: [n_zonas, n_niveis, n_prateleiras, n_fileiras]},
            validades: vec![],
        }
    }
    pub fn validades(&self) -> &Vec<NaiveDate> {
        &self.validades
    }
    pub fn itens_por_coordenadas(&self) -> &HashMap<u32, HashSet<[usize; 4]>> {
        &self.itens_por_coordenadas
    }
    pub fn nomes_por_identificador(&self) -> &HashMap<String, u32> {
        &self.nomes_por_identificador
    }
    pub fn tem_item(&self, identificador: u32) -> Option<usize> {
        match self.itens_por_coordenadas.get(&identificador) {
            None => {None}
            Some(set) => {Some(set.len())}
        }
    }
    pub fn tem_item_nome(&self, nome: String) -> Option<usize> {
        let item_identificador = self.nomes_por_identificador.get(&nome)?;
        self.tem_item(*item_identificador)
    }
    pub fn onde_item(&self, identificador: u32) -> Option<HashSet<[usize; 4]>> {
        self.itens_por_coordenadas.get(&identificador).cloned()
    }
    fn obter_zona(&mut self, id_fil: usize, id_pra: usize, id_niv: usize, id_zon: usize) -> Result<&mut Zona, String> {
        let max = self.round_robin.maximos;
        if (id_fil<max[3]) & (id_pra<max[2]) & (id_niv<max[1]) & (id_zon<max[0]){
            Ok(&mut self.lista_fileiras[id_fil].lista_prateleiras[id_pra].lista_niveis[id_niv].lista_zonas[id_zon])
        }
        else{
            Err("Índices fora dos limites".to_string())
        }
    }
    fn procura_livre(&mut self, n_zonas: usize, pos_inicial: [usize; 4], zonas_vagas: usize, iter: usize) -> Result<RoundRobin,String>{
        let max_iter = self.round_robin.maximos.iter().fold(1, |acc, m| {acc*m});
        let pos = self.round_robin.atual;
        let zona_atual = self.obter_zona(pos[3], pos[2], pos[1], pos[0]).unwrap();
        if zona_atual.item.is_some(){ //zona atual está ocupada -> aumenta a posição atual e procura mais à frente
            self.round_robin.add1(0, 0).unwrap();
            self.procura_livre(n_zonas, self.round_robin.atual, 0, iter+1)
        }
        else if n_zonas-1 == zonas_vagas{ //nº de zonas vagas consecutivas = nº de zonas necessárias
            return Ok(RoundRobin{atual: pos_inicial, maximos: self.round_robin.maximos});
        }
        else if iter<max_iter { //há um algumas zonas vagas consecutivas, mas ainda não são suficientes
            self.round_robin.add1(0, 0).unwrap();
            self.procura_livre(n_zonas, pos_inicial, zonas_vagas+1, iter+1)
        }
        else{ //já deu a volta ao armazém -> parar função
            return Err("Não há lugar no armazém para um item deste tamaho".to_string());
        }
    }
    //Função auxiliar chamada por inserir_item.
    //Adiciona a informação necessária aos HashMaps nomes_por_identificador e itens_por_coordenada
    fn auxiliar (&mut self, item_nome: String, item_identificador: u32, pos_atual: [usize; 4]) -> Result<(),String>{
        match self.tem_item(item_identificador){
            None => { //ainda não existe um item com este identificador
                match self.nomes_por_identificador.get(&item_nome) {
                    None => {
                        self.itens_por_coordenadas.insert(item_identificador, HashSet::from([pos_atual]));
                        self.nomes_por_identificador.insert(item_nome, item_identificador);
                        Ok(())
                    }
                    Some(_) => {
                        Err("Já existe um item com este nome, mas com outro identificador".to_string())
                    }
                }
            }
            Some(_) => {
                if  self.nomes_por_identificador.get(&item_nome) != Some(&item_identificador){ //não pode haver itens com o mesmo id, mas nomes diferentes
                    Err("Já existe um item com este identificador, mas com outro nome".to_string())
                } else {
                    self.itens_por_coordenadas.get_mut(&item_identificador).unwrap().insert(pos_atual);
                    Ok(())
                }
            }
        }
    }
    pub fn inserir_item <I: Item + 'static + Clone> (&mut self, item: I) -> Result<String, String> {
        let item_nome = item.nome().clone();
        let item_identificador = *item.identificador();
        let item_box = Box::new(item);
        match item_box.qualidade(){
            Qualidade::Oversized{n_zonas} => {
                let mut pos = self.procura_livre(*n_zonas, self.round_robin.atual, 0, 0)?;
                self.auxiliar(item_nome, item_identificador, pos.atual)?;
                for _i in 0..*n_zonas{
                    let zona = self.obter_zona(pos.atual[3], pos.atual[2], pos.atual[1], pos.atual[0]).unwrap();
                    zona.inserir_item(item_box.clone())?;
                    pos.add1(0, 0)?;
                }
            }
            Qualidade::Fragil{max, validade} => {
                if self.quantidade_item(item_identificador)+item_box.quantidade() > *max {
                    return Err(format!("Não pode haver no armazém uma quantidade maior do que {} para este item Fragil", max));
                } else {
                    let pos = self.procura_livre(1, self.round_robin.atual, 0, 0)?;
                    self.auxiliar(item_nome, item_identificador, pos.atual)?;
                    self.validades.push(*validade);
                    let zona = self.obter_zona(pos.atual[3], pos.atual[2], pos.atual[1], pos.atual[0])?;
                    zona.inserir_item(item_box)?;
                }
            }
            Qualidade::Normal => {
                let pos = self.procura_livre(1, self.round_robin.atual, 0, 0)?;
                self.auxiliar(item_nome, item_identificador, pos.atual)?;
                let zona = self.obter_zona(pos.atual[3], pos.atual[2], pos.atual[1], pos.atual[0])?;
                zona.inserir_item(item_box)?;
            }
        }
        Ok("Sucesso".to_string())
    }
    pub fn quantidade_item(&self, item_identificador: u32) -> u32 {
        match self.itens_por_coordenadas.get(&item_identificador){
            None => 0,
            Some (set) => {
                let mut qt_total = 0;
                for pos in set.iter(){
                    qt_total+=self.obter_item(pos[3], pos[2], pos[1], pos[0]).unwrap().quantidade();
                }
                qt_total
            }
        }
    }
    pub fn obter_item(&self, id_fil: usize, id_pra: usize, id_niv: usize, id_zon: usize) -> Option<&Box<dyn Item>>{
        let max = self.round_robin.maximos;
        if (id_fil<max[3]) & (id_pra<max[2]) & (id_niv<max[1]) & (id_zon<max[0]) {
            self.lista_fileiras[id_fil].lista_prateleiras[id_pra].lista_niveis[id_niv].lista_zonas[id_zon].item.as_ref()
        } else {
            None
        }
    }
    fn remover_item_oversized(&mut self, mut rr: RoundRobin, n_zonas: usize, iter: usize) -> Result<Box<dyn Item>, String>{
       let zona = self.obter_zona(rr.atual[3], rr.atual[2], rr.atual[1], rr.atual[0])?;
       if iter==n_zonas-1{
           zona.remover_item()
       } else {
           zona.remover_item()?;
           rr.add1(0,0)?;
           self.remover_item_oversized(rr, n_zonas, iter+1)
       }
    }
    fn auxiliar_remov(&mut self, item_id: u32, item_nome: String, pos: [usize; 4]){
        if self.tem_item(item_id).unwrap()>1{
            self.itens_por_coordenadas.get_mut(&item_id).unwrap().remove(&pos);
        } else {
            self.itens_por_coordenadas.remove(&item_id);
            self.nomes_por_identificador.remove(&item_nome);
        }
    }
    fn remov_validade(&mut self, validade: NaiveDate){
        for (i,data) in self.validades.iter().enumerate(){
            if validade == *data {
                self.validades.remove(i);
                return;
            }
        }
    }
    pub fn remover_item(&mut self, id_fileira: usize, id_prateleira: usize, id_nivel: usize, id_zona: usize) -> Result<Box<dyn Item>, String> {
        match self.obter_item(id_fileira, id_prateleira, id_nivel, id_zona){
            None => {Err("Não existe um item nesta coordenada. Operação cancelada".to_string())}
            Some(item_box) => {
                let item_id = item_box.identificador().clone();
                let item_nome = item_box.nome().clone();
                match item_box.qualidade(){
                    Qualidade::Fragil{validade, ..} => {
                        self.remov_validade(*validade);
                        self.auxiliar_remov(item_id, item_nome, [id_zona, id_nivel, id_prateleira, id_fileira]);
                        let zona = self.obter_zona(id_fileira, id_prateleira, id_nivel, id_zona)?;
                        zona.remover_item()
                    }
                    Qualidade::Normal => {
                        self.auxiliar_remov(item_id, item_nome, [id_zona, id_nivel, id_prateleira, id_fileira]);
                        let zona = self.obter_zona(id_fileira, id_prateleira, id_nivel, id_zona)?;
                        zona.remover_item()
                    }
                    Qualidade::Oversized {n_zonas} => {
                        let n_zonas = n_zonas.clone();
                        let rr = RoundRobin{atual: [id_zona, id_nivel, id_prateleira, id_fileira], maximos: self.round_robin.maximos};
                        if self.itens_por_coordenadas.get(&item_id).unwrap().contains(&rr.atual) {
                            self.auxiliar_remov(item_id, item_nome, rr.atual);
                            self.remover_item_oversized(rr, n_zonas, 0)
                        }
                        else {
                            Err("Coordenadas dadas estão no meio de um item Oversized. Operação cancelada".to_string())
                        }
                    }
                }
            }
        }
    }
    pub fn itens_ordenados_nome(&mut self){
        let mut nomes_vec: Vec<_> = self.nomes_por_identificador.iter().collect();
        nomes_vec.sort_by_key(|par_valores| par_valores.0.to_lowercase()); // ordenar pelo nome
        //println!("{:?}", nomes_vec);
        println!("Lista ordenada por nome dos itens no armazém:");
        for par_valores in nomes_vec{
            for pos in self.onde_item(*par_valores.1).unwrap(){
                println!("{:?}", self.obter_item(pos[3], pos[2], pos[1], pos[0]).unwrap());
            }
        }
    }
    pub fn validade_3dias(&mut self, ano:i32, mes: u32, dia: u32) -> Result<u32, String> {
        match NaiveDate::from_ymd_opt(ano, mes, dia){
            None => {Err("Data inserida inválida.".to_string())}
            Some(data) => {
                let mut cont = 0;
                for validade in self.validades.iter(){
                    //println!("{}", (*validade-data).num_days())
                    if (*validade-data).num_days() <= 3 {
                        cont+=1;
                    }
                }
                Ok(cont)
            }
        }
    }
}
impl RoundRobin{
    pub fn add1(&mut self, i: usize, iter: u32) -> Result<(), String> {
        let quo = (self.atual[i]+1)/self.maximos[i];
        //println!("{}", quo);
        self.atual[i] = (self.atual[i]+1)%self.maximos[i];
        if iter>30{
            Err("Demasiadas iterações. Operação cancelada".to_string())
        }
        else if quo > 0{
            self.add1((i+1)%4, iter+1)
        }
        else {
            Ok(())
        }
    }
}
impl fmt::Debug for dyn Item{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[id: {}, nome: {}, quantidade: {}, qualidade:{:?}]", self.identificador(), self.nome(), self.quantidade(), self.qualidade())
    }
}
impl fmt::Display for Zona{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Zona {}: {{ {:?}, timestamp: {} }}", self.id, self.item, self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}
impl fmt::Display for Nivel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nivel {}:", self.id)?;
        for zona in &self.lista_zonas{
            write!(f, "\n   {}", zona)?;
        }Ok(())
    }
}
impl fmt::Display for Prateleira {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Prateleira {}:", self.id)?;
        for nivel in &self.lista_niveis{
            write!(f, "\n  {}", nivel)?;
        }Ok(())
    }
}
impl fmt::Display for Fileira {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fileira {}:", self.id)?;
        for prateleira in &self.lista_prateleiras{
            write!(f, "\n {}", prateleira)?;
        }Ok(())
    }
}
impl fmt::Display for Armazem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Armazém:")?;
        for fileira in &self.lista_fileiras{
            write!(f, "\n{}", fileira)?;
        }Ok(())
    }
}