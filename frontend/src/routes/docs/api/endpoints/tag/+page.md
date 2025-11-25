# Tag

## POST /v1/tag

Creates a new tag

See [Design/Color](/docs/design/color) for available colors

### Request

```json
{
  "name": "string",
  "color": "string",
  "category_id": "string"
}
```

### Response

**201 Created**

```json
{
  "id": "string",
  "user_id": "string",
  "created_at": 12345,
  "name": "string",
  "color": "string",
  "category_id": "string"
}
```

**400 Bad Request**

## PATCH /v1/tag/:id

Updates an existing tag

### Request

```json
{
  "name": "string",
  "color": "string"
}
```

### Response

**200 OK**

```json
{
  "id": "string",
  "user_id": "string",
  "created_at": 12345,
  "name": "string",
  "color": "string",
  "category_id": "string"
}
```

**400 Bad Request**

**404 Not Found**

## DELETE /v1/tag/:id

Deletes a tag

### Response

**204 No Content**

**400 Bad Request**

**404 Not Found**
