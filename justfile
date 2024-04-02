default:
    @cargo build

test *targets:
    @cargo test {{targets}} --lib -- --nocapture

qtest *targets:
    @cargo test {{targets}} --lib