# User Categories

## GET /v1/user/categories

Gets current user's categories with associated tags

### Response

**200 OK**

```json
[
  {
    "id": "string",
    "name": "string",
    "user_id": "string",
    "created_at": 12345,
    "tags": [
      {
        "id": "string",
        "user_id": "string",
        "created_at": 12345,
        "name": "string",
        "color": "string",
        "category_id": "string"
      }
    ]
  }
]
```

**401 Unauthorized**
