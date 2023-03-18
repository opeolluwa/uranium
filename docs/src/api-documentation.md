# API Documentation
| | HTTP Verb | endpoint | Description | Request Body | Response Body | Response Code | 
|--|-----------|----------|-------------|--------------|---------------|---------------|
| 1. | GET, PUT, POST, PATCH, DELETE   |  /  | the base URL | not applicable | HTML page | 200 |
| 2. | POST   |  /auth/sign-up  | the account registration | ```json  { "firstname" : "adeoye"}``` | HTML page | 201 |
| 3. | GET  |  /auth/confirm-email  | account verification endpoint | not applicable | HTML page | 200 |

