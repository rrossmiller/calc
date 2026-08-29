#! env /bin/zsh

cd ./tree-sitter-calc
tree-sitter generate

cd ../calc-rs
clear
# cargo run -- -d
cargo run 
