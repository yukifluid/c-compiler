assert() {
    expected="$1"
    input="$2"

    ./target/debug/c-compiler "$input" > tmp.s
    cc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
      echo "$input => $actual"
    else
      echo "$input => $expected expected, but got $actual"
      exit 1
    fi
}

cargo build
cargo test

echo "running tests"
assert 0  "0"
assert 42 "42"
assert 6  "1+2+3"
assert 2  "4+3-5"

echo "test result: ok"