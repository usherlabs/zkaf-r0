
# WIP: ZK Attestation Framework - by [Usher Labs](https://www.usher.so)

## Supported platforms

- **Linux x86-64**

Docker must operate on a compatible OS to work.
**Apple Silicon:** Is not compatible even with `--platform linux/amd64`.

## Prerequisites
- `RUST`: The RISC Zero zkVM requires Rust, so start by [installing Rust and  `rustup`](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you don't already have it. Please note that you will need to follow the recommended Rust installation instructions that use [rustup](https://rustup.rs/) rather than any of the alternative Rust installation options, as RISC Zero depends on the [rustup](https://rustup.rs/) tool specifically.

- `CUDA` If you intend to run this `GPU` in order to reduce proof generation time, then you would need to install the required dependencies. Instructions on how to install and validate your installation of the dependencies on ubuntu can be found [here](https://docs.nvidia.com/cuda/cuda-installation-guide-linux/index.html#ubuntu).


## [Installation](https://dev.risczero.com/api/zkvm/install)
Next, install the  [`cargo risczero`](https://crates.io/crates/cargo-risczero)  tool and use its  [`install`  command](https://crates.io/crates/cargo-risczero)  to install the toolchain by running:

```
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
```

If this is successful, it will finish by printing the message

```
The risc0 toolchain is now ready to use.
```

You can verify the toolchain was installed correctly by running

```
rustup toolchain list --verbose | grep risc0
```

which should list  `risc0`  along with its path.

## Generating the data.
```
cd zkaf-r0
cargo build --release
```
The process for generating a proof for a tlsn proof can be broken down as showed below:

### Generating the TLS proof:
After a TLS proof for a tweet has been generated, it is then placed in the `fixtures` directory [here](https://github.com/usherlabs/zkaf-r0/blob/master/host/fixtures/twitter_proof.json).

### Generating ZK private inputs:
Following the previous step, a build script which can be found [here](https://github.com/usherlabs/zkaf-r0/blob/master/methods/build.rs) is used to validate TLS Proof's Session and then generates the inputs to the zk circuit.
```
struct ZkParam {
    header: SessionHeader,
    substrings: SubstringsProof,
}
```


## Proof Generation 
A proof can either be verified on a CPU or on a GPU.

### GPU-based
To generate a zkProof using `GPU` acceleration, we first confirm we have installed `CUDA`  as directed in the prerequisites. 
CUDA is a feature flag included at the end of the `Cargo.toml` file of the `host` directory.
```
[features]
cuda = ["risc0-zkvm/cuda"]
```

The proof can be generated by running `cargo run --release -F cuda` at the root of the project.

### CPU-based

The proof can be generated by running `cargo run --release` at the root of the project.

Note: Running the proof generation process on a CPU can be time-consuming so it can be run in dev mode by 
`RISC0_DEV_MODE=1 cargo run --release` at the root of the project


