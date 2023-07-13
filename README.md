# Tinyterp
(Work in progress)

純粋なRustで作られたシンプルなインタプリタ言語。
[チュートリアル](https://github.com/uthree/tinyterp/blob/main/tutorial.md)

## インストール
このリポジトリをクローンして
```
cargo run
```
することでREPLを起動できます。
```
cargo run <ファイル名>
```
でファイルを実行することができます。

## ライブラリとして使う
```rust
use tinyterp::Runtime;
```