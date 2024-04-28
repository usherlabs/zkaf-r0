#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental

extern crate alloc;

use alloc::string::String;
use risc0_zkvm::guest::env;
use tlsn_substrings_verifier::proof::{SessionHeader, SubstringsProof};

risc0_zkvm::guest::entry!(main);



fn main() {
    // read the substring
    let (session_header, substrings): (SessionHeader, SubstringsProof) = env::read();
    let (mut sent, mut recv) = substrings.verify(&session_header).unwrap();
    
    // convert to string
    sent.set_redacted(b'X');
    recv.set_redacted(b'X');

    // log the request and response
    let request = String::from_utf8(sent.data().to_vec()).unwrap();
    let response = String::from_utf8(recv.data().to_vec()).unwrap();


    // write request and response to the journal public output
    env::log("committing data to journal");
    env::commit(&(request, response));
    env::log("committed data to journal");
}
