use base64::{Engine as _, engine::general_purpose};
use ethers::{
    core::types::{Filter, H160},
    providers::{Http, Middleware, Provider, StreamExt}, prelude::{abigen, types::U256},
};
use eyre::Result;
use std::{sync::Arc, fs::File, str::from_utf8, io::Write};
use dotenv::dotenv;
use serde_json::Value;

abigen!(
    IERC721,
    r#"[
        function tokenURI(uint256 id) external view returns (string tokenURI)
    ]"#,
);

const NOUN_ADDRESS: &str = "0x93ecac71499147627DFEc6d0E494d50fCFFf10EE";

#[tokio::main]
async fn main() -> Result<()> {
    get_past_nouns().await;
    listen_for_new_nouns().await;

    Ok(())
}

async fn listen_for_new_nouns() {
    dotenv().ok();
    let provider = get_client().await;
    let client = Arc::new(provider);
    let noun_contract = IERC721::new(NOUN_ADDRESS.parse::<H160>().expect("wrong address"), client.clone());
    
    let filter = Filter::new()
        .address(noun_contract.address())
        .event("NounCreated(uint256,(uint48,uint48,uint48,uint48,uint48))");

    let mut stream = client.watch(&filter).await.unwrap();

    while let Some(log) = stream.next().await {
        println!("new noun created!");
        let index = U256::from(log.topics[1].as_ref());
        let noun_uri = noun_contract.token_uri(index).await;
        match noun_uri {
            Ok(res) => {
                process_uri(&res, index);
            }
            Err(e) => {
                println!("error: {:?}", e);
                continue;
            }
        }
    }
    

}

async fn get_past_nouns() {
    dotenv().ok();
    let provider = get_client().await;
    let client = Arc::new(provider);
    let noun_contract = IERC721::new(NOUN_ADDRESS.parse::<H160>().expect("wrong address"), client.clone());
    
    let filter = Filter::new()
        .address(noun_contract.address())
        .event("NounCreated(uint256,(uint48,uint48,uint48,uint48,uint48))")
        .from_block(15614289);

    let logs = client.get_logs(&filter).await.unwrap();
    println!("{} pools found!", logs.iter().len());

    for log in logs.iter() {
        let index = U256::from(log.topics[1].as_ref());
        let noun_uri = noun_contract.token_uri(index).await;
        match noun_uri {
            Ok(res) => {
                process_uri(&res, index);
            }
            Err(e) => {
                println!("error: {:?}", e);
                continue;
            }
        }
    }
}

fn process_uri(base64_string: &str, index: U256) {
    let decoded = general_purpose::STANDARD.decode(&base64_string[29..]).unwrap();
    let json: Value = serde_json::from_str(from_utf8(&decoded).unwrap()).unwrap();
    let image = general_purpose::STANDARD.decode(&json["image"].to_string()[27..&json["image"].to_string().len()-1]).unwrap();
    let res = File::create(format!("nouns/public-noun-{}.svg", index)); 
    match res {
        Ok(mut file) => {
            // println!("file created");
            file.write_all(image.as_ref()).unwrap();
        }
        Err(e) => {
            println!("error: {:?}", e);
            return;
        }
    }
}

async fn get_client() -> Provider<Http>  {
    let provider = Provider::<Http>::try_from(std::env::var("PROVIDER").expect("PROVIDER not provided")).expect("provider incorrect");
    return provider;
}