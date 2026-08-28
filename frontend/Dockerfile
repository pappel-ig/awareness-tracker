FROM node:26-alpine3.24 as build
RUN npm install -g pnpm
WORKDIR /app
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN --mount=type=cache,id=pnpm,target=/root/.local/share/pnpm/store pnpm install --frozen-lockfile
COPY . .
RUN pnpm generate

FROM nginx:stable-alpine as runtime
COPY --from=build /app/.output/public /usr/share/nginx/html/
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]