#![no_main]
use risc0_zkvm::guest::env;
use wasmer::{Instance, Module, Store, imports, Function, Value};
use wasmer_wasi::WasiState;

risc0_zkvm::guest::entry!(main);

fn main() {
    // read the substring
    let (session, substrings, session_prover, pub_key): (String, String, String, String) = env::read();

    // Convert session_prover to bytes
    let session_prover_bytes = session_prover.into_bytes();

    // Create a Store
    let mut store = Store::default();

    // Compile the WebAssembly binary to a Module
    let module = match Module::new(&store, &session_prover_bytes) {
        Ok(module) => module,
        Err(e) => {
            env::log(&format!("Error compiling module: {}", e));
            return;
        }
    };

    // Create a WASI environment
    let wasi_env = match WasiState::new("my_program").finalize(&mut store) {
        Ok(env) => env,
        Err(e) => {
            env::log(&format!("Error creating WASI environment: {}", e));
            return;
        }
    };

    // Create an ImportObject from the WASI env
    let import_object = match wasi_env.import_object(&mut store, &module) {
        Ok(obj) => obj,
        Err(e) => {
            env::log(&format!("Error creating import object: {}", e));
            return;
        }
    };

    // Instantiate the module
    let instance = match Instance::new(&mut store, &module, &import_object) {
        Ok(inst) => inst,
        Err(e) => {
            env::log(&format!("Error instantiating module: {}", e));
            return;
        }
    };

    // Access and execute the `add_numbers` function
    let verify_proof = match instance.exports.get_function("verify_proof") {
        Ok(func) => func,
        Err(e) => {
            env::log(&format!("Error accessing function: {}", e));
            return;
        }
    };

    // let result = match verify_proof.call(&mut store, &[Value::String(session.to_string()), Value::String(pub_key.to_string())]) {
    //     Ok(res) => res,
    //     Err(e) => {
    //         env::log(&format!("Error calling function: {}", e));
    //         return;
    //     }
    // };

    // handle deserialization manually
    // ? more efficiently pass bytes instead of strings?
    // let session_header: SessionHeader = serde_json::from_str(&session_header).unwrap();
    // let substrings: SubstringsProof = serde_json::from_str(&substrings).unwrap();

    // let (sent, recv) = substrings.verify(&session_header).unwrap();

    // // Log that we've successfully recovered the request and response...
    // let is_req = !sent.data().to_vec().is_empty();
    // let is_res = !recv.data().to_vec().is_empty();

    // env::log("committing results to journal");
    // env::commit(&(is_req, is_res));
}
