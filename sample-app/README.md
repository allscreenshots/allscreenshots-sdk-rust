# Rust SDK demo

This local browser app calls the generated SDK at the repository root. It supports sync capture, async submit/poll/download, quota, image preview, and PNG download.

Build the SDK and demo using `python3 tools/verify.py --build`. Set `ALLSCREENSHOTS_API_KEY` in your shell, optionally set `ALLSCREENSHOTS_BASE_URL`, then run `python3 sample-app/serve.py` and open http://127.0.0.1:7070. The key is never sent to the demo browser.

You can also run the worker with `quota`, `sync`, or `async`; see `sdk-build.json` for its command. Capture workers use responseType URL with a submission key, then download through the SDK. To recover a submission, reuse `ALLSCREENSHOTS_IDEMPOTENCY_KEY` with the same request.
