#! env /bin/zsh

cd ./tree-sitter-rlang
tree-sitter generate

cd ../rlang-rust
clear
# cargo run -- -d
cargo run 
