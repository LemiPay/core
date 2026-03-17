# 🚀 LemiPay

**LemiPay** es una plataforma de tesorería compartida descentralizada que permite a grupos de usuarios gestionar fondos de forma colaborativa mediante smart contracts en Ethereum.

Este proyecto se desarrolla en el contexto de la materia **Laboratorio 1 (Ingeniería Informática)**, con un enfoque en arquitectura completa, buenas prácticas y diseño de sistemas distribuidos.

---

## 🧠 ¿Qué problema resuelve?

LemiPay permite que múltiples wallets:

* 💰 Depositen fondos en una tesorería común
* 🗳️ Propongan y voten retiros
* 📊 Participen en decisiones con reglas de gobernanza
* 🔁 Organicen rondas de aportes
* 📈 (Futuro) Generen rendimiento vía DeFi

👉 Todo sin depender de una entidad central.

---

## 🏗️ Arquitectura

```
Frontend (SvelteKit)
        |
        v
Backend API (Rust + Axum)
        |
        +---- PostgreSQL
        |
        v
Ethereum Smart Contracts (Solidity)
```

---

## ⚙️ Stack Tecnológico

### 🔗 Blockchain

* Solidity
* Foundry (testing)

### 🦀 Backend

* Rust
* Axum (HTTP server)
* Diesel (ORM, sync)
* r2d2 (connection pool)
* Serde (JSON)
* ethers-rs (interacción con Ethereum)

### 🗄️ Base de Datos

* PostgreSQL (Docker)

### 🎨 Frontend

* SvelteKit

### 🐳 Infraestructura

* Docker
* Docker Compose

---

## 📦 Estructura del Proyecto

```
lemipay/
├── backend/        # API en Rust (Axum + Diesel)
├── frontend/       # App web (SvelteKit)
├── contracts/      # Smart contracts (Solidity + Foundry)
├── docker-compose.yml
├── .env
└── README.md
```

---

## 🚀 Cómo correr el proyecto

### 1. Clonar el repositorio

```bash
git clone https://github.com/tu-org/lemipay.git
cd lemipay
```

---

### 2. Configurar variables de entorno

Crear archivo `.env` en la raíz:

Ejemplo en `.env.example`:

---

### 3. Levantar todo con Docker

```bash
docker compose up --build
```

---

### 4. Acceder a los servicios

* Frontend → http://localhost:5173
* Backend → http://localhost:3000
* DB → localhost:5432

---

## 🧪 Migrations (Diesel)

Dentro del container del backend o localmente:

```bash
diesel migration run
```

---

## 🔐 Diseño On-chain vs Off-chain

### On-chain (Ethereum)

* Miembros del grupo
* Fondos
* Propuestas
* Votos
* Reglas de gobernanza

### Off-chain (PostgreSQL)

* Nombres de grupos
* Metadata
* Historial indexado
* Datos de UI

👉 Esto optimiza costos de gas y performance.

---

## 🗳️ Gobernanza

Cada grupo puede definir sus propias reglas:

* Unanimidad
* Mayoría simple
* 2/3
* Porcentaje custom

---

## 🧑‍💻 Objetivo académico

Este proyecto busca aplicar:

* Arquitectura de sistemas distribuidos
* Integración blockchain + backend tradicional
* Manejo de estado on-chain/off-chain
* Buenas prácticas de ingeniería

---

## 🤝 Contribuciones

Proyecto académico en desarrollo.
Se aceptan ideas, mejoras y feedback 🚀