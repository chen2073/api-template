FROM golang:1.24-alpine3.21 AS builder

WORKDIR /api

COPY api/ ./

RUN go mod download

RUN go build -o main .

FROM alpine:3.21

WORKDIR /api

COPY --from=builder /api/main ./main

CMD ["/api/main"]