# alpine base image
FROM alpine:3.20 AS alpine-base

FROM alpine-base AS alpine-builder

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

FROM alpine-base AS alpine-runner

WORKDIR /api

COPY --from=alpine-builder /api/_build/default/bin/main.exe ./

CMD [ "/api/main.exe" ]