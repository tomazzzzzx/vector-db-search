# Vector DB Search

GPU-accelerated vector database with HNSW indexing and IVF-PQ fallback.

## Features
- HNSW index with GPU distance computation
- IVF-PQ for billion-scale datasets
- Metadata filtering (pre/post)
- REST API + gRPC interface

## Performance (1M vectors, dim=768)
| Method | QPS | Recall@10 |
|--------|-----|----------|
| HNSW (GPU) | 45K | 98.2% |
| IVF-PQ (GPU) | 120K | 95.1% |

## License: MIT
