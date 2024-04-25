use methods::{HELLO_GUEST_ELF, HELLO_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use p256::pkcs8::DecodePublicKey;
pub mod types;

use std::str;
use std::time::Duration;

use tlsn_core::{
    merkle::MerkleProof,
    proof::{SessionProof, TlsProof},
};

use types::{convert_hashmap_from_std_to_brown, PublicSubstringsProof};
use crate::types::{CustomHashMap, GuestSubstringsProof, OpeningsHashMap};
use tlsn_substrings_verifier::proof::SessionHeader;

fn main() {
    // derive the header and the sessionsubstring
        // Deserialize the proof
    let proof = std::fs::read_to_string("fixtures/proof.json").unwrap();
    let proof: TlsProof = serde_json::from_str(proof.as_str()).unwrap();

    let TlsProof {
        // The session proof establishes the identity of the server and the commitments
        // to the TLS transcript.
        session,
        // The substrings proof proves select portions of the transcript, while redacting
        // anything the Prover chose not to disclose.
        substrings,
    } = proof;

    // Verify the session proof against the Notary's public key
    //
    // This verifies the identity of the server using a default certificate verifier which trusts
    // the root certificates from the `webpki-roots` crate.
    session
        .verify_with_default_cert_verifier(notary_pubkey())
        .unwrap();

    let SessionProof {
        // The session header that was signed by the Notary is a succinct commitment to the TLS transcript.
        header,
        // This is the server name, checked against the certificate chain shared in the TLS handshake.
        server_name,
        ..
    } = session;
    
    // The time at which the session was recorded
    let time = chrono::DateTime::UNIX_EPOCH + Duration::from_secs(header.time());



    // type conversion occurs here
    // we need to convert the types to the no_std counterpart used in the zk circuit.
    // so we extract the parameters, convert them ourselves
    // and reconstruct the struct to mirror the zk circuit counterpart.
    let serialized_substring = serde_json::to_string(&substrings).expect("Serialization failed");
    let public_substring: PublicSubstringsProof =
        serde_json::from_str(&serialized_substring).expect("DeSerialization failed");

    let parsed_inclusion_proof: MerkleProof =
        serde_json::from_str(&serde_json::to_string(&public_substring.inclusion_proof).unwrap())
            .expect("Deserialization failed");

    let parsed_openings: OpeningsHashMap =
        convert_hashmap_from_std_to_brown(public_substring.openings);

    let parsed_openings = CustomHashMap(parsed_openings);
   
    // the final struct which mirrors the expected struct we use in the zk circuit.
    let guest_formatted_prooof = GuestSubstringsProof {
        openings: parsed_openings,
        inclusion_proof: parsed_inclusion_proof,
    };

    // let serialized_substring = serde_json::to_string(&guest_formatted_prooof).expect("Serialization failed");
    let guest_formatted_header: SessionHeader = serde_json::from_str(&serde_json::to_string(&header).unwrap()).unwrap();


    let input: (SessionHeader, GuestSubstringsProof) = (guest_formatted_header, guest_formatted_prooof);
    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, HELLO_GUEST_ELF).unwrap();

    // Extract journal of receipt
    let output: u32= receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("Hello, world! I generated a proof of guest execution! {:?} is a public output from journal ", output);
}

/// Returns a Notary pubkey trusted by this Verifier
fn notary_pubkey() -> p256::PublicKey {
    let pem_file = str::from_utf8(include_bytes!("../../fixtures/notary.pub")).unwrap();
    p256::PublicKey::from_public_key_pem(pem_file).unwrap()
}
