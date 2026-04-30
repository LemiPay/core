# Arquitectura y Diseño

> The big refactor
> 

---

[El Problema](https://www.notion.so/El-Problema-34caf716b4d68093aff1e3b0fd89fd5c?pvs=21)

# 🏗️ LemiPay Backend Architecture

## 🎯 Objetivo

Definir una arquitectura clara, mantenible y escalable para el backend en Rust, basada en:

- Clean Architecture (adaptada)
- Domain-Driven Design (liviano)
- Principios SOLID
- Inmutabilidad y funciones puras

---

# 🧱 1. Estructura del Proyecto

```bash
src/
│
├── app/                # Wiring (estado global, config, DI)
│   ├── state.rs
│   ├── router.rs
│
├── domain/             # 🔥 Núcleo puro del negocio
│   ├── group/
│   ├── proposal/
│   ├── treasury/
│   ├── expense/
│   ├── user/
│
├── application/        # 🔥 Casos de uso
│   ├── group/
│   ├── proposal/
│   ├── treasury/
│   ├── expense/
│   ├── user/
│
├── infrastructure/     # 🔌 Implementaciones externas
│   ├── db/
│   │   ├── repositories/
│   │   ├── models/
│   │   ├── schema.rs
│   │
│   ├── blockchain/
│
├── interfaces/         # 🌐 HTTP (Axum)
│   ├── handlers/
│   ├── dto/
│   ├── routes/
│
└── main.rs
```

---

# 🧠 2. Domain Layer (CORE)

## Reglas

- ❌ Sin Diesel
- ❌ Sin async
- ❌ Sin Axum
- ❌ Sin DB models
- ✅ Inmutable
- ✅ Funciones puras
- ✅ Value Objects
- ✅ Policies

---

## 📦 Organización

```bash
domain/
  proposal/
    withdraw/
      entity.rs
      policy.rs
      error.rs
```

---

## 🧾 Entidades

```rust
pub struct WithdrawProposal {
    pub id: ProposalId,
    pub group_id: GroupId,
    pub amount: Money,
    pub created_by: UserId,
}
```

---

## 🔒 Invariantes

```rust
impl Money {
    pub fn new(amount: Decimal, currency: CurrencyId) -> Result<Self, DomainError> {
        if amount <= Decimal::ZERO {
            return Err(DomainError::InvalidAmount);
        }
        Ok(Self { amount, currency });
    }
}
```

---

## ⚖️ Policies (reglas de negocio)

```rust
pub struct WithdrawPolicy;

impl WithdrawPolicy {
    pub fn validate(
        group: &Group,
        user_id: UserId,
        amount: Money,
        balance: Money,
    ) -> Result<(), WithdrawError> {
        if !group.is_member(user_id) {
            return Err(WithdrawError::NotMember);
        }

        if balance.amount < amount.amount {
            return Err(WithdrawError::InsufficientBalance);
        }

        Ok(())
    }
}
```

---

# 🔁 3. Application Layer (Use Cases)

## 📁 Estructura

```bash
application/
  proposal/
    create_withdraw/
      use_case.rs
      dto.rs
      error.rs
```

---

## 📥 DTOs

```rust
pub struct CreateWithdrawInput {
    pub group_id: GroupId,
    pub creator_id: UserId,
    pub amount: Money,
}

pub struct CreateWithdrawOutput {
    pub proposal_id: ProposalId,
}
```

---

## ⚠️ Errores por Use Case

- Un enum por caso de uso
- No exponer errores de infraestructura

```rust
pub enum CreateWithdrawError {
    GroupNotFound,
    NotMember,
    InsufficientBalance,
    InternalError,
}
```

---

## 🧠 Use Case

- Orquesta lógica
- Usa múltiples repos si es necesario
- Sync (por ahora)

---

# 🔌 4. Repositories

## 📌 Definición (traits)

```rust
pub trait GroupRepository {
    fn find_by_id(&self, id: GroupId) -> Result<Option<Group>, RepoError>;
}
```

---

## 📌 Implementación

```bash
infrastructure/db/repositories/
  diesel_group_repository.rs
```

---

## 📌 Reglas

- Por agregado (no por tabla)
- Use case puede usar múltiples repos
- No lógica de negocio en repos

---

# 🗄️ 5. Infraestructura

## Separación estricta

- Domain models ≠ DB models

```rust
impl From<DbGroup> for Group
impl From<Group> for DbGroup
```

---

# 🌐 6. Interfaces (HTTP)

## 📌 Handlers

- Sin lógica de negocio
- Solo mapping

```rust
async fn handler(...) {
    → map request → input
    → ejecutar use case
    → map error → response
}
```

---

## 📌 DTOs

Separados:

- Request / Response (HTTP)
- Input / Output (Use case)

---

# 💰 7. Value Objects

## Ejemplos

```rust
struct UserId(Uuid);
struct GroupId(Uuid);
struct ProposalId(Uuid);

struct Money {
    amount: Decimal,
    currency: CurrencyId,
}
```

---

## Reglas

- IDs siempre tipados
- Money obligatorio
- No mezclar monedas

---

# ⚙️ 8. Configuración (Policies dinámicas)

Ubicada en:

```bash
app/state.rs
```

```rust
pub struct AppConfig {
    pub governance: GovernanceConfig,
}
```

Se inyecta en use cases o policies.

---

# 🧪 9. Testing Strategy

## Tipos

- Domain → puro (sin mocks)
- Use cases → mocks manuales
- Integration → opcional

---

## Objetivo

- Testear lógica sin DB
- Test rápido y aislado

---

# 🔴 10. Manejo de Errores

## Capas

| Capa | Tipo |
| --- | --- |
| Domain | DomainError |
| Application | UseCaseError |
| Infra | RepoError |

---

## Reglas

- Domain → errores propios
- Application → mapea errores
- Infra → nunca se expone

---

# 🔄 11. Mutabilidad

- Preferencia: inmutabilidad
- Métodos devuelven nuevo estado

---

# 🧠 12. Convenciones

## Lenguaje

- Código: Inglés
- Frontend: Español
- Mensajes de error: Español

---

## Naming

- Rust: idiomático
- DB: snake_case
- JSON: camelCase
- Traits: PascalCase

---

# 🚀 13. Decisiones a Futuro

## No implementado aún

- Unit of Work
- CQRS
- Domain Events
- Async use cases

---

## Diseñado para soportar

- Multi-currency
- DeFi integrations
- Gobernanza configurable

---

# ⚠️ 14. Anti-patterns

❌ Lógica en handlers

❌ Lógica en repositories

❌ Usar modelos de DB en domain

❌ Validaciones duplicadas

❌ Domain async

❌ Pasar conexiones por todos lados

---

# 🧭 15. Filosofía

- El **domain es la verdad**
- El resto es implementación
- Código debe leerse como reglas de negocio
- Todo lo importante debe ser testeable sin DB

---

# ✅ Resultado

Esta arquitectura permite:

- Escalar sin romper todo
- Testear fácil
- Mantener claridad
- Integrar blockchain y DB sin caos
- Evolucionar hacia sistemas más complejos

---