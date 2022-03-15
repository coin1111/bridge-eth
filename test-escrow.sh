transfer_id=aab47fa3a3dc42bc8cbc48c02182669b
# deposit pete to transfer to todd
cargo run --package bridge-eth --bin bridge-eth -- deposit pete  todd 10 "$transfer_id"

cargo run --package bridge-eth --bin bridge-eth -- withdraw pete todd 10 "$transfer_id"

cargo run --package bridge-eth --bin bridge-eth -- close-transfer-account "$transfer_id"
