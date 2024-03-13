default:
    just --list

dev:
    bacon -- --example basic

run:
    cargo r --example basic

run-release:
    cargo r --release --example basic

alias d := dev
alias r := run
alias rr := run-release
