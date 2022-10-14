use headers::{Authorization, HeaderMapExt};
use hyper::body::HttpBody;
use hyper::{Body, Client, Method, Request, Uri};
use serde_json::json;
use tokio::io::{stdout, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let basic = Authorization::basic("Aladdin", "open sesame");

    let mut req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
              "update": {},
              "fields": {
                "summary": "Main order flow broken",
                "parent": {
                  "key": "PROJ-123"
                },
                "issuetype": {
                  "id": "10000"
                },
                "components": [
                  {
                    "id": "10000"
                  }
                ],
                "customfield_20000": "06/Jul/19 3:25 PM",
                "customfield_40000": {
                  "type": "doc",
                  "version": 1,
                  "content": [
                    {
                      "type": "paragraph",
                      "content": [
                        {
                          "text": "Occurs on all orders",
                          "type": "text"
                        }
                      ]
                    }
                  ]
                },
                "customfield_70000": [
                  "jira-administrators",
                  "jira-software-users"
                ],
                "project": {
                  "id": "10000"
                },
                "description": {
                  "type": "doc",
                  "version": 1,
                  "content": [
                    {
                      "type": "paragraph",
                      "content": [
                        {
                          "text": "Order entry fails when selecting supplier.",
                          "type": "text"
                        }
                      ]
                    }
                  ]
                },
                "reporter": {
                  "id": "5b10a2844c20165700ede21g"
                },
                "fixVersions": [
                  {
                    "id": "10001"
                  }
                ],
                "customfield_10000": "09/Jun/19",
                "priority": {
                  "id": "20000"
                },
                "labels": [
                  "bugfix",
                  "blitz_test"
                ],
                "timetracking": {
                  "remainingEstimate": "5",
                  "originalEstimate": "10"
                },
                "customfield_30000": [
                  "10000",
                  "10002"
                ],
                "customfield_80000": {
                  "value": "red"
                },
                "security": {
                  "id": "10000"
                },
                "environment": {
                  "type": "doc",
                  "version": 1,
                  "content": [
                    {
                      "type": "paragraph",
                      "content": [
                        {
                          "text": "UAT",
                          "type": "text"
                        }
                      ]
                    }
                  ]
                },
                "versions": [
                  {
                    "id": "10000"
                  }
                ],
                "duedate": "2019-05-11",
                "customfield_60000": "jira-software-users",
                "customfield_50000": {
                  "type": "doc",
                  "version": 1,
                  "content": [
                    {
                      "type": "paragraph",
                      "content": [
                        {
                          "text": "Could impact day-to-day work.",
                          "type": "text"
                        }
                      ]
                    }
                  ]
                },
                "assignee": {
                  "id": "5b109f2e9729b51b54dc274d"
                }
              }
            })
            .to_string(),
        ))?;

    let headers = req.headers_mut();
    headers.typed_insert(basic);

    let client = Client::new();

    let mut resp = client.request(req).await?;

    println!("Response: {}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
