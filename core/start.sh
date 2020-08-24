set -e

diesel migration run

ROCKET_PORT=$PORT tpdp-core
