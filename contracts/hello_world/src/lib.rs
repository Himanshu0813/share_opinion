// Working
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, String, Symbol, symbol_short};

const COUNT_OP : Symbol = symbol_short!("COUNT_OP");

#[contracttype]
pub enum OPbook {Feedback(u32)}

#[contracttype]
#[derive(Clone)]
pub struct Feedback {opinion_id: u32,message: String,}

#[contract]
pub struct ShareOpinion;

#[contractimpl]
impl ShareOpinion {
    pub fn post_opinion(env: Env, opinion_msg: String) -> u32 {
        let mut opinion_count: u32 = env.storage().instance().get(&COUNT_OP).unwrap_or(0);
            opinion_count += 1;
        let mut opinion_details = Self::fetch_opinion(env.clone(), opinion_count.clone());
        opinion_details.opinion_id = opinion_count;
        opinion_details.message = opinion_msg;
        env.storage().instance().set(&OPbook:: Feedback(opinion_details.opinion_id.clone()), &opinion_details);
        env.storage().instance().set(&COUNT_OP, &opinion_details.opinion_id.clone());
        env.storage().instance().extend_ttl(5000, 5000);
        return opinion_details.opinion_id;        
    }

    pub fn fetch_opinion(env: Env, opinion_id: u32) -> Feedback{
        let key = OPbook::Feedback(opinion_id.clone());
        env.storage().instance().get(&key).unwrap_or(Feedback {opinion_id: 0, message: String::from_str(&env, "Invalid opinion ID!")})
    }
}