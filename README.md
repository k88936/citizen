
api gen
```shell
java -Xmx32G  -jar ~\.m2\repository\org\openapitools\openapi-generator-cli\7.21.0-SNAPSHOT\openapi-generator-cli-7.21.0-SNAPSHOT.jar  generate -g rust -i https://teamcity.jetbrains.com/app/rest/swagger.json -o ./api  --skip-validate-spec --additional-properties=packageName=api

```