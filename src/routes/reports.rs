use rocket::{get, serde::json::Json, http::Status};
use glob::glob;

#[get("/reports")]
pub async fn report_list(
) -> Result<Json<Vec<String>>, Status> {
    Ok(Json(glob("output/reports/*.csv").expect("Failed to read glob pattern").map(|x| {
      x.unwrap().file_name().unwrap().to_str().unwrap().to_string()
    }).collect()))
}

#[get("/statistics")]
pub async fn statistics_list(
) -> Result<Json<Vec<String>>, Status> {
    Ok(Json(glob("output/reports/*.json").expect("Failed to read glob pattern").map(|x| {
      x.unwrap().file_name().unwrap().to_str().unwrap().to_string()
    }).collect()))
}
