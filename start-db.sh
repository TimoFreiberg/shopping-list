#!/bin/sh

docker run \
  --name shopping-list-db \
  --env POSTGRES_PASSWORD=password \
  --env POSTGRES_USER=shopping_list \
  --env POSTGRES_DB=shopping_list \
  --detach --rm \
  --publish 5432:5432 \
  postgres
