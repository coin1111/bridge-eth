set -x
transfer_id=aab47fa3a3dc42bc8cbc48c02182669b
# deposit pete to transfer to todd
cargo run --package bridge-eth --bin bridge-eth -- deposit pete  todd 10 "$transfer_id"

# observe deposit in the contract
cargo run --package bridge-eth --bin bridge-eth -- get-locked-info "$transfer_id"

# withdraw to todd
cargo run --package bridge-eth --bin bridge-eth -- withdraw pete todd 10 "$transfer_id"

# observe deposit in the contract as completed
# locked is_completed=false
cargo run --package bridge-eth --bin bridge-eth -- get-locked-info "$transfer_id"
# unlocked
cargo run --package bridge-eth --bin bridge-eth -- get-unlocked-info "$transfer_id"

# close transfer account
cargo run --package bridge-eth --bin bridge-eth -- close-transfer-account "$transfer_id"

# observe deposit in the contract as completed
# locked is_completed=true
cargo run --package bridge-eth --bin bridge-eth -- get-locked-info "$transfer_id"

# check balance 
# must be 10
cargo run --package bridge-eth --bin bridge-eth -- balance todd
