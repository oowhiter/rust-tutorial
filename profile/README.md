### Cargo.toml

```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

### rustdoc

```sh
# ドキュメントをビルドしブラウザで開く
cargo doc --open
```
