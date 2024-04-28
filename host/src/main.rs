use core::str;

use methods::ZKAF_ELF;
use p256::pkcs8::DecodePublicKey;
use risc0_zkvm::{default_prover, ExecutorEnv};
use tlsn_core::proof::{SessionProof, TlsProof};


use tlsn_substrings_verifier::{self, proof::{SessionHeader, SubstringsProof}};

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
        ..
    } = session;


    // type conversion occurs here
    // we need to convert from the tlsn core definitions to the definitions from the verifier
    let verifier_parsed_header: SessionHeader = serde_json::from_str(&serde_json::to_string(&header).unwrap()).unwrap();
    let verifier_parsed_substrings: SubstringsProof = serde_json::from_str(&serde_json::to_string(&substrings).expect("Serialization failed")).unwrap();

    // pass the input to the guest code
    let input: (SessionHeader, SubstringsProof) = (verifier_parsed_header, verifier_parsed_substrings);
    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, ZKAF_ELF).unwrap();

    // Extract journal of receipt
    let (request, response): (String, String)= receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("I generated a proof of guest execution!");
    println!("Request: \n{}", request);
    println!("Response: \n {}", response);
}

/// Returns a Notary pubkey trusted by this Verifier
fn notary_pubkey() -> p256::PublicKey {
    let pem_file = str::from_utf8(include_bytes!("../../fixtures/notary.pub")).unwrap();
    p256::PublicKey::from_public_key_pem(pem_file).unwrap()
}
