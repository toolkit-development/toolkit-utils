use candid::CandidType;
use ic_http_certification::HttpResponse;
use serde::Deserialize;

#[derive(Clone, CandidType, Deserialize)]
pub struct PathEntry {
    pub match_path: Vec<String>,
    pub response: HttpResponse,
}
