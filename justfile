default:
    just --list

container-tag := "cowsay-rs"

# Build the container file
[group("Docker")]
build-container:
    podman build --tag {{ container-tag }} --file docker/cowsay/Containerfile .

# Run container with an interactive terminal
[group("Docker")]
run-container:
    podman run -it --rm --entrypoint /bin/bash {{ container-tag }}
