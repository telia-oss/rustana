## Rustana is a client for Grafana

# Installation
Rustana is no available on crates.io ye. To use Rustana in your Rust program built with Cargo, add it as a dependency.

```toml
[dependencies]
rustana = { git = "https://github.com/telia-oss/rustana.git", tag = "0.0.1" }
```

# Usage
Rustana containing Rust types for Grafana's API.
```rust
use rustana::GrafanaClient;

let mut client = GrafanaClient::new(&url, &token);

fn main() {
    let dashboard_id = "FT5SF";
    match client.get_dashboard_by_id(&dashboard_id) {
        Ok(dashboard) => println!("Dashboard response: {:?}", dashboard),
        Err(e) => println!("error fetching dashboard: {:?}", e),
    }

}

```