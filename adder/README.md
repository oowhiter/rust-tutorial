[tests](tests) : 結合テスト

```sh
# テストスレッドを1つにして逐次実行する
cargo test -- --test-threads=1
# 出力をキャプチャしない
cargo test -- --nocapture
# 特定のテスト関数のみを実行する
cargo test exploration
# 特定のテスト関数のみを実行する
cargo test can
# ignoredテストのみを実行する
cargo test -- --ignored
# 特定の統合テストのみを実行する
cargo test --test integration_test
# 統合テストのみを実行する
cargo test --test '*'
```
