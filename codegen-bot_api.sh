# cargo run -p api --bin api
docker run --rm --network host -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i http://host.docker.internal:3001/spec \
    -g rust \
    -o /local/packages/bot_sdk