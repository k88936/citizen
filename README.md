# Citizen - TeamCity CLI

A command-line interface for TeamCity CI/CD server.

### Development

The API client is generated from TeamCity's OpenAPI spec:

```bash
java -Xmx32G -jar ~/.m2/repository/org/openapitools/openapi-generator-cli/7.21.0-SNAPSHOT/openapi-generator-cli-7.21.0-SNAPSHOT.jar generate -g rust -i https://teamcity.jetbrains.com/app/rest/swagger.json -o ./api --skip-validate-spec --additional-properties=packageName=api
```
