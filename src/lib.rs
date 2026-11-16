//! Helpers shared by the experiments of the PHastR paper: the parser of
//! PHastR configurations, and a key type hashed as the reference
//! implementation of PHast/PHast+ (`ph`) does.

/// Parses a PHastR configuration `<S>:<log2 L>:<lambda>[:<log2 R>]`: bits
/// per seed, base-2 logarithm of the slice length, expected bucket size,
/// and base-2 logarithm of the number of layouts (2 if missing). Returns
/// the builder, the number of bits per seed, and a description.
pub fn parse_config(fields: &[&str]) -> (sux::func::PHastRBuilder, u32, String) {
    let s: u32 = fields[0].parse().unwrap();
    let ll: u32 = fields[1].parse().unwrap();
    let lam: f64 = fields[2].parse().unwrap();
    let lr: u32 = fields.get(3).map(|x| x.parse().unwrap()).unwrap_or(2);
    (
        sux::func::PHastRBuilder::default()
            .seed_bits(s)
            .log2_slice_len(ll)
            .bucket_size(lam)
            .log2_layouts(lr),
        s,
        format!("PHast-R S={s} L={} R={} l={lam}", 1 << ll, 1 << lr),
    )
}

/// A 64-bit key hashed by sux with GxHash, exactly as `ph::BuildGxHash`
/// hashes a `u64` (`GxHasher::with_seed`, `write_u64`, `finish`), so that
/// PHastR and the reference implementation pay the same hashing cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct GxKey(pub u64);

impl sux::utils::ToSig<[u64; 1]> for GxKey {
    #[inline(always)]
    fn to_sig(key: impl std::borrow::Borrow<Self>, seed: u64) -> [u64; 1] {
        use std::hash::Hasher;
        let mut h = gxhash::GxHasher::with_seed(seed as i64);
        h.write_u64(key.borrow().0);
        [h.finish()]
    }
}
