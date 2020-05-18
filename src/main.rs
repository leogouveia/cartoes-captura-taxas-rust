extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Catalogo {
    #[serde(rename(deserialize = "@odata.context"))]
    odata_context: String,

    #[serde(rename(deserialize = "value"))]
    emissores: Vec<CatalogoEmissor>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CatalogoEmissor {
    #[serde(rename(deserialize = "NomeInstituicao"))]
    nome_instituicao: String,

    #[serde(rename(deserialize = "CnpjInstituicao"))]
    cnpj_instituicao: String,

    #[serde(rename(deserialize = "URLConsulta"))]
    url_consulta: String,

    #[serde(rename(deserialize = "URLDados"))]
    url_dados: String,

    #[serde(rename(deserialize = "Versao"))]
    versao: String,

    #[serde(rename(deserialize = "Recurso"))]
    recurso: String,

    #[serde(rename(deserialize = "Situacao"))]
    situacao: String,

    #[serde(rename(deserialize = "Api"))]
    api: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Emissor {
    #[serde(rename(deserialize = "emissorCnpj"))]
    cnpj: String,

    #[serde(rename(deserialize = "emissorNome"))]
    nome: String,

    #[serde(rename(deserialize = "historicoTaxas"))]
    taxas: Value,
}

// TODO: Poder escolher entre capturar somente a ultima taxa ou histórico
// TODO: Salvar CSV
// TODO: Capturar taxas de forma assincrona?
/**
 *
 */
fn main() {
    println!(".Capturando catalogos");
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Não foi possivel criar reqwest client");
    let catalogo = get_catalogo(&client).expect("Erro ao capturar catalogo");
    println!("..Catalogo capturado com sucesso");
    let emissores_catalogo = catalogo.emissores;
    let mut emissores: Vec<Emissor> = vec![];

    for emissor_catalogo in emissores_catalogo {
        println!(".Capturando emissor: {}", emissor_catalogo.nome_instituicao);
        match get_emissor(&client, &emissor_catalogo) {
            Ok(emissor) => {
                println!("..Emissor capturado com sucesso");
                emissores.push(emissor)
            }
            Err(error) => {
                println!("..Houve um erro ao capturar o emissor {:?}", error);
                ()
            }
        };
    }
    println!("{:?}", emissores.len());
}

/**
 *
 */
fn get_emissor(
    client: &reqwest::blocking::Client,
    emissor: &CatalogoEmissor,
) -> Result<Emissor, Box<dyn Error>> {
    let res = client.get(&emissor.url_dados).send()?;
    let body = res.text()?;
    let emissor_taxas: Emissor = serde_json::from_str(&body)?;
    Ok(emissor_taxas)
}

/**
 *
 */
fn get_catalogo(client: &reqwest::blocking::Client) -> Result<Catalogo, Box<dyn Error>> {
    let request_url = "https://olinda.bcb.gov.br/olinda/servico/DASFN/versao/v1/odata/Recursos?$top=10000&$filter=Api%20eq%20'taxas_cartoes'%20and%20Recurso%20eq%20'%2Fitens'&$format=json";
    // println!("{}", request_url);
    let res = client.get(request_url).send()?;
    let body = res
        .text()
        .expect("Houve um erro ao capturar o texto do corpo");
    let catalogo: Catalogo = serde_json::from_str(&body).expect("Errou ao abrir json");
    Ok(catalogo)
}
