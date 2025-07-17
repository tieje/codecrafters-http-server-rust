# Installs codecrafters cli.
install-cli:
    curl https://codecrafters.io/install.sh | sh

# Runs tests for current problem.
test:
    codecrafters test

# Runs all previous tests.
prev:
    codecrafters test --previous

# Submits code to codecrafters. Shortcut for git push.
submit:
    codecrafters submit
