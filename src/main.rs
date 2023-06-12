use ethers::{
    core::types::{Address, Filter, H160},
    providers::{Http, Middleware, Provider}, prelude::abigen, types::U256,
};
use eyre::Result;
use std::{sync::Arc, fs, path::Path};
use dotenv::dotenv;

abigen!(
    IERC721,
    r#"[
        function tokenURI(uint256 id) external view returns (string tokenURI)
        function contractURI() external view returns (string contractURI)
    ]"#,
);


/// This example demonstrates filtering and parsing event logs by fetching all Uniswap V3 pools
/// where both tokens are in the set [USDC, USDT, DAI].
///
/// V3 factory reference: https://github.com/Uniswap/v3-core/blob/main/contracts/interfaces/IUniswapV3Factory.sol
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let provider = Provider::<Http>::try_from(std::env::var("PROVIDER")?)?;
    let client = Arc::new(provider);
    let noun_contract = IERC721::new("0x93ecac71499147627DFEc6d0E494d50fCFFf10EE".parse::<H160>()?, client);
    let noun_uri = noun_contract.contract_uri().await?;
    println!("{}", noun_uri);

    let current_nouns = check_dir();
    get_past_nouns(current_nouns).await;



    Ok(())
}

fn check_dir() -> i32 {
    loop {
        let mut i = 0;
        let exists = Path::new(&format!("public-noun-{}", i)).exists();
        if exists == false {
            return i
        }
        i+=1;
    }
}

async fn get_past_nouns(mut i: i32) -> U256 {
    loop {
        dotenv().ok();
        let provider = Provider::<Http>::try_from(std::env::var("PROVIDER").expect("PROVIDER NOT PROVIDED XD")).expect("PROVIDER IS WRONG");
        let client = Arc::new(provider);
        let noun_contract = IERC721::new("0x93ecac71499147627DFEc6d0E494d50fCFFf10EE".parse::<H160>().expect("wrong address"), client);
        
        let noun_uri = noun_contract.token_uri(i.into()).await; {
            // match noun_uri {
            //     Ok(x) => {
            //         x
            //     }
            //     Err(e) => {
            //         println!("{:?}", e);
            //     }
            // }
        }
        
        i+=1;
    }
}

fn uri_to_svg() {

}