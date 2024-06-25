#![no_main]
use risc0_zkvm::guest::env;
// use wasmer::{Instance, Module, Store, imports, Function, Value};
// use wasmer_wasi::WasiState;

use wasmtime::*;

risc0_zkvm::guest::entry!(main);

struct MyState {
    name: String,
    count: usize,
}

fn main() {
    // read the substring
    let (session, substrings, session_prover, pub_key): (String, String, String, String) = env::read();

    // Convert session_prover to bytes
    let session_prover_bytes = session_prover.into_bytes();

    // First the wasm module needs to be compiled. This is done with a global
    // "compilation environment" within an `Engine`. Note that engines can be
    // further configured through `Config` if desired instead of using the
    // default like this is here.
    env::log("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_binary(&engine, &session_prover_bytes).unwrap();

    // After a module is compiled we create a `Store` which will contain
    // instantiated modules and other items like host functions. A Store
    // contains an arbitrary piece of host information, and we use `MyState`
    // here.
    env::log("Initializing...");
    let mut store = Store::new(
        &engine,
        MyState {
            name: "hello, world!".to_string(),
            count: 0,
        },
    );

    // Our wasm module we'll be instantiating requires one imported function.
    // the function takes no parameters and returns no results. We create a host
    // implementation of that function here, and the `caller` parameter here is
    // used to get access to our original `MyState` value.
    env::log("Creating callback...");
    let hello_func = Func::wrap(&mut store, |mut caller: Caller<'_, MyState>| {
        env::log("Calling back...");
        env::log(&format!("> {}", caller.data().name));
        caller.data_mut().count += 1;
    });

    // Once we've got that all set up we can then move to the instantiation
    // phase, pairing together a compiled module as well as a set of imports.
    // Note that this is where the wasm `start` function, if any, would run.
    env::log("Instantiating module...");
    let imports = [hello_func.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;

    // Next we poke around a bit to extract the `run` function from the module.
    env::log("Extracting export...");
    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;

    // And last but not least we can call it!
    env::log("Calling export...");
    run.call(&mut store, ())?;

    env::log("Done.");

    // Create a Store
    // let mut store = Store::default();

    // // Compile the WebAssembly binary to a Module
    // let module = match Module::new(&store, &session_prover_bytes) {
    //     Ok(module) => module,
    //     Err(e) => {
    //         env::log(&format!("Error compiling module: {}", e));
    //         return;
    //     }
    // };

    // // Create a WASI environment
    // let wasi_env = match WasiState::new("my_program").finalize(&mut store) {
    //     Ok(env) => env,
    //     Err(e) => {
    //         env::log(&format!("Error creating WASI environment: {}", e));
    //         return;
    //     }
    // };

    // // Create an ImportObject from the WASI env
    // let import_object = match wasi_env.import_object(&mut store, &module) {
    //     Ok(obj) => obj,
    //     Err(e) => {
    //         env::log(&format!("Error creating import object: {}", e));
    //         return;
    //     }
    // };

    // // Instantiate the module
    // let instance = match Instance::new(&mut store, &module, &import_object) {
    //     Ok(inst) => inst,
    //     Err(e) => {
    //         env::log(&format!("Error instantiating module: {}", e));
    //         return;
    //     }
    // };

    // // Access and execute the `add_numbers` function
    // let verify_proof = match instance.exports.get_function("verify_proof") {
    //     Ok(func) => func,
    //     Err(e) => {
    //         env::log(&format!("Error accessing function: {}", e));
    //         return;
    //     }
    // };

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
