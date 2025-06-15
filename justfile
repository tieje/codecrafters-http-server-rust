# Installs codecrafters cli
install-cli:
    curl https://codecrafters.io/install.sh | sh

test:
    codecrafters test

prev:
    codecrafters test --previous

submit:
    codecrafters submit
