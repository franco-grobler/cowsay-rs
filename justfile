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

# Create a new release
[confirm]
[group("Release")]
create-release:
    ./scripts/create-release.sh

# Update github action job
[group("CI")]
update-ci-actions:
    ./scripts/update-github-actions-checkout-version.sh
