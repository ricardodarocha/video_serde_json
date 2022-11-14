// use serde_json::Value;
use serde_derive::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Dados {
    nome: String,
    idade: Option<u32>,
    endereco: Endereco,
    fones: Vec<String>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Endereco {
    rua: String,
    bairro: String,
    numero: u32,
    complemento: String     
}

fn main() {
    let json_str = std::include_str!("../src/dados.json");
    // let res: Value = serde_json::from_str(json_str).unwrap();
    let res: Dados = serde_json::from_str(json_str).unwrap();
    
    // let nome = &res["nome"].as_str().unwrap();
    // println!("Olá {}", nome);
    println!("Olá {}", res.nome);
    
    // let idade = &res["idade"];
    match res.idade {
        Some(x) => println!("{:?} anos",  x),
        None => println!("idade não informada")
    }
    
    // let bairro = &res["endereco"]["bairro"].as_str().unwrap();
    // let rua = &res["endereco"]["rua"].as_str().unwrap();
    // let numero = &res["endereco"]["numero"];
    // println!("{}, {} ", rua, numero);
    // println!("{} ", bairro);
    dbg!("{}", res);
}

