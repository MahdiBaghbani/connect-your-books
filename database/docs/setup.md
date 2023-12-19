You have to install SeaORM CLI for migration tasks.
```shell
cargo install sea-orm-cli
```

```shell
sea-orm-cli migrate generate users 
```

```shell
sea-orm-cli migrate -u postgres://cyb:cyb@127.0.0.1:5432/cyb
```

```shell
cd entity/src
sea-orm-cli generate entity -u postgres://cyb:cyb@127.0.0.1:5432/cyb
```
