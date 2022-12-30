IMAGE_NAME ?= "grpc-service"
IMAGE_TAG ?= "latest" 

help:
	@printf "Usage:\n"
	@printf "  make [target]\n\n"
	@printf "Targets:\n"
	@printf "  help             Show this help\n"
	@printf "  codegen          Generate the language specific code\n"
	@printf "  build            Build the service\n"
	@printf "  run              Run the service\n"
	@printf "  test             Test the service\n"
	@printf "  list             List the service functions\n"
	@printf "  clean-codegen    Clean the generated code\n"
	@printf "  clean            Clean the service artifacts\n\n"

default: help

docker:
	@which docker > /dev/null || (printf "docker is not installed.\n" && exit 1)

codegen: docker clean-codegen
	@printf "=> Generating the code...\n"
	@mkdir codegen
	@docker build -t protoc:latest --target protoc .
	@docker run --rm -it -v $(PWD):/app -w /app protoc:latest \
		-I protos payments.proto \
		--prost_out=codegen \
		--tonic_out=codegen \
		--tonic_opt=no_client

build: docker
	@printf "=> Building the service...\n"
	@docker build -t $(IMAGE_NAME):$(IMAGE_TAG) .

run: docker
	@printf "=> Running the service...\n"
	@docker run --user 1000:1000 --rm -it -p 50051:50051 --name grpc-service $(IMAGE_NAME):$(IMAGE_TAG) 

test: docker
	@printf "=> Running the tests...\n"
	@docker run --rm -it --network host -v $(PWD):/app -w /app fullstorydev/grpcurl:latest \
		-import-path protos \
		-proto payments.proto \
		-plaintext \
		-d '{"from_address": "mwSmXgUGd78mUup5332qCyDUy32LoGCAfJ", "to_address": "myPrkbuYHZNGB8CPnZbDNzp945SLcjPK6z", "amount": 22}' \
		localhost:50051 \
		payments.Bitcoin/SendPayment

list: docker
	@printf "=> Available services:\n"
	@docker run --rm -it --network host -v $(PWD):/app -w /app fullstorydev/grpcurl:latest \
		-import-path protos \
		-proto payments.proto \
		-plaintext \
		localhost:50051 \
		list

clean-codegen:
	@printf "=> Cleaning the generated code...\n"
	@rm -rf codegen

clean: docker clean-codegen
	@printf "=> Cleaning the service...\n"
	@docker rm -f grpc-service || true
	@docker rmi $(IMAGE_NAME)-base:$(IMAGE_TAG) || true
	@docker rmi $(IMAGE_NAME):$(IMAGE_TAG) || true
	@rm -rf target
	@rm -rf examples/rust/target

.PHONY: help default docker codegen build run test list clean-codegen clean
