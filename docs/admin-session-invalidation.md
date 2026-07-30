# Administrator session invalidation

Session invalidation is part of each security-sensitive transaction:

| Change | Access sessions | Refresh tokens |
|---|---|---|
| Administrator changes own password | current access session remains; other access sessions are revoked | all existing refresh tokens are revoked |
| Administrator resets another password | all target-user sessions are revoked; replacement password requires self-change | all target-user refresh tokens are revoked |
| User is banned | all target-user sessions are revoked | all target-user refresh tokens are revoked |
| User is unbanned | no session is created | no token is restored |
| User roles change | all target-user sessions are revoked | all target-user refresh tokens are revoked |
| Permissions assigned to a role change | sessions for affected role members are revoked | refresh tokens for affected members are revoked |
| One session is revoked | selected access session and its refresh lineage are rejected | selected refresh lineage is revoked |
| All sessions are revoked or user is deleted | all target-user sessions are revoked | all target-user refresh tokens are revoked |

Authorization is loaded from PostgreSQL for authenticated requests and is not a browser-only
navigation decision. A revoked or newly forbidden session therefore cannot retain access by
calling a typed API route directly.
