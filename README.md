# 📌 Skill Matching Engine (Rust + Axum)

An **AI-powered skill matching microservice** written in Rust ⚡.  
This service parses resumes (PDF/TXT) and matches candidate skills against job descriptions or internship postings.  

Built as part of our **Internship & Job Recommender System (Ministry of Corporate Affairs project)**.

---

## ✨ Features
- 📄 **Resume Parsing**
  - Upload resumes in **PDF** or **TXT** format  
  - Extracts raw text using [`pdftotext`](https://poppler.freedesktop.org/)  

- 🎯 **Skill Extraction**
  - Uses regex patterns to detect skills (Rust, C++, Python, SQL, Docker, React, etc.)  
  - Skill list is easily customizable in `main.rs`  

- 🤝 **Job Matching**
  - Compares candidate skills with job requirements  
  - Uses **Jaccard similarity** to calculate overlap  
  - Weighted scoring system (skills = 60%, experience = 25% [future], education = 15% [future])  

- 🌐 **HTTP API (Axum)**
  - `/parse` → Upload a resume file → returns raw text  
  - `/match` → Send candidate + jobs → returns match scores  

---

## 🛠 Tech Stack
- **Language**: Rust 🦀  
- **Framework**: [Axum 0.7](https://github.com/tokio-rs/axum)  
- **Async Runtime**: [Tokio](https://tokio.rs/)  
- **Dependencies**: serde, regex, once_cell, anyhow  
- **External Tool**: [`pdftotext`](https://poppler.freedesktop.org/) for PDF parsing  

---

## 📂 Project Structure
```
skillMatchingEngine/
├── Cargo.toml
├── Dockerfile
└── src/
    └── main.rs
```

---

## ⚡ Setup & Installation

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install `pdftotext` (for PDF parsing)
```bash
sudo apt-get install poppler-utils    # Linux
brew install poppler                  # macOS
```

### 3. Clone and Build
```bash
git clone https://github.com/your-username/skillMatchingEngine.git
cd skillMatchingEngine
cargo build --release
```

### 4. Run
```bash
cargo run
```

Output:
```
Resume matcher listening on 127.0.0.1:8081
```

---

## 🚀 API Usage

### 1. Upload Resume (`/parse`)
```bash
curl -X POST http://127.0.0.1:8081/parse   -F "file=@resume.pdf"
```

Response:
```
<raw extracted text from resume>
```

---

### 2. Match Candidate to Jobs (`/match`)
```bash
curl -X POST http://127.0.0.1:8081/match -H "Content-Type: application/json" -d '{
  "candidate": {
    "name": "Alice",
    "raw_text": "Alice knows Rust, Docker, PostgreSQL, and Python. Worked with NLP."
  },
  "jobs": [
    {
      "id": "job1",
      "title": "Backend Intern",
      "description": "Looking for Rust developer with Docker and PostgreSQL skills.",
      "required_skills": ["rust", "docker", "postgresql"]
    },
    {
      "id": "job2",
      "title": "Frontend Intern",
      "description": "React and JavaScript work with HTML/CSS.",
      "required_skills": ["react", "javascript", "html", "css"]
    }
  ],
  "top_k": 2
}'
```

Response:
```json
[
  {
    "job_id": "job1",
    "score": 60.0,
    "matched_skills": ["docker","postgresql","rust"],
    "explanation": "skill_jaccard=0.600"
  },
  {
    "job_id": "job2",
    "score": 0.0,
    "matched_skills": [],
    "explanation": "skill_jaccard=0.000"
  }
]
```

---

## 🐳 Run with Docker

### 1. Build Image
```bash
docker build -t skill-matching-engine .
```

### 2. Run Container
```bash
docker run -p 8081:8081 skill-matching-engine
```

Service will be available at:
```
http://127.0.0.1:8081
```

---

## 🔮 Roadmap
- [ ] Support **DOCX** parsing (via `zip` crate)  
- [ ] Extract **experience** (years, roles, dates)  
- [ ] Extract **education** & qualifications  
- [ ] Semantic similarity using embeddings (AI/NLP)  
- [ ] Merge `/parse` + `/match` into one API (`/match_with_file`)  

---

## 📜 License
MIT License © 2025  
