main_build:
	docker build -f main.dockerfile -t main:latest .
	kind load docker-image main:latest --name sidecar-test

sidecar_build:
	docker build -f sidecar.dockerfile -t sidecar:latest .
	kind load docker-image sidecar:latest --name sidecar-test

