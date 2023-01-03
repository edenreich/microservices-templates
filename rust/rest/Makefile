IMAGE_NAME ?= "rest-service"
IMAGE_TAG ?= "latest" 

help:
	@printf "Usage:\n"
	@printf "  make [target]\n\n"
	@printf "Targets:\n"
	@printf "  help             Show this help\n"
	@printf "  build            Build the service\n"
	@printf "  run              Run the service\n"
	@printf "  test             Test the service\n"
	@printf "  clean            Clean the service artifacts\n\n"

default: help

docker:
	@which docker > /dev/null || (printf "docker is not installed.\n" && exit 1)

build: docker
	@printf "=> Building the service...\n"
	@docker build -t $(IMAGE_NAME):$(IMAGE_TAG) .

run: docker
	@printf "=> Running the service...\n"
	@docker run --user 1000:1000 --rm -it -p 8080:8080 --name rest-service $(IMAGE_NAME):$(IMAGE_TAG) 

test: docker
	@printf "=> Running the tests...\n"
	@docker run --rm -it --net host loadimpact/loadgentest-hey:latest -n 10000 -c 100 -m GET http://localhost:8080/api/v1/payments

clean: docker
	@printf "=> Cleaning the service...\n"
	@docker rm -f rest-service || true
	@docker rmi $(IMAGE_NAME)-base:$(IMAGE_TAG) || true
	@docker rmi $(IMAGE_NAME):$(IMAGE_TAG) || true
	@rm -rf target

.PHONY: help default docker build run test clean