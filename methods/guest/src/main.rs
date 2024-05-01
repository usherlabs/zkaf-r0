#![no_main]
use risc0_zkvm::guest::env;
use tlsn_substrings_verifier::proof::{SessionHeader, SubstringsProof};

risc0_zkvm::guest::entry!(main);


fn main() {
    // read the substring
    let (session_header,  substrings): (String, String) = env::read();

    // handle deserialization manually
    // ? more efficiently pass bytes instead of strings?
    let session_header: SessionHeader = serde_json::from_str(&session_header).unwrap();
    let substrings: SubstringsProof = serde_json::from_str(&substrings).unwrap();

    let (sent, recv) = substrings.verify(&session_header).unwrap();

    // Log that we've successfully recovered the request and response...
    let is_req = !sent.data().to_vec().is_empty();
    let is_res = !recv.data().to_vec().is_empty();

    env::log("committing results to journal");
    env::commit(&(is_req, is_res));
}
