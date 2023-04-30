# Hello Axum on Cloud Run

An example of deploying a REST API using Rust and Axum web framework to [GCP Cloud Run](https://cloud.google.com/run).

See more: [Cloud Run で Rust / Axum 製 Web アプリケーションを動かす](https://zenn.dev/collabostyle/articles/89a9171ab0c0e5)

## Deploy

```
gcloud run deploy
```

## Response

URL: `https://{your-cloud-run-url}/mountains`

### 200 OK

```json
{
    "mountains": [
        {
            "id": 1,
            "name": "恐山"
        },
        {
            "id": 2,
            "name": "比叡山"
        },
        {
            "id": 3,
            "name": "高野山"
        }
    ],
    "total": 3
}
```
