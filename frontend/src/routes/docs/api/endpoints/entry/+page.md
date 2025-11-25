# Entry

## POST /v1/entry

Creates a new entry

### Request

```json
{
  "mood": 5, // 1-5
  "date": "YYYY-MM-DD",
  "entry": "string", // optional
  "selected_tags": ["string"]
}
```

### Response

**201 Created**

```json
{
  "id": "string",
  "user_id": "string",
  "date": "YYYY-MM-DD",
  "created_at": 12345,
  "mood": 5,
  "entry": "string",
  "selected_tags": ["string"]
}
```

**400 Bad Request**

## PATCH /v1/entry/:id

Updates an existing entry

### Request

```json
{
  "mood": 5, // 1-5
  "date": "YYYY-MM-DD",
  "entry": "string", // optional
  "selected_tags": ["string"]
}
```

### Response

**200 OK**

```json
{
  "id": "string",
  "user_id": "string",
  "date": "YYYY-MM-DD",
  "created_at": 12345,
  "mood": 5,
  "entry": "string",
  "selected_tags": ["string"]
}
```

**400 Bad Request**

**404 Not Found**

## DELETE /v1/entry/:id

Deletes an entry

### Response

**204 No Content**

**400 Bad Request**

**404 Not Found**
