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

ENV NODE_ENV=production
ENV PORT=5173

# Copiamos build + deps ya instaladas
COPY --from=builder /app/build ./build
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./

EXPOSE 5173

CMD ["node", "build"]