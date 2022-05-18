use cosmwasm_std::{Deps, StdResult};
use terraswap::asset::AssetInfo;

use icco::common::SaleCore;

use crate::{
    error::ContributorError,
    msg::{
        AcceptedAssetResponse, BuyerStatusResponse, SaleResponse, SaleStatusResponse,
        SaleTimesResponse, TotalAllocationResponse, TotalContributionResponse,
    },
    state::{
        Config, ACCEPTED_ASSETS, ASSET_INDICES, BUYER_STATUSES, CONFIG, SALES, SALE_STATUSES,
        SALE_TIMES, TOTAL_ALLOCATIONS, TOTAL_CONTRIBUTIONS,
    },
};

pub fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_sale(deps: Deps, sale_id: &[u8]) -> StdResult<SaleResponse> {
    Ok(SaleResponse {
        id: sale_id.to_vec(),
        sale: SALES.load(deps.storage, sale_id)?,
    })
}

pub fn query_sale_status(deps: Deps, sale_id: &[u8]) -> StdResult<SaleStatusResponse> {
    Ok(SaleStatusResponse {
        id: sale_id.to_vec(),
        status: SALE_STATUSES.load(deps.storage, sale_id)?,
    })
}

pub fn query_sale_times(deps: Deps, sale_id: &[u8]) -> StdResult<SaleTimesResponse> {
    Ok(SaleTimesResponse {
        id: sale_id.to_vec(),
        times: SALE_TIMES.load(deps.storage, sale_id)?,
    })
}

pub fn query_total_contribution(
    deps: Deps,
    sale_id: &[u8],
    token_index: u8,
) -> StdResult<TotalContributionResponse> {
    Ok(TotalContributionResponse {
        id: sale_id.to_vec(),
        token_index,
        amount: TOTAL_CONTRIBUTIONS.load(deps.storage, (sale_id, token_index.into()))?,
    })
}

pub fn query_total_allocation(
    deps: Deps,
    sale_id: &[u8],
    token_index: u8,
) -> StdResult<TotalAllocationResponse> {
    Ok(TotalAllocationResponse {
        id: sale_id.to_vec(),
        token_index,
        allocation: TOTAL_ALLOCATIONS.load(deps.storage, (sale_id, token_index.into()))?,
    })
}

pub fn query_accepted_asset(
    deps: Deps,
    sale_id: &[u8],
    token_index: u8,
) -> StdResult<AcceptedAssetResponse> {
    Ok(AcceptedAssetResponse {
        id: sale_id.to_vec(),
        token_index,
        asset_info: ACCEPTED_ASSETS.load(deps.storage, (sale_id, token_index.into()))?,
    })
}

pub fn query_asset_index(
    deps: Deps,
    sale_id: &[u8],
    asset_info: AssetInfo,
) -> StdResult<AcceptedAssetResponse> {
    let asset = match asset_info.clone() {
        AssetInfo::NativeToken { denom } => denom,
        AssetInfo::Token { contract_addr } => contract_addr,
    };

    Ok(AcceptedAssetResponse {
        id: sale_id.to_vec(),
        asset_info,
        token_index: ASSET_INDICES.load(deps.storage, (sale_id, asset))?,
    })
}

pub fn query_buyer_status(
    deps: Deps,
    sale_id: &[u8],
    token_index: u8,
    buyer: String,
) -> StdResult<BuyerStatusResponse> {
    let asset_info = ACCEPTED_ASSETS.load(deps.storage, (sale_id, token_index.into()))?;

    let validated = deps.api.addr_validate(buyer.as_str())?;
    let status = BUYER_STATUSES.load(deps.storage, (sale_id, token_index.into(), validated))?;

    Ok(BuyerStatusResponse {
        id: sale_id.to_vec(),
        token_index,
        buyer,
        asset_info,
        status,
    })
}