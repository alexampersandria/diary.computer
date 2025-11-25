# Entries

## GET /v1/entries

Gets entries with filtering and pagination

### Query Parameters

- `from_date`: YYYY-MM-DD (optional)
- `to_date`: YYYY-MM-DD (optional)
- `tags`: Comma-separated tag IDs (optional)
- `from_mood`: 1-5 (optional)
- `to_mood`: 1-5 (optional)
- `order`: date_asc, date_desc, mood_asc, mood_desc (optional, default: date_desc)
- `limit`: Number of entries to return (optional, default: 31)
- `offset`: Pagination offset (optional, default: 0)

### Response

**200 OK**

```json
{
  "data": [
    {
      "id": "string",
      "user_id": "string",
      "created_at": 12345,
      "mood": 5,
      "entry": "string",
      "date": "YYYY-MM-DD",
      "selected_tags": ["string"]
    }
  ],
  "pagination": {
    "limit": 31,
    "offset": 0,
    "total_count": 100
  }
}
```

**400 Bad Request**
