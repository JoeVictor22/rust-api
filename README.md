## Diesel commands

```shell
# create db
diesel setup

# create custom migration
diesel migration generate MIGRATION_NAME

# upgrade
diesel migration run

# downgrade
diesel migration redo

# generate orm
diesel print-schema > src/schema.rs
# or use this code on schema.rs
infer_schema!("dotenv:DATABASE_URL");
```


