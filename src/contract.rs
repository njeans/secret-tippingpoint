use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage, CanonicalAddr,
};
use base64::encode;

use crate::msg::{HandleMsg, InitMsg, QueryMsg, CheckBatchResponse};
use crate::state::*;


pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        owner: deps.api.canonical_address(&env.message.sender)?,
    };
    

    config(&mut deps.storage).save(&state)?;

    // UNDONE(): The following is failing:
    for pharmacist in msg.pharmacists {
        let pharm_can = deps.api.canonical_address(&pharmacist)?;
        let key = [CONFIG_KEY_P, pharm_can.as_slice()];
        let key = key.concat();
        let p_state = true;
        register(&mut deps.storage, &key, &p_state).ok();
    }

    for manufacturer in msg.manufacturers {
        let manuf_can = deps.api.canonical_address(&manufacturer)?;
        let key = [CONFIG_KEY_M, manuf_can.as_slice()];
        let key = key.concat();
        let m_state = true;
        register(&mut deps.storage, &key, &m_state).ok();
    }

    // debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    return match msg {
        HandleMsg::CreateBatch { batch_id, locations, threshold} => try_create_batch(deps, env, batch_id, locations, threshold),
        HandleMsg::AddPatient { symptom_token, batch_id } => try_add_patient(deps, env, symptom_token, batch_id ),
        HandleMsg::AddSymptom { symptom_token, batch_id  } => try_add_symptom(deps, env, symptom_token, batch_id),
    };
}

pub fn try_create_batch<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    bid: BatchId,
    l: Vec<String>,
    t: u64,
) -> StdResult<HandleResponse> {

    let m_id = deps.api.canonical_address(&env.message.sender)?;
    let m_key = [CONFIG_KEY_M, m_id.as_slice()];
    let m_key = m_key.concat();
    let m_exists: bool = load(&deps.storage, &m_key).unwrap_or(false);
    if !m_exists {
        return Err(StdError::GenericErr{
            msg: "Manufacturer id not found".to_string(),
            backtrace: None
        });
    }

    let state = BatchState {
        locations: l,
        threshold: t,
        count : 0
    };

    let batch_key = [CONFIG_KEY_B,&bid.to_be_bytes()];
    let batch_key:&[u8] = &batch_key.concat();

    match register(&mut deps.storage, &batch_key, &state) {
        Ok(_) => {
            // debug_print("batch saved successfully");
            Ok(HandleResponse::default())
        }
        Err(e) => Err(e)
    }
}

pub fn try_add_patient<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    st: SymptomToken,
    bid: BatchId,
) -> StdResult<HandleResponse> {
    let p_id = deps.api.canonical_address(&env.message.sender)?;
    let p_key = [CONFIG_KEY_P, p_id.as_slice()];
    let p_key = p_key.concat();
    let p_exists: bool = load(&deps.storage, &p_key)?;
    if !p_exists {
        return Err(StdError::GenericErr{
            msg: "Pharmacist id not found".to_string(),
            backtrace: None
        });
    }

    let patient_key = [CONFIG_KEY_P,&st.to_be_bytes(),&bid.to_be_bytes()];
    let patient_key:&[u8] = &patient_key.concat();
    let state = false;

    match register(&mut deps.storage, &patient_key, &state) {
        Ok(_) => {
            // debug_print("patient added successfully");
            return Ok(HandleResponse::default());
        }
        Err(e) => Err(e)
    }

}

pub fn try_add_symptom<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    st: SymptomToken,
    bid: BatchId,
) -> StdResult<HandleResponse> {
    let patient_key = [CONFIG_KEY_P,&st.to_be_bytes(),&bid.to_be_bytes()];
    let patient_key:&[u8] = &patient_key.concat();
    let st_used: bool = load(&deps.storage, &patient_key)?;

    if !st_used {
        let mut batch_key = [CONFIG_KEY_B,&bid.to_be_bytes()];
        let batch_key:&[u8] = &batch_key.concat();
        let mut batch_state: BatchState = match load(&deps.storage, &batch_key) {
            Ok(x) => x,
            Err(e) => {
                return Err(e);
            }
        };
        batch_state.count += 1;
        match update(&mut deps.storage, &batch_key, &batch_state) {
            Ok(_) => {
                // debug_print("patient symptom added successfully");
                return Ok(HandleResponse::default());
            }
            Err(e) => Err(e)
        }
    } else {
        return Err(StdError::GenericErr{
            msg: "Symptom token already used".to_string(),
            backtrace: None
        });
    }

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
    let mut key = [CONFIG_KEY_B, &batchId.to_be_bytes()];
    let key = key.concat();
    // The following is failing, why?
    let state: StdResult<BatchState> = load(&deps.storage, &key);
    if !(state.is_ok()) {
        return Err(StdError::GenericErr{
            msg: "Batch id not found".to_string() + &base64::encode(&key),
            backtrace: None
        });
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
