//! 生成先と生成コードから、決定的な指紋を計算する。

pub(crate) fn fingerprint(path: &str, normalized_schema: &str) -> [u64; 4] {
    let canonical = format!("{path}\0{normalized_schema}");
    [
        fnv1a(canonical.as_bytes(), 0xcbf29ce484222325),
        fnv1a(canonical.as_bytes(), 0x84222325cbf29ce4),
        fnv1a(canonical.as_bytes(), 0x9e3779b185ebca87),
        fnv1a(canonical.as_bytes(), 0xd6e8feb86659fd93),
    ]
}

pub(crate) fn fnv1a(bytes: &[u8], seed: u64) -> u64 {
    bytes.iter().fold(seed, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}
