run:
	uvicorn app.main:app --reload --env-file=.env

build:
	docker build -t interview-app .

dr:
	docker run --gpus all --env-file .env -p 80:80 interview-app

