# Entries

## GET /v1/entries

Gets entries with filtering and pagination

### Query Parameters

| param     | type     | desc                                                                               | default     |
| --------- | -------- | ---------------------------------------------------------------------------------- | ----------- |
| from_date | `string` | (YYYY-MM-DD) Start date for filtering entries                                      |             |
| to_date   | `string` | (YYYY-MM-DD) End date for filtering entries                                        |             |
| tags      | `string` | Comma-separated tag IDs, filters contain, returns entries matching **all** tag ids |             |
| from_mood | `number` | (1-5) Minimum mood value for filtering                                             |             |
| to_mood   | `number` | (1-5) Maximum mood value for filtering                                             |             |
| order     | `string` | Sort order: `date_asc`, `date_desc`, `mood_asc`, `mood_desc`                       | `date_desc` |
| limit     | `number` | Number of entries to return                                                        | `31`        |
| offset    | `number` | Pagination offset                                                                  | `0`         |

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
