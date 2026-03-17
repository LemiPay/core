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