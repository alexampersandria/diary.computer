# Category

## POST /v1/category

Creates a new category

### Request

```json
{
  "name": "string"
}
```

### Response

**201 Created**

```json
{
  "id": "string",
  "name": "string",
  "user_id": "string",
  "created_at": 12345
}
```

**400 Bad Request**

## PATCH /v1/category/:id

Updates an existing category

### Request

```json
{
  "name": "string"
}
```

### Response

**200 OK**

```json
{
  "id": "string",
  "name": "string",
  "user_id": "string",
  "created_at": 12345
}
```

**400 Bad Request**

**404 Not Found**

## DELETE /v1/category/:id

Deletes a category

### Response

**204 No Content**

**400 Bad Request**

**404 Not Found**
