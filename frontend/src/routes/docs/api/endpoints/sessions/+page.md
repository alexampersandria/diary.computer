# Sessions

## GET /v1/sessions

Gets all sessions for current user

### Response

**200 OK**

```json
[
  {
    "id": "string",
    "user_id": "string",
    "created_at": 12345,
    "accessed_at": 12345,
    "ip_address": "string",
    "user_agent": "string"
  }
]
```
