use chrono::prelude::*;
pub mod lib2;
use lib2::{Item, Qualidade, Armazem};

#[derive(Debug, Clone)]
struct Produto{
    identificador: u32,
    nome: String,
    preço: f64,
    quantidade: u32,
    qualidade: Qualidade,
}
#[derive(Debug, Clone)]
struct Livro{
    identificador: u32,
    titulo: String,
    autor: String,
    preço: f64,
    quantidade: u32,
    qualidade: Qualidade,
}
#[derive(Debug, Clone)]
struct Estatua{
    identificador: u32,
    nome: String,
    autor: String,
    quantidade: u32,
    qualidade: Qualidade,
}
fn main() {
    let presunto = Produto{
        identificador: 100,
        nome: String::from("Presunto"),
        preço: 5.0,
        quantidade: 5,
        qualidade: Qualidade::Fragil {
            validade: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            max: 10
        },
    };
    let livro1 = Livro{
        identificador: 200,
        titulo: String::from("Os Maias"),
        autor: String::from("Eça de Queirós"),
        preço: 20.0,
        quantidade: 5,
        qualidade: Qualidade::Normal
    };
    let estatua1 = Estatua {
        identificador: 300,
        nome: String::from("David"),
        autor: String::from("Miguel Ângelo"),
        quantidade: 1,
        qualidade: Qualidade::Oversized{n_zonas: 2},
    };
    let queijo = Produto{
        identificador: 101,
        nome: String::from("Queijo"),
        preço: 4.0,
        quantidade: 10,
        qualidade: Qualidade::Fragil {
            validade: NaiveDate::from_ymd_opt(2025, 1, 2).unwrap(),
            max: 10
        },
    };
    let estatua2 = Estatua {
        identificador: 301,
        nome: String::from("O Pensador"),
        autor: String::from("Auguste Rodin"),
        quantidade: 1,
        qualidade: Qualidade::Oversized{n_zonas: 2},
    };
    let estatua_mt_grande = Estatua {
        identificador: 302,
        nome: String::from("Cristo Redentor"),
        autor: String::from("Paul Landowski"),
        quantidade: 1,
        qualidade: Qualidade::Oversized{n_zonas: 20},
    };
    let livro_mesmo_nome = Livro{ //não permitido (?)
        identificador: 201,
        titulo: String::from("Os Maias"),
        autor: String::from("Eça de Queirós"),
        preço: 20.0,
        quantidade: 5,
        qualidade: Qualidade::Normal
    };
    let livro_mesmo_id = Livro{ //não permitido
        identificador: 200,
        titulo: String::from("Memorial do Convento"),
        autor: String::from("José Saramago"),
        preço: 20.0,
        quantidade: 5,
        qualidade: Qualidade::Normal
    };
    let livro1_outra_vez = Livro{ //permitido
        identificador: 200,
        titulo: String::from("Os Maias"),
        autor: String::from("Eça de Queirós"),
        preço: 20.0,
        quantidade: 10,
        qualidade: Qualidade::Normal
    };
    let presunto_a_mais = Produto{
        identificador: 100,
        nome: String::from("Presunto"),
        preço: 5.0,
        quantidade: 6,
        qualidade: Qualidade::Fragil {
            validade: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            max: 10
        },
    };

    /*let mut round_robin = RoundRobin{atual: [0;4], maximos: [2, 2, 2, 2]};
    let max_iter = round_robin.maximos.iter().fold(1, |acc, m| {acc*m});
    let mut i = 0;
    for i in 0..max_iter {
        println!("{}: {:?}", i, round_robin.atual);
        let _ = round_robin.add1(0, 0);
    }
    println!("{}", max_iter);*/

    println!("----Criar armazém e inserir itens----");
    let mut armazem = Armazem::new(2, 2, 2, 2);
    println!("{:?}", armazem.inserir_item(presunto));
    println!("{:?}", armazem.inserir_item(livro1));
    println!("{:?}", armazem.inserir_item(estatua1));
    println!("{:?}", armazem.inserir_item(queijo));
    println!("{:?}", armazem.inserir_item(estatua2));
    println!("{:?}", armazem.inserir_item(estatua_mt_grande)); //não há espaço para este item
    println!("{:?}", armazem.inserir_item(livro_mesmo_nome)); //não permitido, pois já existe item com mesmo nome, mas id diferente
    println!("{:?}", armazem.inserir_item(livro_mesmo_id)); //não permitido, pois já existe item com mesmo id, mas nome diferente
    println!("{:?}", armazem.inserir_item(livro1_outra_vez)); //permitido
    println!("{:?}", armazem.inserir_item(presunto_a_mais)); //tentativa de colocar mais 6 de presunto, quando já existem 5 e o máximo é 10 (não permitido)
    println!("{}", armazem);
    println!("{:?}", armazem.itens_por_coordenadas());
    println!("{:?}", armazem.nomes_por_identificador());
    println!("{:?}", armazem.validades());
    //println!("{:?}", armazem.procura_livre(1, armazem.round_robin.atual, 0, 0));

    println!("---Procurar se o armazém tem determinado item e se sim, quantos?---");
    println!("O armazém tem item com id 200? {:?}", armazem.tem_item(200));
    println!("O armazém tem item com id 301? {:?}", armazem.tem_item(301));
    println!("O armazém tem item com id 1? {:?}", armazem.tem_item(1));
    println!("O armazém tem item com nome Os Maias? {:?}", armazem.tem_item_nome("Os Maias".to_string()));
    println!("O armazém tem item com nome David? {:?}", armazem.tem_item_nome("David".to_string()));
    println!("O armazém tem item com nome Laranjas? {:?}", armazem.tem_item_nome("Laranjas".to_string()));

    println!("---Procurar se o armazém tem tem determinado item e se sim, onde? [zona, nivel, prateleira, fileira]---");
    println!("Onde está o item com id 200? {:?}", armazem.onde_item(200));
    println!("Onde está o item com id 301? {:?}", armazem.onde_item(301));
    println!("Onde está o item com id 1? {:?}", armazem.onde_item(1));

    println!("---Remover itens do armazém---");
    println!("Remover item da Fileira 0, Prateleira 0, Nivel 0, Zona 1: {:?}", armazem.remover_item(0,0,0,1));
    println!("Remover item da Fileira 0, Prateleira 0, Nivel 1, Zona 0: {:?}", armazem.remover_item(0,0,1,0));
    println!("Remover item da Fileira 0, Prateleira 1, Nivel 1, Zona 0: {:?}", armazem.remover_item(0,1,1,0));
    println!("Remover item da Fileira 1, Prateleira 1, Nivel 1, Zona 1: {:?}", armazem.remover_item(1,1,1,1));
    println!("{}", armazem);
    println!("{:?}", armazem.itens_por_coordenadas());
    println!("{:?}", armazem.nomes_por_identificador());
    println!("O armazém tem item com id 300? {:?}", armazem.tem_item(300));
    println!("O armazém tem item com id 200? {:?}", armazem.tem_item(200));

    println!("---Obter a lista completa dos itens no armazém ordenada alfabeticamente---");
    armazem.itens_ordenados_nome();

    println!("---Verificar quantos produtos estarão fora de validade em 3 dias de uma certa data---");
    let presunto_mesma_data = Produto{
        identificador: 100,
        nome: String::from("Presunto"),
        preço: 5.0,
        quantidade: 5,
        qualidade: Qualidade::Fragil {
            validade: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            max: 10
        },
    };
    println!("(Inserir mais presunto): {:?}", armazem.inserir_item(presunto_mesma_data));

    println!("Quantos produtos estarão fora de validade a 3 dias de 29/12/2024? {:?}", armazem.validade_3dias(2024, 12, 29));
    println!("Quantos produtos estarão fora de validade a 3 dias de 30/12/2024? {:?}", armazem.validade_3dias(2024, 12, 30));
    println!("Quantos produtos estarão fora de validade a 3 dias de 05/01/2025? {:?}", armazem.validade_3dias(2025, 01, 5));
    println!("Validades do armazém: {:?}", armazem.validades());

    println!("---Armazém final:---");
    println!("{}", armazem);

}
impl Item for Produto{
    fn identificador(&self) -> &u32{
        &self.identificador
    }
    fn nome(&self) -> &String{
        &self.nome
    }
    fn quantidade(&self) -> &u32{
        &self.quantidade
    }
    fn qualidade(&self) -> &Qualidade{
        &self.qualidade
    }
}
impl Item for Livro{
    fn identificador(&self) -> &u32{
        &self.identificador
    }
    fn nome(&self) -> &String{
        &self.titulo
    }
    fn quantidade(&self) -> &u32 {
        &self.quantidade
    }
    fn qualidade(&self) -> &Qualidade{
        &self.qualidade
    }
}
impl Item for Estatua{
    fn identificador(&self) -> &u32{
        &self.identificador
    }
    fn nome(&self) -> &String{
        &self.nome
    }
    fn quantidade(&self) -> &u32{
        &self.quantidade
    }
    fn qualidade(&self) -> &Qualidade{
        &self.qualidade
    }
}