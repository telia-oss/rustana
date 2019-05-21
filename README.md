## Rustana is a client for Grafana

# Installation
Rustana is available on [crates.io](https://crates.io/crates/rustana). To use Rustana in your Rust program built with Cargo, add it as a dependency.

```toml
[dependencies]
rustana = "0.0.2"
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