# API Endpoints

All API endpoints are at `/api`, `/` is for the built frontend in the `./www/` directory

## Endpoint Documentation

- [Auth](/docs/api/endpoints/auth) - Authentication and sessions
- [Category](/docs/api/endpoints/category) - Category operations
- [Entry](/docs/api/endpoints/entry) - Journal entry operations
- [Tag](/docs/api/endpoints/tag) - Tag operations
- [Metrics](/docs/api/endpoints/metrics) - User statistics and metrics
- [Sessions](/docs/api/endpoints/sessions) - Session management
- [User](/docs/api/endpoints/user) - User management and profile

### Error

All API errors follow the same format, see [API error response](/docs/api/endpoints/error) for details

### GET /

Returns package information and version

#### Response

**200 OK**

```json
{
  "name": "diary-dot-computer-backend",
  "version": "0.0.1"
}
```
