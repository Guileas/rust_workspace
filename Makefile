CONTAINER_NAME="rust_workspace"

build:
	docker-compose build

start:
	docker-compose up -d

init:
	docker exec -it ${CONTAINER_NAME} sh
