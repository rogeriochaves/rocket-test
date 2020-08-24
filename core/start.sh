set -e

diesel migration run

ROCKET_PORT=$PORT_CORE tpdp-core
