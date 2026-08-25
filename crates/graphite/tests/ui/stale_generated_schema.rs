struct Person;

#[allow(non_snake_case)]
mod Stale {
    pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [0; 4];
}

graphite::graph_schema! {
    generated = "generated/stale.rs";
    schema Stale {
        node Person;
    }
}

fn main() {}
