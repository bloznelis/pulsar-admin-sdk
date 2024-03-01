# Fetches new pulsar admin spec
fetch-spec:
  wget https://pulsar.apache.org/swagger/master//swagger.json

# Generates rust code out of the openapi spec
generate:
   docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli generate -i /local/swagger.json -g rust -o /local/generated --skip-validate-spec
