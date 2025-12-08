# build frontend
FROM oven/bun:1 AS frontend-builder
WORKDIR /usr/diary.computer/frontend

# install dependencies into temp directory
# this will cache them and speed up future builds
FROM frontend-builder AS install
RUN mkdir -p /temp/dev
COPY frontend/package.json frontend/bun.lock /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

# install with --production (exclude devDependencies)
RUN mkdir -p /temp/prod
COPY frontend/package.json frontend/bun.lock /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production

# copy node_modules from temp directory
FROM frontend-builder AS prerelease
COPY --from=install /temp/dev/node_modules node_modules
COPY frontend/ .

# build using npm because running vite commands via bun hangs indefinitely
FROM node:20 AS release
WORKDIR /usr/diary.computer/frontend
COPY --from=prerelease /usr/diary.computer/frontend .
COPY --from=install /temp/prod/node_modules node_modules
RUN npm run build

# build backend
FROM rust:1.90.0 AS backend-builder
WORKDIR /usr/diary.computer/backend
COPY backend/ .
RUN cargo build --release

# final image
FROM ubuntu:22.04 AS final
WORKDIR /usr/diary.computer
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install postgresql -y
COPY --from=backend-builder /usr/diary.computer/backend/target/release/diarycomputer ./server/diary.computer
COPY --from=release /usr/diary.computer/www www
ENV PORT=3137
EXPOSE 3137
WORKDIR /usr/diary.computer/server
ENTRYPOINT [ "./diary.computer" ]
