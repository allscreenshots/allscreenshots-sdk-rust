# Migration from the handwritten SDK

This is a new major SDK version. Move package references from `sdk/` to the repository root; all endpoint clients and models are now generated from OpenAPI. Method names, model constructors, enum names, and errors have changed. Use the generated API reference and `sample-app` as executable examples.

Configure the API base URL and X-API-Key using the generated client configuration. The quota response has nested `screenshots` and `bandwidth` objects. Request defaults come from the backend DTOs. Action, output, and schedule destination discriminators use lower-case wire names.

Capture/download endpoints return raw response bodies to support binary and JSON without guessing a decoder. Use responseType URL and an Idempotency-Key for recoverable submissions. Download results through SDK endpoints; do not forward your API key to storage hosts. Only safe GET polling is retried; the demo never resubmits a capture automatically. Reusing a submission key with a different body fails.
