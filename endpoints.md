Here's a list of all the HTTP endpoints exposed by your backend for client communication:

## API Groups Endpoints
- `GET /api/groups/{email}` - Get all API groups for a user with preferences applied
- `POST /api/upload` - Upload API groups configuration file (YAML/JSON)
- `POST /api/group` - Add a new API group
- `PUT /api/group` - Update an existing API group
- `DELETE /api/groups/{email}/{group_id}` - Delete an API group

## User Preferences Endpoints
- `GET /api/user/preferences/{email}` - Get user preferences
- `POST /api/user/preferences` - Update user preferences
- `DELETE /api/user/preferences/{email}` - Reset user preferences

## API Key Management Endpoints
- `GET /api/user/keys/{email}` - Get status of all API keys for a user
- `POST /api/user/keys` - Generate a new API key for a user
- `DELETE /api/user/keys/{email}/{key_id}` - Revoke a specific API key
- `DELETE /api/user/keys/{email}` - Revoke all API keys for a user
- `GET /api/key/usage/{email}/{key_id}` - Get usage statistics for a specific API key
- `POST /api/key/validate` - Validate an API key

## Credit Balance Endpoints
- `GET /api/user/credits/{email}` - Get credit balance for a user
- `POST /api/user/credits` - Update credit balance for a user

## System Endpoints
- `GET /api/health` - Health check endpoint

Each of these endpoints accepts and returns JSON data and follows RESTful conventions. The multi-key system now allows users to manage multiple API keys independently while maintaining a single credit balance attached to their email.
