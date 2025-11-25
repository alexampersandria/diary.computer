# Auth

## POST /v1/auth

Creates a new session (log in)

### Request

```json
{
  "email": "string",
  "password": "string"
}
```

### Response

**201 Created**

```json
{
  "id": "string",
  "user_id": "string",
  "created_at": 12345,
  "accessed_at": 12345,
  "ip_address": "string",
  "user_agent": "string"
}
```

**401 Unauthorized** - Invalid password

**400 Bad Request**

## GET /v1/auth/config

Gets authentication configuration

### Response

**200 OK**

```json
{
  "invite_required": true
}
```
