# Tokio: Running async/await tasks, network communication, handling concurrency, managing timers and signals. 
cargo add tokio --features full

# Axum: Web applications, RESTful APIs, type-safe route handling, easy middleware integration (tracing, auth, compression).
cargo add axum

# Sqlx-cli manages database interactions, specifically for migrations (schema changes) and compile-time SQL checking
cargo add sqlx --features postgres,runtime-tokio,chrono,uuid,macros

# serde_json handle json
cargo add serde_json

# Serde used to derive struct serialization / deserialization
cargo add serde --features derive

# Chorno used for Date functions
cargo add chrono --no-default-features -F serde -F clock

# Validator used for validation serde value
cargo add validator --features derive

# Dotenv used for environment access
cargo add dotenvy

# UUID feature access
cargo add uuid --features v4,serde
