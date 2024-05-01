use serde::{Serialize, Deserialize};
use std::fs;

use tlsn_core::{proof::{SessionProof, SubstringsProof, TlsProof}, SessionHeader};
use p256::pkcs8::DecodePublicKey;
use std::str;


#[derive(Serialize, Deserialize)]
struct ZkParam {
    header: SessionHeader,
    substrings: SubstringsProof,
}


fn build_proof() -> Result<(), Box<dyn std::error::Error>> {
    let proof = std::fs::read_to_string("../host/fixtures/proof.json").unwrap();
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
    let params = ZkParam{
        header,
        substrings,
    };

    let json = serde_json::to_string(&params)?;
    fs::write("../host/fixtures/zk_params.json", json)?;
    Ok(())
}

/// Returns a Notary pubkey trusted by this Verifier
fn notary_pubkey() -> p256::PublicKey {
    let pem_file = str::from_utf8(include_bytes!("../host/fixtures/notary.pub")).unwrap();
    p256::PublicKey::from_public_key_pem(pem_file).unwrap()
}


fn main() {
    let _ = build_proof();
    risc0_build::embed_methods();
}