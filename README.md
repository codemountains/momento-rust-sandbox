# Momento Sandbox for Rust SDK

[Momento Cache – Momento](https://jp.gomomento.com/services/momento-cache/)

## Setup

### Create `.env`

```
MOMENTO_AUTH_TOKEN=
MOMENTO_CACHE_NAME=
```

### Set up the API Key

1. Go to the [Momento console](https://console.gomomento.com/tokens) and follow the instructions to log in with your email address, Google account, or GitHub account.
2. In the console, select the [API Keys](https://console.gomomento.com/tokens) menu option.
3. Once on the API key page, select the information that matches where your caches live.
4. Once complete, click on the Generate button to create your API Key.

## Testing

```shell
cargo run
```
