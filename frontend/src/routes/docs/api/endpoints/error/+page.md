# API error response

All API errors follow the same format, this can be used in combination with the HTTP status code to determine or display errors

## Error format example

Example when creating a user with an email that is already in use:

```json
{
  "code": "EmailAlreadyInUse",
  "message": "Email already in use"
}
```
