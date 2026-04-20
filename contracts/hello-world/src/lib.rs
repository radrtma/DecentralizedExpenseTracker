#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // SET INCOME
    pub fn set_income(env: Env, amount: i128) {
        env.storage().instance().set(&symbol_short!("income"), &amount);
    }

    // GET INCOME
    pub fn get_income(env: Env) -> i128 {
        env.storage().instance().get(&symbol_short!("income")).unwrap()
    }

    // SET EXPENSE
    pub fn set_expense(env: Env, amount: i128) {
        env.storage().instance().set(&symbol_short!("expense"), &amount);
    }

    // GET EXPENSE
    pub fn get_expense(env: Env) -> i128 {
        env.storage().instance().get(&symbol_short!("expense")).unwrap()
    }

    // HITUNG SALDO
    pub fn get_balance(env: Env) -> i128 {
        let income = env.storage().instance().get(&symbol_short!("income")).unwrap_or(0);
        let expense = env.storage().instance().get(&symbol_short!("expense")).unwrap_or(0);

        income - expense
    }
}

mod test;