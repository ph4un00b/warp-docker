docker network create jamon

docker run --rm --detach -p 8080:8080 --name sqld -e RUST_LOG=sqld=debug,info --network jamon ghcr.io/tursodatabase/libsql-server:main

docker run --rm --detach -p 4560:4560 --network jamon rustiko:chef

@REM docker run --rm -p 4560:4560 --network jamon rustiko:test
