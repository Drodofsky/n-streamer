mod analyzed_schedule;
mod parsed_schedule;
pub use analyzed_schedule::*;
pub use parsed_schedule::*;

pub use super::*;
pub async fn get_analyzed_schedule() -> Result<AnalyzedSchedule, Error> {
    let json: ScheduleRequest =
        reqwest::get("https://nhkworldpremium.com/backend/api/v1/front/episodes?lang=en")
            .await?
            .json()
            .await?;
    if json.status != 400 {
        return Err(Error::Api(format!("API: {}", json.status)));
    }
    json.item.try_into()
}
