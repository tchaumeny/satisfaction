EXAMPLES_DIR=./examples
BIN=$(realpath ./target/release/satisfaction)

cargo build --release &&

(
    cd $EXAMPLES_DIR

    $BIN -k 2 -n 10 -s 10000 --alpha-start 0 --alpha-end 5 --alpha-steps 50 --verbose
    $BIN -k 2 -n 50 -s 10000 --alpha-start 0 --alpha-end 4 --alpha-steps 40 --verbose
    $BIN -k 2 -n 1000 -s 1000 --alpha-start 0 --alpha-end 3 --alpha-steps 30 --verbose
    $BIN -k 3 -n 50 -s 1000 --alpha-start 0 --alpha-end 9 --alpha-steps 90 --verbose
)
