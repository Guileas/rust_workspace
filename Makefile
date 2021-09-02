CONTAINER_NAME="api_rocket_workspace"

build:
	docker-compose build

start:
	docker-compose down && docker-compose up -d

init:
	docker exec -it ${CONTAINER_NAME} sh
