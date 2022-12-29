FROM golang:1.20-rc-alpine3.17 AS protoc-go-plugin
RUN go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

FROM znly/protoc:0.4.0 AS protoc
WORKDIR /app
COPY --from=protoc-go-plugin /go/bin/protoc-gen-go-grpc /usr/bin/protoc-gen-go-grpc
COPY protos protos
RUN mkdir -p codegen
RUN protoc \
    -I protos \
    --go_out=/app/codegen \
    --go-grpc_out=/app/codegen \
    --go_opt=paths=source_relative \
    --go-grpc_opt=paths=source_relative \
    payments.proto
ENTRYPOINT [ "protoc" ]

FROM golang:1.20-rc-alpine3.17 AS builder
WORKDIR /app
COPY --from=protoc /app/codegen codegen
COPY src src
COPY go.* ./
RUN go mod tidy
RUN GOOS=linux CGO_ENABLED=0 go build -ldflags="-s -w" -o /app/bin/client src/client.go

FROM scratch
COPY --from=builder /app/bin/client /app/client
ENTRYPOINT [ "/app/client" ]
