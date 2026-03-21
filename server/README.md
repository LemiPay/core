# Lemipay Server

## Docs
- [Diesel ORM](https://diesel.rs/)

### ORM Guide
primero crear una migration
```bash
diesel migration generate <<ejemplo: user>>
```
editar el up.sql y el down.sql y luego

```bash
diesel migration run
```

Si esta el schema pero no el SQL, hacer:
```bash
diesel migration generate <<ejemplo: user>> --diff-schema
```

Testear una funcion
```bash
cargo run --bin show_users
```