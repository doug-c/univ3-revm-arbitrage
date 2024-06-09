/// This example demonstrates how to use the `eth_call` method to query the Uniswap V3 quoter contract for a quote.

use alloy_provider::{Provider, ProviderBuilder};
use std::sync::Arc;
pub mod source;
use anyhow::Result;
use revm::primitives::U256;
use std::ops::Div;
use dotenv::dotenv;

use crate::source::{
    build_tx, decode_quote_response, me, measure_end, measure_start, official_quoter_addr,
    one_ether, quote_calldata, usdc_addr, weth_addr,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let provider = ProviderBuilder::new().on_http(std::env::var("ETH_RPC_URL").unwrap().parse()?);
    let provider = Arc::new(provider);

    let base_fee = provider.get_gas_price().await?;

    let volume = one_ether().div(U256::from(10)); // 0.1 ETH
    let calldata = quote_calldata(weth_addr(), usdc_addr(), volume, 3000); // ABI-encoded calldata

    let tx = build_tx(official_quoter_addr(), me(), calldata, base_fee);
    let start = measure_start("eth_call_one");
    let call = provider.call(&tx).await?;

    let amount_out = decode_quote_response(call)?;
    println!("{} WETH -> USDC {}", volume, amount_out);

    measure_end(start);

    Ok(())
}
