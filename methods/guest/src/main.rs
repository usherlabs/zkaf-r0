#![no_main]
use risc0_zkvm::guest::env;

use tlsn_core::proof::{SessionProof, TlsProof};
use elliptic_curve::pkcs8::DecodePublicKey;

risc0_zkvm::guest::entry!(main);


fn main() {
    // read the substring
    let (proof, pub_key): (String, String) = env::read();

    let tls_proof: TlsProof = serde_json::from_str(proof.as_str()).unwrap();

    let TlsProof {
        session,
        substrings,
    } = tls_proof;

    // Verify the session proof against the Notary's public key
    //
    // This verifies the identity of the server using a default certificate verifier which trusts
    // the root certificates from the `webpki-roots` crate.
    let pub_key = match p256::PublicKey::from_public_key_pem(pub_key.as_str()) {
        Ok(key) => key,
        Err(e) => panic!("INVALID PUBLIC KEY: {:?}", e),
    };
    match session.verify_with_default_cert_verifier(pub_key) {
        Ok(_) => (),
        Err(e) => panic!("FAILED TO VERIFY SESSION: {:?}", e),
    };

    let SessionProof {
        // The session header that was signed by the Notary is a succinct commitment to the TLS transcript.
        header,
        // This is the session_info, which contains the server_name, that is checked against the
        // certificate chain shared in the TLS handshake.
        // session_info,
        ..
    } = session;

    let (mut sent, mut recv) = substrings.verify(&header).unwrap();

    // Replace the bytes which the Prover chose not to disclose with 'X'
    sent.set_redacted(b'X');
    recv.set_redacted(b'X');

    // Log that we've successfully recovered the request and response...
    let is_req = !sent.data().to_vec().is_empty();
    let is_res = !recv.data().to_vec().is_empty();

    env::log("committing results to journal");
    env::commit(&(is_req, is_res));
}
