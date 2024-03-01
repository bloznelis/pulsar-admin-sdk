# Fetches new pulsar admin spec
fetch-spec:
  wget https://pulsar.apache.org/swagger/master//swagger.json

# Generates rust code out of the openapi spec
generate:
   docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli generate -i /local/openapi.json -g rust -o /local/generated --skip-validate-spec --package-name="pulsar-admin-sdk"
   sudo chown -R lukas:lukas ./generated
