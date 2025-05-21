# alpine base image
FROM alpine:3.21 AS base

FROM base AS builder

RUN apk update && \
    apk upgrade && \
    apk add build-base opam

RUN opam init --bare -a -y --disable-sandboxing \
    && opam update

RUN opam switch create default ocaml-base-compiler.5.3.0

RUN opam install -y dune

WORKDIR /api

COPY api/ ./

RUN opam exec -- dune build

FROM base AS runner

WORKDIR /api

COPY --from=alpine-builder /api/_build/default/bin/main.exe ./main.exe

CMD [ "/api/main.exe" ]