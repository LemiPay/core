# ---------- BUILD ----------
FROM node:24-alpine AS builder

RUN corepack enable

WORKDIR /app

COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY . .
RUN pnpm build

# ---------- RUNTIME ----------
FROM node:24-alpine

RUN corepack enable

WORKDIR /app

# Solo lo necesario
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./
COPY --from=builder /app/pnpm-lock.yaml ./

# Instalar solo deps de producción
RUN pnpm install --prod --frozen-lockfile

EXPOSE 5173

CMD ["pnpm", "preview", "--host"]