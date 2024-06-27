cargo clean
rm call-sites.json
rm graph.dat
rm graph.dot
rm graph.svg
rm types.json
echo "Cleanup done."
sleep 1
echo "Running MIRAI analysis..."
export MIRAI_FLAGS="--call_graph_config $(pwd)/cg-config.json --single_func main"
cargo mirai --release
cat graph.dot | dot -Tsvg -o graph.svg
