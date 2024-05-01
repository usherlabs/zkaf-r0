use core::str;

use methods::{ZKAF_ELF, ZKAF_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};


use tlsn_substrings_verifier::{self, proof::{SessionHeader, SubstringsProof}, merkle::MerkleRoot};

#[derive(Serialize, Deserialize, Debug)]
struct ZkParam {
    header: SessionHeader,
    substrings: SubstringsProof,
}

fn main() {
    // Obtain the default prover.
    let prover = default_prover();

    // read in th einputs from json
    let proof_params = std::fs::read_to_string("inputs/zk_params.json").unwrap();
    let proof_params: ZkParam = serde_json::from_str(proof_params.as_str()).unwrap();

    // pass the input to the guest code
    // let input: (SessionHeader, SubstringsProof) = (proof_params.header, proof_params.substrings);
    let input: ([u8; 32], MerkleRoot, usize, usize) = ([
        168,
        221,
        97,
        226,
        163,
        161,
        86,
        84,
        159,
        109,
        125,
        195,
        4,
        170,
        2,
        197,
        18,
        67,
        205,
        141,
        143,
        61,
        88,
        21,
        166,
        227,
        122,
        78,
        18,
        126,
        151,
        170
    ], MerkleRoot::from([
        86,
        15,
        84,
        245,
        15,
        231,
        162,
        234,
        78,
        122,
        38,
        20,
        100,
        4,
        183,
        199,
        216,
        164,
        191,
        48,
        229,
        202,
        147,
        145,
        40,
        59,
        66,
        163,
        209,
        105,
        153,
        61
    ]), 388, 1939);
    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, ZKAF_ELF).unwrap();

    // Extract journal of receipt
    let (request, response): (bool, bool)= receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("I generated a proof of guest execution!");
    println!("Request: \n{}", request);
    println!("Response: \n {}", response);


    receipt.verify(ZKAF_ID).unwrap();

    println!("Generated proof is verified!");
}
