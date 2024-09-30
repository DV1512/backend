# Backend

## Versioning

This api uses api versioning to ensure backwards compatibility to other systems and clients.

### Implementation

The api endpoints are versioned and prefixed with `/api/vx` where `x` is the version number.

Each version has its own documentation under the `/api/docs/vx` endpoint. See the [documentation](#documentation) section for more details.

### Current version

The current version is `1.0.0` and endpoints are prefixed with `/api/v1`

## Documentation

There are several ways of viewing the documentation for this api powered by [utoipa](https://github.com/juhaku/utoipa)

There are several versions of the documentation can be found according to the versioning in the [Versioning](#versioning) section.

### Endpoints

Documentation endpoints that do not include a version number are global documentation and include all endpoints for all versions.

- `/api/docs/swagger/` - Swagger UI
- `/api/docs/scalar/` - Scalar UI
- `/api/docs/redoc/` - Redoc UI
- `/api/docs/rapidoc/` - RapiDoc UI
- `/api/docs/openapi.json` - OpenAPI Spec

- `/api/docs/v1/swagger/` - Swagger UI
- `/api/docs/v1/scalar/` - Scalar UI
- `/api/docs/v1/redoc/` - Redoc UI
- `/api/docs/v1/rapidoc/` - RapiDoc UI
- `/api/docs/v1/openapi.json` - OpenAPI Spec
