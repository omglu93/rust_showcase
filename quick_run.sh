#! /bin/bash

docker-compose up --force-recreate -d

diesel setup

cargo run