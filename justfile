
project := "otp-session-service"

alias t := test
alias pre := test-all
alias b := build
alias fmt := format

os := `uname`


# run the standard tests
test:
    clear
    cargo test


# run the standard tests + clippy and fmt
test-all:
    clear
    cargo test -- --include-ignored && just format

# clean the project
clean:
    cargo clean

# build the debug target
build:
    clear
    cargo build

# run fmt and clippy
format:
    cargo fmt && cargo clippy

# build the docs
docs:
    cargo doc --no-deps --open

# watch the current folders and run tests when a file is changed
watch:
    watchexec -d 500 -c -e rs cargo test && cargo fmt && cargo clippy

# cover - runs code test coverage report and writes to coverage folder
cover:
    cargo tarpaulin --out html --output-dir coverage && mv coverage/tarpaulin-report.html coverage/index.html

# start a http server in the coverage folder
serve-cover:
    cd coverage && python3 -m http.server 8080

# merge the develop branch to main
merge:
    git push && git checkout main && git pull && git merge develop && git push && git checkout develop

