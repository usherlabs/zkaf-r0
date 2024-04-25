#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental

extern crate alloc;

use alloc::string::{String, ToString};
use risc0_zkvm::guest::env;
use tlsn_substrings_verifier::proof::{SessionHeader, SubstringsProof};

risc0_zkvm::guest::entry!(main);



fn main() {
    // TODO: Implement your guest code here

    // read the substring
    // logging this data does produce results
    let (session_header, substrings): (SessionHeader, SubstringsProof) = env::read();

    // trying to reconstruct structs from strings throw an error
    // // reconstruct the header and substring
    // let substrings: SubstringsProof = serde_json::from_str(&serialized_substrings).expect("Deserialization failed");
    // let header: SessionHeader = serde_json::from_str(&serialized_header)
    //     .expect("Deserialization failed");

    // however calling this function throws an error, the same error from above.
    let (mut sent, mut recv) = substrings.verify(&session_header).unwrap();
    
    // convert to string
    sent.set_redacted(b'X');
    recv.set_redacted(b'X');

    // log the request and response
    let request = String::from_utf8(sent.data().to_vec()).unwrap();
    let response = String::from_utf8(recv.data().to_vec()).unwrap();

    env::log("request");
    env::log(&request);

    env::log("response");
    env::log(&response);


    // write public output to the journal
    // env::commit(&request);
    env::log("committing data to journal");
    env::commit(&(request, response));
    env::log("committed data to journal");

}
