# Build frontend
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
# then copy all (non-ignored) project files into the image
FROM frontend-builder AS prerelease
COPY --from=install /temp/dev/node_modules node_modules
COPY frontend/ .

# build using npm because running vite commands via bun hangs indefinitely
FROM node:20 AS release
WORKDIR /usr/diary.computer/frontend
COPY --from=prerelease /usr/diary.computer/frontend .
COPY --from=install /temp/prod/node_modules node_modules
ENV NODE_ENV=production
RUN npm run build

# Build backend
# does not work with slim images due to missing dependencies (cc)
FROM rust:1.88.0 AS backend-builder
WORKDIR /usr/diary.computer/backend
# copy source files
COPY backend/ .
# build release binary
RUN cargo build --release

# TODO: postgres and .env

# Final image
FROM ubuntu:22.04 AS final
WORKDIR /usr/diary.computer
ARG DEBIAN_FRONTEND=noninteractive
# install postgresql client
RUN apt-get update && apt-get install postgresql -y
# copy backend binary
COPY --from=backend-builder /usr/diary.computer/backend/target/release/diary-dot-computer-backend ./server/diary-dot-computer
# copy frontend build files from ./www
COPY --from=release /usr/diary.computer/www www
# expose port 3000
EXPOSE 3000
# run the backend server
WORKDIR /usr/diary.computer/server
ENTRYPOINT [ "./diary-dot-computer" ]
