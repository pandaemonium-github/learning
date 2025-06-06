
# postresql

## database size

```sql
select pg_size_pretty(pg_database_size('<database name>'));
```

## list databases

```postresql
\dt
```

## quick number of rows of a table

```postresql
select reltuples from pg_class where relname = '<table name>';
```

## sqlx

create migrations:
```
sqlx migrate add -r init
```

migrate and revert:
```
sqlx migrate run
sqlx migrate revert
```

migration file example:

```sql
DROP TYPE IF EXISTS genders;

CREATE TYPE genders AS ENUM ('M', 'F');

DROP TABLE IF EXISTS "member";

CREATE TABLE "member" (
  id integer NOT NULL PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  password varchar(100) NOT NULL,
  birth_date date NOT NULL,
  first_name varchar(32) NOT NULL,
  last_name varchar(32) NOT NULL,
  gender genders NOT NULL,
  joined_date date NOT NULL,
  created_date timestamp with time zone NOT NULL
);

CREATE INDEX member_email_idx ON member (email);
```
