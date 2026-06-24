#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct WalletConfig {
    pub owner: Address,
    pub guardians: Vec<Address>,
    pub recovery_status: bool,
}

const WALLET_DATA: Symbol = symbol_short!("WAL_DATA");

#[contract]
pub struct BioWalletContract;

#[contractimpl]
impl BioWalletContract {
    pub fn get_wallet_config(env: Env) -> Vec<WalletConfig> {
        env.storage().instance().get(&WALLET_DATA).unwrap_or(Vec::new(&env))
    }

    pub fn register_wallet(env: Env, owner: Address, guardians: Vec<Address>) -> String {
        owner.require_auth();

        let mut configs: Vec<WalletConfig> = env.storage().instance().get(&WALLET_DATA).unwrap_or(Vec::new(&env));
        let config = WalletConfig {
            owner,
            guardians,
            recovery_status: false,
        };

        configs.push_back(config);
        env.storage().instance().set(&WALLET_DATA, &configs);

        String::from_str(&env, "Wallet registered successfully")
    }

    pub fn initiate_recovery(env: Env, owner: Address) -> String {
        let mut configs: Vec<WalletConfig> = env.storage().instance().get(&WALLET_DATA).unwrap_or(Vec::new(&env));
        let mut found = false;

        for i in 0..configs.len() {
            let mut conf = configs.get(i).unwrap();
            if conf.owner == owner {
                conf.recovery_status = true;
                configs.set(i, conf);
                env.storage().instance().set(&WALLET_DATA, &configs);
                found = true;
                break;
            }
        }

        if found {
            String::from_str(&env, "Social recovery initiated")
        } else {
            String::from_str(&env, "Wallet configuration not found")
        }
    }
}

mod test;
