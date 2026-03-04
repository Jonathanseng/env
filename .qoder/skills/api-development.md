# API Development Skill

This skill teaches best practices for designing and implementing RESTful APIs.

## Purpose

Create consistent, well-documented, and robust APIs that follow industry standards.

## RESTful Design Principles

### Resource Naming

✅ Good:
- `/users` - Collection of users
- `/users/123` - Specific user
- `/users/123/posts` - Posts by user

❌ Bad:
- `/getUsers` - Verb in path
- `/user/123/data` - Unnecessary nesting
- `/getAllUserData` - Multiple violations

### HTTP Methods

| Method | Purpose | Idempotent |
|--------|---------|------------|
| GET | Retrieve resource | Yes |
| POST | Create resource | No |
| PUT | Replace resource | Yes |
| PATCH | Update resource | No |
| DELETE | Remove resource | Yes |

## Response Format

### Success Responses

```json
{
  "data": { ... },
  "meta": {
    "timestamp": "2024-01-15T10:30:00Z",
    "version": "1.0"
  }
}
```

### Error Responses

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid email format",
    "details": [
      {
        "field": "email",
        "issue": "Must be a valid email address"
      }
    ]
  }
}
```

### Status Codes

- `200 OK` - Successful GET, PUT, PATCH
- `201 Created` - Successful resource creation
- `204 No Content` - Successful DELETE
- `400 Bad Request` - Invalid input
- `401 Unauthorized` - Authentication required
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - Resource doesn't exist
- `409 Conflict` - Resource conflict
- `422 Unprocessable Entity` - Validation error
- `500 Internal Server Error` - Server error

## Input Validation

Always validate:
1. Required fields are present
2. Data types match expectations
3. String lengths within limits
4. Email formats are valid
5. Numbers are in acceptable ranges
6. Enums match allowed values

## Security Checklist

- [ ] Authentication required for protected routes
- [ ] Authorization checked for resource access
- [ ] Input sanitized to prevent injection attacks
- [ ] Rate limiting implemented
- [ ] Sensitive data filtered from responses
- [ ] HTTPS enforced in production
- [ ] CORS properly configured
- [ ] API versioning strategy defined

## Documentation Requirements

For each endpoint, document:
1. Endpoint path and HTTP method
2. Purpose and behavior
3. Required/authentication status
4. Request parameters and body schema
5. Response schema for all status codes
6. Error scenarios and handling
7. Rate limits if applicable
8. Version information

## Implementation Template

```typescript
// Route handler structure
async function getUser(req, res) {
  try {
    // 1. Validate parameters
    const userId = validateUserId(req.params.id);
    
    // 2. Check authorization
    await authorizeAccess(req.user, userId);
    
    // 3. Fetch resource
    const user = await db.users.findById(userId);
    
    // 4. Return response
    res.json({
      data: sanitizeUser(user),
      meta: buildMeta()
    });
  } catch (error) {
    handleError(error, res);
  }
}
```

## Testing Guidelines

Test each endpoint for:
- Happy path scenarios
- Missing required fields
- Invalid data types
- Boundary conditions
- Authentication failures
- Authorization failures
- Rate limit enforcement
- Error response format