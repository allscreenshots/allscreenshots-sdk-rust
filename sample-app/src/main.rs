use allscreenshots_sdk::{apis::{configuration::{Configuration,ApiKey},screenshot_api,job_api,usage_api},models::{ScreenshotRequest,ScreenshotJsonResponse}};
use std::{env,time::{Duration,Instant}};
fn get(name:&str,fallback:&str)->String {env::var(name).unwrap_or_else(|_|fallback.to_owned())}
#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
 let mut config=Configuration::new();config.base_path=get("ALLSCREENSHOTS_BASE_URL","https://api.allscreenshots.com");config.api_key=Some(ApiKey{prefix:None,key:env::var("ALLSCREENSHOTS_API_KEY")?});config.client=reqwest::Client::builder().timeout(Duration::from_secs(180)).build()?;
 let mode=env::args().nth(1).unwrap_or("quota".to_owned());
 if mode=="quota" {let response=usage_api::get_quota(&config).await?;let Some(usage_api::GetQuotaSuccess::Status200(q))=response.entity else{return Err("Invalid quota response".into())};println!("{}",serde_json::to_string(&q)?);return Ok(())}
 let mut request=ScreenshotRequest::new(get("ALLSCREENSHOTS_URL","https://example.com"));request.response_type=Some(allscreenshots_sdk::models::screenshot_request::ResponseType::Url);request.format=Some("png".to_owned());
 let key=get("ALLSCREENSHOTS_IDEMPOTENCY_KEY",&format!("demo-{}",std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos()));
 let bytes=if mode=="sync" {
  let raw=screenshot_api::capture_sync(&config,request,Some(&key)).await?.bytes().await?;
  let metadata:ScreenshotJsonResponse=serde_json::from_slice(&raw)?;let url=metadata.result_url.flatten().ok_or("Missing result URL")?;let parts:Vec<_>=url.split('/').collect();
  screenshot_api::get_sync_capture_result(&config,parts[parts.len()-2]).await?.bytes().await?
 } else if mode=="async" {
  let response=job_api::create_async_job(&config,request,Some(&key)).await?;let Some(job_api::CreateAsyncJobSuccess::Status202(job))=response.entity else{return Err("Invalid async response".into())};let deadline=Instant::now()+Duration::from_secs(180);
  loop {let response=job_api::get_job_status(&config,&job.id).await?;let Some(job_api::GetJobStatusSuccess::Status200(status))=response.entity else{return Err("Invalid job status".into())};let state=serde_json::to_value(status.status)?;if state=="COMPLETED" {break};if state=="FAILED"||state=="CANCELLED" {return Err("Capture failed".into())};if Instant::now()>deadline{return Err("Polling timed out".into())};tokio::time::sleep(Duration::from_secs(2)).await;}
  job_api::get_job_result(&config,&job.id).await?.bytes().await?
 } else {return Err("Expected quota, sync, or async".into())};
 let output=get("ALLSCREENSHOTS_OUTPUT","capture.png");std::fs::write(&output,&bytes)?;println!("{}",serde_json::json!({"output":output}));Ok(())
}
