#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


extern crate alloc;
use risc0_zkvm::guest::env;
use alloc::string::ToString;

risc0_zkvm::guest::entry!(main);

fn main() {
    // // Implement your guest code here

    // read the input
    let (input, input_2): (u32, u32) = env::read();

    // do something with the input
    assert_eq!(input, input_2);

    let proof = zkaf_guest::constants::PROOF;
    let pub_key = zkaf_guest::constants::PUB_KEY;
    let res = zkaf_guest::utils::verify_proof(&proof.to_string(), &pub_key.to_string());

    // write public output to the journal
    env::commit(&input);
}
