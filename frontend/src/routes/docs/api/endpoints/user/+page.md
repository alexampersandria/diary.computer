# User

## POST /v1/user

Creates a new user

### Request

```json
{
  "name": "string",
  "email": "string",
  "password": "string",
  "invite": "string" // optional
}
```

See [Auth Config](/docs/api/endpoints/auth/#get-v1authconfig) to determine if invite is required

### Response

**201 Created**

returns a session, see [Auth](/docs/api/endpoints/auth/#post-v1auth)

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

**400 Bad Request**

**409 Conflict** - Email already in use

## GET /v1/user

gets current user information

### Response

**200 OK**

```json
{
  "id": "string",
  "created_at": 12345,
  "name": "string",
  "email": "string",
  "invite": "string" // nullable
}
```

**401 Unauthorized**

## PATCH /v1/user

Updates current user. Both fields required

### Request

```json
{
  "name": "string",
  "email": "string"
}
```

### Response

**204 No Content**

**400 Bad Request**

**401 Unauthorized**

## DELETE /v1/user

Deletes current user

### Response

**204 No Content**

**400 Bad Request**

**401 Unauthorized**

## PATCH /v1/user/password

Updates current user's password

### Request

```json
{
  "password": "string"
}
```

### Response

**204 No Content**

**400 Bad Request**

**401 Unauthorized**
