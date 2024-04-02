set windows-shell := ["nu", "-c"]

default:
    @cargo build

test *targets:
    @cargo test {{targets}} --lib -- --nocapture

qtest *targets:
    @cargo test {{targets}} --lib

push:
    @git add .
    @git commit -m .
    @git push origin master

pull:
    @git pull origin master