//! This contract returns a helloworld message.
//! this is derived from rust quick-start:
//! https://docs.near.org/docs/develop/contracts/rust/intro
//!
//!
//! [helloworld]: struct.Helloworld.html#method.helloworld


use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Helloworld {    
}

#[near_bindgen]
impl Helloworld {

    pub fn helloworld(&self, name: String) -> String {
        // I am not good with rust, but format! seems to avoid the type problem
        return format!("{}{}", "hello world! ", name);
    }
}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-counter-tutorial -- --nocapture
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn helloworld() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Helloworld {  };
        let result = contract.helloworld("alice".to_string());
        println!("helloworld returns: {}", result);
        // todo: make proper assert
        //assert_eq!(-1, contract.get_num());
    }
}