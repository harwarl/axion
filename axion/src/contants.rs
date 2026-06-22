pub const DEPENDENCIES: &[&str] = &[
    "async-trait",
    "axum",
    "dotenv",
    "futures-util",
    "tokio --features full",
    "serde --features derive",
    "serde_json",
    "tower",
    "tracing",
    "tracing-subscriber --features tracing",
];
