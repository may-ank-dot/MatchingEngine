use axum::{
    extract::{Json, Multipart},
    routing::post,
    Router,
};
use axum::serve;
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, net::SocketAddr, process::Command, fs};
use regex::Regex;
use once_cell::sync::Lazy;
use anyhow::Result;
use std::path::PathBuf;

// ================== Skill Extraction ====================
static SKILL_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    let skills = vec![
        r"rust\b", r"c\+\+", r"python\b", r"java\b", r"sql\b", r"postgresql\b",
        r"docker\b", r"kubernetes\b", r"linux\b", r"html\b", r"css\b", r"javascript\b",
        r"react\b", r"node\.?js\b", r"nlp\b", r"natural language processing\b",
    ];
    skills
        .into_iter()
        .map(|p| Regex::new(&format!("(?i){}", p)).unwrap())
        .collect()
});

// ================== Data Models ====================
#[derive(Deserialize)]
struct CandidateInput {
    name: Option<String>,
    raw_text: String,
}

#[derive(Deserialize)]
struct JobInput {
    id: String,
    title: String,
    description: String,
    required_skills: Option<Vec<String>>,
}

#[derive(Serialize)]
struct MatchResult {
    job_id: String,
    score: f64,
    matched_skills: Vec<String>,
    explanation: String,
}

#[derive(Deserialize)]
struct MatchRequest {
    candidate: CandidateInput,
    jobs: Vec<JobInput>,
    top_k: Option<usize>,
}

// ================== Core Functions ====================
fn extract_skills_from_text(text: &str) -> Vec<String> {
    let mut found = HashSet::new();
    for re in SKILL_PATTERNS.iter() {
        for cap in re.find_iter(text) {
            found.insert(cap.as_str().to_lowercase());
        }
    }
    let mut v: Vec<String> = found.into_iter().collect();
    v.sort();
    v
}

fn jaccard_similarity(a: &HashSet<String>, b: &HashSet<String>) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let inter = a.intersection(b).count() as f64;
    let uni = a.union(b).count() as f64;
    if uni == 0.0 {
        0.0
    } else {
        inter / uni
    }
}

// ================== Handlers ====================
async fn handle_match(
    Json(payload): Json<MatchRequest>,
) -> Result<Json<Vec<MatchResult>>, (axum::http::StatusCode, String)> {
    let candidate_skills = extract_skills_from_text(&payload.candidate.raw_text);
    let cand_set: HashSet<String> = candidate_skills.iter().cloned().collect();

    let mut results: Vec<MatchResult> = vec![];

    for job in payload.jobs.iter() {
        let mut job_skills: HashSet<String> = HashSet::new();
        if let Some(req) = &job.required_skills {
            for s in req {
                job_skills.insert(s.to_lowercase());
            }
        }
        let job_extracted = extract_skills_from_text(&job.description);
        for s in job_extracted {
            job_skills.insert(s.to_lowercase());
        }

        let skill_score = jaccard_similarity(&cand_set, &job_skills);
        let experience_score = 0.0f64;

        let final_score =
            100.0 * (0.6 * skill_score + 0.25 * experience_score + 0.15 * 0.0);

        let matched: Vec<String> =
            cand_set.intersection(&job_skills).cloned().collect();

        let explanation = format!("skill_jaccard={:.3}", skill_score);

        results.push(MatchResult {
            job_id: job.id.clone(),
            score: (final_score * 100.0).round() / 100.0,
            matched_skills: matched,
            explanation,
        });
    }

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    let top_k = payload.top_k.unwrap_or(results.len()).min(results.len());
    results.truncate(top_k);

    Ok(Json(results))
}

async fn handle_parse(mut multipart: Multipart) -> Result<String, (axum::http::StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("upload").to_string();
        let data = field.bytes().await.unwrap();

        let path = PathBuf::from(format!("/tmp/{}", file_name));
        fs::write(&path, &data).unwrap();

        let text = if file_name.ends_with(".pdf") {
            // Use `pdftotext` (must be installed: sudo apt install poppler-utils)
            let output = Command::new("pdftotext")
                .arg(&path)
                .arg("-") // output to stdout
                .output()
                .unwrap();
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            // Assume plain text
            String::from_utf8_lossy(&data).to_string()
        };

        return Ok(text);
    }
    Err((axum::http::StatusCode::BAD_REQUEST, "No file uploaded".into()))
}

// ================== Main ====================
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/match", post(handle_match))
        .route("/parse", post(handle_parse));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("Resume matcher listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

