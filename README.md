# Reddit Scraper with AI Analysis

A Rust-based application that analyzes Reddit posts using AI models to provide semantic search and question answering capabilities.

![Reddit Scraper Architecture](https://mermaid.ink/img/pako:eNp1kU1rAjEQhv_KMKcuFKTeFLq77R70UBDxEpaQZNzg5oNkIrjif-9mV1q7xVwy87x5mUymsRZLFKJ2rg_k-CpCJzP7JnA8uEVrk1Jko5bW5NLCOcHGxs4QX3nzTqkaSGa9wfbWOVnlUtdUHkXuiUJKTnW0kQjO4GJw3NtN9q7qzh3pCXKVQcaegxHCqsebxNfYkCvnZ-5sQnM1G7JKhj6jQG3e3hUlcpQRTvEzXfN7KcS_pL8lB9lH83WZV59qGC1rl4Bf3c_GXJiJ-Nf1GOvw5zVE3gzgbdkG25GHwaeI1hLKYv5YLDdF8VgUEILvXKcogoxFFJPWkbcO3WwAM8QX4qEPJHGijR59nOiVpOGjLMWyG9YM3DzxO5UG3dRxpAm9xVLcAGzFsuM?type=png)

## Table of Contents
- [Overview](#overview)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Quick Setup](#quick-setup)
- [Running the Application](#running-the-application)
- [Example Usage](#example-usage)
- [Troubleshooting](#troubleshooting)

## Overview

This project demonstrates how to use AI models for semantic understanding and question answering on Reddit content. It:

1. Uses pre-scraped Reddit posts data (included in the repo)
2. Processes the text into chunks
3. Creates embeddings of these chunks using the nomic-embed-text model
4. Stores these embeddings in a Qdrant vector database
5. Allows users to ask questions about the content
6. Retrieves relevant post sections and generates answers using the llama3.2 model

## Architecture

The application consists of several components:

- **Data Source**: Pre-scraped Reddit posts (JSON)
- **Text Processing**: Chunking module that breaks down posts into manageable pieces
- **Embeddings**: Ollama's nomic-embed-text model converts text into vector representations
- **Vector Storage**: Qdrant database stores and enables semantic search of the vectors
- **Question Answering**: Ollama's llama3.2 model generates answers based on retrieved content

## Prerequisites

- **Rust** (latest stable version)
- **Docker** for running Qdrant
- **Ollama** for running the AI models

## Quick Setup

### 1. Install Docker

If you don't have Docker installed:
- **macOS/Windows**: Download and install [Docker Desktop](https://www.docker.com/products/docker-desktop)
- **Linux**: Follow the [installation instructions](https://docs.docker.com/engine/install/) for your distribution

### 2. Install Ollama

Download and install Ollama from the [official website](https://ollama.ai/download).

### 3. Install Rust

If you don't have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 4. Clone the Repository

```bash
git clone <repository-url>
cd reddit_scraper
```

### 5. Set Up Qdrant

Start the Qdrant vector database with Docker:

```bash
docker run -d --name qdrant -p 6333:6333 -v $(pwd)/qdrant_storage:/qdrant/storage qdrant/qdrant
```

### 6. Pull Ollama Models

Pull the required models:

```bash
ollama pull llama3.2
ollama pull nomic-embed-text
```

### 7. Configure Environment

Create a `.env` file in the project root:

```bash
cat > .env << EOL
USERNAME="username"
CLIENT_ID="client_id"
COLLECTION_NAME="reddit_posts"
BASE_URL="http://localhost:6333/"
EOL
```

Note: For the demo with dummy data, you don't need real Reddit API credentials.

## Running the Application

### 1. Build the Project

```bash
cargo build --release
```

### 2. Run the Application

```bash
cargo run --release
```

The application will:
1. Load the pre-scraped Reddit posts from `reddit_posts.json`
2. Process the posts and create embeddings
3. Store these in Qdrant
4. Ask a sample question about the posts
5. Display the AI-generated answer

## Example Usage

The default execution asks the question: "What are these posts talking about?"

To modify the question, edit the `user_question` variable in `src/main.rs`:

```rust
let user_question = "What are the main topics in these Reddit posts?";
```

Then rebuild and run the application.

### Sample Output

```
Loading Reddit posts from file...
Posts prepared successfully
Processing and embedding post fd8h2m...
embeddings prepared successfully
✅ Stored vector for post fd8h2m
Processing and embedding post gh71kr...
embeddings prepared successfully
✅ Stored vector for post gh71kr
...
Searching for posts similar to question...
Generating answer with llama3.2...

Based on the Reddit posts provided, the main topics being discussed are:
1. Rust programming language features and patterns
2. Community discussions about specific Rust crates and libraries
3. Questions about error handling and best practices
4. Performance considerations and optimizations in Rust
5. Comparisons between Rust and other programming languages
...
```

## Troubleshooting

### Docker Issues

**Problem**: Qdrant container fails to start  
**Solution**: Make sure Docker is running and check if the port 6333 is already in use:
```bash
docker logs qdrant
```

To stop and remove the container to start fresh:
```bash
docker stop qdrant
docker rm qdrant
```

### Ollama Issues

**Problem**: "Model not found" error  
**Solution**: Ensure you've pulled the required models:
```bash
ollama pull llama3.2
ollama pull nomic-embed-text
```

**Problem**: Ollama service not running  
**Solution**: Start the Ollama service:
```bash
# macOS/Linux
ollama serve

# Windows
# Start from the application menu
```

### Rust Build Issues

**Problem**: Compilation errors  
**Solution**: Make sure you have the latest stable Rust:
```bash
rustup update stable
```

### Vector Storage Issues

**Problem**: Application can't connect to Qdrant  
**Solution**: Check if Qdrant is running and accessible:
```bash
curl http://localhost:6333/collections
```

If it returns an error, restart the Docker container.

---

## Acknowledgements

- [Reddit API](https://www.reddit.com/dev/api/)
- [Ollama](https://ollama.ai)
- [Qdrant](https://qdrant.tech)
- [Rust Programming Language](https://www.rust-lang.org)

