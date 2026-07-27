# API error responses

Application API failures return a JSON problem document. Clients should use the HTTP status and
the `kind` field for control flow; `detail` is stable human-readable text and is not intended for
parsing.

Administrator and generated CRUD responses use:

```http
Content-Type: application/problem+json
```

The notification API currently serializes the same document as `application/json`.

## Response body

```json
{
  "detail": "request validation failed",
  "kind": "validation",
  "request_id": null,
  "status": 422,
  "violations": []
}
```

| Field | Type | Meaning |
| --- | --- | --- |
| `detail` | string | Redacted, human-readable summary of the failure. |
| `kind` | string | Stable machine-readable error category. |
| `request_id` | string or `null` | Request correlation identifier when one is available. |
| `status` | integer | HTTP response status, repeated in the body. |
| `violations` | array | Field-level validation failures. It is empty when no safe field detail is available. |

A violation has this shape:

```json
{
  "detail": "value is invalid",
  "field": "field_name"
}
```

## Status mapping

| HTTP status | `kind` | `detail` |
| ---: | --- | --- |
| 400 | `invalid_request` | `invalid request` |
| 401 | `authentication` | `authentication required` |
| 403 | `authorization` | `authorization failed` |
| 404 | `not_found` | `resource not found` |
| 405 | `method_not_allowed` | `method not allowed` |
| 409 | `conflict` | `resource state conflict` |
| 412 | `precondition` | `resource precondition failed` |
| 413 | `payload_too_large` | `request body is too large` |
| 422 | `validation` | `request validation failed` |
| 425 | `in_progress` | `matching request is still in progress` |
| 428 | `precondition_required` | `request precondition is required` |
| 429 | `rate_limited` | `request rate limit exceeded` |
| 500–599 | `internal` | `internal server error` |
| Any other error status | `request_failed` | `request failed` |

The administrator API can return 401, 403, 405, 409, 413, 422, 429, and 500. A 403 also covers
CSRF rejection. A 429 response includes:

```http
Retry-After: 60
```

The notification creation API returns 422 for invalid JSON or an invalid payload and 500 for
persistence failures. Generated CRUD routes additionally use 404 for an unknown resource or route
and may use 412, 425, and 428 for concurrency and idempotency preconditions.

Each operation in the generated OpenAPI document lists the subset of these statuses that the route
declares. Error bodies use the `ApiProblem` schema.

## Information disclosure

Internal error details are retained only in server telemetry. API responses do not expose database
errors, SQL, password data, backtraces, source locations, or internal error chains. All 5xx
responses therefore use the same redacted `internal` problem document.
