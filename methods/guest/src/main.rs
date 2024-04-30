#![no_main]
use risc0_zkvm::guest::env;
use tlsn_substrings_verifier::proof::{SessionHeader, SubstringsProof};

risc0_zkvm::guest::entry!(main);


fn main() {
    // read the substring
    let (session_header, substrings): (SessionHeader, SubstringsProof) = env::read();
    let (sent, recv) = substrings.verify(&session_header).unwrap();

    // set redacted string value
    // sent.set_redacted(b'X');
    // recv.set_redacted(b'X');

    // recover the request and response
    let request = String::from_utf8(sent.data().to_vec()).unwrap();
    let response = String::from_utf8(recv.data().to_vec()).unwrap();

    // Log that we've successfully recovered the request and response...
    let is_req = !request.is_empty();
    let is_res = !response.is_empty();

    env::log("committing results to journal");
    env::commit(&(is_req, is_res));

    // // write request and response to the journal public output
    // env::log("committing data to journal");
    // env::commit(&(request, response));
    // env::log("committed data to journal");
}
