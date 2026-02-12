#!/usr/bin/env bash
set -eu

until docker exec mysql-db-primary sh -c 'export MYSQL_PWD=password; mysql -h 127.0.0.1 -uroot -e ";"'
do
    echo "Waiting for mysql-db-primary database connection..."
    sleep 5
done

priv_stmt='CREATE USER IF NOT EXISTS "replica_user"@"%" IDENTIFIED BY "replica_pwd"; GRANT REPLICATION SLAVE ON *.* TO "replica_user"@"%"; FLUSH PRIVILEGES;'
docker exec mysql-db-primary sh -c "export MYSQL_PWD=password; mysql -h 127.0.0.1 -uroot -e '$priv_stmt'"

docker exec -it mysql-db-primary /bin/bash -c 'mysql -h 127.0.0.1 -uroot -p db < /src/schema.sql'
