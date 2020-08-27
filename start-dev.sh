#!/usr/bin/env bash

docker-compose up tpdp_postgres &
(cd site && npm start) &
(cd core && sleep 2 && cargo watch -w src -x run)