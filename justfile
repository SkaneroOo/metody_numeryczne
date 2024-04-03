set windows-shell := ["nu", "-c"]

default:
    @cargo build

test *targets:
    @cargo test {{targets}} --lib -- --nocapture

qtest *targets:
    @cargo test {{targets}} --lib

push message=".":
    @git add .
    @git commit -m {{message}}
    @git push origin master

pull:
    @git pull origin master