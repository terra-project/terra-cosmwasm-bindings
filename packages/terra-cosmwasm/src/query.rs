use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Coin, Decimal, QueryRequest, Uint128};
use crate::route::TerraRoute;

/// TerraQueryWrapper is an override of QueryRequest::Custom to access Terra-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TerraQueryWrapper {
    pub route: TerraRoute,
    pub query_data: TerraQuery,
}

/// TerraQuery is defines avaliable query datas
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TerraQuery {
    Swap {
        offer_coin: Coin,
        ask_denom: String,
    },
    TaxRate {},
    TaxCap {
        denom: String,
    },
    ExchangeRates {
        base_denom: String,
        quote_denoms: Vec<String>,
    },
}

// This is a simpler way to making queries
impl Into<QueryRequest<TerraQueryWrapper>> for TerraQueryWrapper {
    fn into(self) -> QueryRequest<TerraQueryWrapper> {
        QueryRequest::Custom(self)
    }
}

/// SwapResponse is data format returned from SwapRequest::Simulate query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapResponse {
    pub receive: Coin,
}

/// TaxRateResponse is data format returned from TreasuryRequest::TaxRate query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TaxRateResponse {
    pub rate: Decimal,
}

/// TaxCapResponse is data format returned from TreasuryRequest::TaxCap query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TaxCapResponse {
    pub cap: Uint128,
}

/// ExchangeRateItem is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRateItem {
    pub quote_denom: String,
    pub exchange_rate: Decimal,
}

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRatesResponse {
    pub base_denom: String,
    pub exchange_rates: Vec<ExchangeRateItem>,
}
