IMAGE_NAME ?= "grpc-go-client"
IMAGE_TAG ?= "latest" 

help:
	@printf "Usage:\n"
	@printf "  make [target]\n\n"
	@printf "Targets:\n"
	@printf "  help       Show this help\n"
	@printf "  codegen    Generate the language specific code\n"
	@printf "  build      Build the client\n"
	@printf "  run        Run the client\n"
	@printf "  clean      Clean the client artifacts\n\n"

default: help

docker:
	@which docker > /dev/null || (printf "Docker is not installed.\n" && exit 1)

codegen: docker clean-codegen
	@mkdir codegen
	@printf "=> Generating the code...\n"
	@docker build -t protoc:latest --target protoc .
	@docker run --rm -it -v $(PWD):/app -w /app protoc:latest \
		-I protos \
		--go_out=/app/codegen \
		--go-grpc_out=/app/codegen \
		--go_opt=paths=source_relative \
		--go-grpc_opt=paths=source_relative \
		payments.proto

build: docker
	@printf "=> Building the client...\n"
	@docker build -t $(IMAGE_NAME):$(IMAGE_TAG) .

run: docker
	@printf "=> Running the client...\n"
	@docker run --user 1000:1000 --net host --rm -it --name grpc-rust-client $(IMAGE_NAME):$(IMAGE_TAG) 

clean-codegen:
	@printf "=> Cleaning the generated code...\n"
	@rm -rf codegen

clean: docker clean-codegen
	@printf "=> Cleaning the client...\n"
	@docker rm -f grpc-rust-client || true
	@docker rmi $(IMAGE_NAME)-base:$(IMAGE_TAG) || true
	@docker rmi $(IMAGE_NAME):$(IMAGE_TAG) || true
	@rm -rf target

.PHONY: help default docker codegen build run clean-codegen clean
