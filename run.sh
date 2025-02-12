#!/bin/bash

function run_client() {
  cargo run --bin client
}

function run_server() {
  cargo run --bin server
}

function connect() {
  telnet 0.0.0.0 6969
}

case "$1" in
  client)
    run_client
    ;;
  server)
    run_server
    ;;
  connect)
    connect
    ;;
  *)
    echo "Usage: $0 {client | server | connect}"
    exit 1
    ;;
esac
