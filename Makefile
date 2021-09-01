CONTAINER_NAME="rust_workspace"

CONTAINER_USER="$(shell id -u):$(shell id -g)"

build:
	docker-compose build

start:
	docker-compose down && docker-compose up -d

init:
	docker exec -it ${CONTAINER_NAME} sh
