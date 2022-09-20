#! /bin/bash

docker-compose up --force-recreate -d

diesel setup

cargo build --release

cargo run --release