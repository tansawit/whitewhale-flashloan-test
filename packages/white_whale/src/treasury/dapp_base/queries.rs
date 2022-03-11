use cosmwasm_std::{to_binary, Binary, Deps, StdResult};

use crate::treasury::dapp_base::msg::{BaseQueryMsg, BaseStateResponse};
use crate::treasury::dapp_base::state::BASESTATE;

/// Handles the common base queries
pub fn handle_base_query(deps: Deps, query: BaseQueryMsg) -> StdResult<Binary> {
    match query {
        BaseQueryMsg::Config {} => to_binary(&try_query_config(deps)?),
    }
}
/// Returns the BaseState
pub fn try_query_config(deps: Deps) -> StdResult<BaseStateResponse> {
    let state = BASESTATE.load(deps.storage)?;

    Ok(BaseStateResponse {
        treasury_address: state.treasury_address.into_string(),
        trader: state.trader.into_string(),
        memory_address: state.memory.address.into_string(),
    })
}
