
publish:
	rm -rf ./target
	docker build . -t vipo/bimaru:latest
	docker push vipo/bimaru:latest

rollout:
	kubectl rollout restart deployment bimaru