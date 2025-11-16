///use blake2b_rfc::blake2b::Blake2b;
///use blake2b_rfc::blake2b;
use blake2b_rfc::Params as Blake2bParams;

use jubjub::{JubjubEngine, ToUniform};

pub fn hash_to_scalar<E: JubjubEngine>(persona: &[u8], a: &[u8], b: &[u8]) -> E::Fs {
///    let mut hasher = blake2b::with_params(64, &[], &[], persona);
    use blake2b_rfc::Params as Blake2bParams;

    let mut hasher = Blake2bParams::new()
        .hash_length(64)
        .personal(persona)
        .to_state();
    hasher.update(a);
    hasher.update(b);
    let ret = hasher.finalize();
    E::Fs::to_uniform(ret.as_ref())
}
