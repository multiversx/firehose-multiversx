FROM connector:latest as connector

FROM golang:1.21 AS builder

WORKDIR /go/firehose-multiversx
COPY . .
RUN go mod tidy
WORKDIR /go/firehose-multiversx/cmd/firemultiversx
RUN go build -v -ldflags="-X main.appVersion=$(git describe --tags --long --dirty)"

FROM ubuntu:22.04

COPY --from=connector /app/connector /app/connector
COPY --from=builder /go/firehose-multiversx/cmd/firemultiversx/firemultiversx /app/firemultiversx

ENTRYPOINT ["/app/firemultiversx"]
