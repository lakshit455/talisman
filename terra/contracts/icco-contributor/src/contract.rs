use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use icco::common::CHAIN_ID;

use crate::{
    error::ContributorError,
    execute::{
        attest_contributions, claim_allocation, claim_refund, contribute, init_sale, sale_aborted,
        sale_sealed,
    },
    hooks::escrow_user_contribution_hook,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    query::{
        query_accepted_asset, query_asset_index, query_buyer_status, query_config, query_sale,
        query_sale_status, query_sale_times, query_total_allocation, query_total_contribution,
    },
    state::{Config, CONFIG},
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let wormhole = deps.api.addr_validate(msg.wormhole.as_str())?;
    let token_bridge = deps.api.addr_validate(msg.token_bridge.as_str())?;

    // we know there is no terra conductor existing. So prevent user
    // from instantiating with one defined
    match msg.conductor_chain {
        CHAIN_ID => return ContributorError::UnsupportedConductor.std_err(),
        _ => {
            let cfg = Config {
                wormhole,
                token_bridge,
                conductor_chain: msg.conductor_chain,
                conductor_address: msg.conductor_address.into(),
                owner: info.sender,
            };
            CONFIG.save(deps.storage, &cfg)?;
            Ok(Response::default())
        }
    }
}

// When CW20 transfers complete, we need to verify the actual amount that is being transferred out
// of the bridge. This is to handle fee tokens where the amount expected to be transferred may be
// less due to burns, fees, etc.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::InitSale { signed_vaa } => init_sale(deps, env, info, signed_vaa),
        ExecuteMsg::Contribute {
            sale_id,
            token_index,
            amount,
        } => contribute(deps, env, info, sale_id.as_slice(), token_index, amount),
        ExecuteMsg::EscrowUserContributionHook {} => escrow_user_contribution_hook(deps, env, info),
        ExecuteMsg::AttestContributions { sale_id } => {
            attest_contributions(deps, env, info, &sale_id)
        }
        ExecuteMsg::SaleSealed { signed_vaa } => sale_sealed(deps, env, info, signed_vaa),
        ExecuteMsg::ClaimAllocation {
            sale_id,
            token_index,
        } => claim_allocation(deps, env, info, &sale_id, token_index),
        ExecuteMsg::SaleAborted { signed_vaa } => sale_aborted(deps, env, info, signed_vaa),
        ExecuteMsg::ClaimRefund {
            sale_id,
            token_index,
        } => claim_refund(deps, env, info, &sale_id, token_index),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Sale { sale_id } => to_binary(&query_sale(deps, &sale_id)?),
        QueryMsg::SaleStatus { sale_id } => to_binary(&query_sale_status(deps, &sale_id)?),
        QueryMsg::SaleTimes { sale_id } => to_binary(&query_sale_times(deps, &sale_id)?),
        QueryMsg::TotalContribution {
            sale_id,
            token_index,
        } => to_binary(&query_total_contribution(
            deps,
            sale_id.as_slice(),
            token_index,
        )?),
        QueryMsg::TotalAllocation {
            sale_id,
            token_index,
        } => to_binary(&query_total_allocation(
            deps,
            sale_id.as_slice(),
            token_index,
        )?),
        QueryMsg::AcceptedAsset {
            sale_id,
            token_index,
        } => to_binary(&query_accepted_asset(
            deps,
            sale_id.as_slice(),
            token_index,
        )?),
        QueryMsg::AssetIndex {
            sale_id,
            asset_info,
        } => to_binary(&query_asset_index(deps, sale_id.as_slice(), asset_info)?),
        QueryMsg::BuyerStatus {
            sale_id,
            token_index,
            buyer,
        } => to_binary(&query_buyer_status(
            deps,
            sale_id.as_slice(),
            token_index,
            buyer,
        )?),
    }
}