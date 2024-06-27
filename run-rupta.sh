export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
# export PTA_LOG=info
cargo clean
/home/endericedragon/repos/rupta/target/release/cargo-pta pta --release -- --entry-func main --dump-call-graph cg.dot
cat cg.dot | dot -Tsvg -o cg.svg
