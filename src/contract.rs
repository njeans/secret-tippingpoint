use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg, CheckBatchResponse};
use crate::state::{config, config_read, State, BatchState, BatchId, load, may_load, update, register};
use crate::state::{CONFIG_KEY_B};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    // let state = State {
    //     count: msg.count,
    //     owner: deps.api.canonical_address(&env.message.sender)?,
    // };

    // config(&mut deps.storage).save(&state)?;

    debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    // match msg {
    //     HandleMsg::Increment {} => try_increment(deps, env),
    //     HandleMsg::Reset { count } => try_reset(deps, env, count),
    // }
    Ok(HandleResponse::default())
}

pub fn try_increment<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> StdResult<HandleResponse> {
    // config(&mut deps.storage).update(|mut state| {
    //     state.count += 1;
    //     debug_print!("count = {}", state.count);
    //     Ok(state)
    // })?;

    debug_print("count incremented successfully");
    Ok(HandleResponse::default())
}

pub fn try_reset<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    count: i32,
) -> StdResult<HandleResponse> {
    // let sender_address_raw = deps.api.canonical_address(&env.message.sender)?;
    // config(&mut deps.storage).update(|mut state| {
    //     if sender_address_raw != state.owner {
    //         return Err(StdError::Unauthorized { backtrace: None });
    //     }
    //     state.count = count;
    //     Ok(state)
    // })?;
    debug_print("count reset successfully");
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::CheckBatch { batch_id } => to_binary(&query_check_batch(&deps, batch_id)?),
    }
}

fn query_check_batch<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, batchId: BatchId) -> StdResult<CheckBatchResponse> {
    let mut key = [CONFIG_KEY_B, &batchId];
    let key = key.concat();
    let state: StdResult<BatchState> = load(&deps.storage, &key);
    if (state.is_err()) {
        return Err(StdError::Unauthorized { backtrace: None });
    }
    let state = state.ok().unwrap();

    if (state.count >= state.threshold) {
        return Ok(CheckBatchResponse { threshold_reached: true, locations: state.locations });
    } else {
        return Ok(CheckBatchResponse { threshold_reached: false, locations: state.locations });
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env};
//     use cosmwasm_std::{coins, from_binary, StdError};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies(20, &[]);

//         let msg = InitMsg { count: 17 };
//         let env = mock_env("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = init(&mut deps, env, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(&deps, QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies(20, &coins(2, "token"));

//         let msg = InitMsg { count: 17 };
//         let env = mock_env("creator", &coins(2, "token"));
//         let _res = init(&mut deps, env, msg).unwrap();

//         // anyone can increment
//         let env = mock_env("anyone", &coins(2, "token"));
//         let msg = HandleMsg::Increment {};
//         let _res = handle(&mut deps, env, msg).unwrap();

//         // should increase counter by 1
//         let res = query(&deps, QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies(20, &coins(2, "token"));

//         let msg = InitMsg { count: 17 };
//         let env = mock_env("creator", &coins(2, "token"));
//         let _res = init(&mut deps, env, msg).unwrap();

//         // not anyone can reset
//         let unauth_env = mock_env("anyone", &coins(2, "token"));
//         let msg = HandleMsg::Reset { count: 5 };
//         let res = handle(&mut deps, unauth_env, msg);
//         match res {
//             Err(StdError::Unauthorized { .. }) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let auth_env = mock_env("creator", &coins(2, "token"));
//         let msg = HandleMsg::Reset { count: 5 };
//         let _res = handle(&mut deps, auth_env, msg).unwrap();

//         // should now be 5
//         let res = query(&deps, QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
