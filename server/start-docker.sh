#!/usr/bin/env $SHELL
docker run --rm  --name postgres -p 5432:5432 -e POSTGRES_PASSWORD=postgres  postgres:16
