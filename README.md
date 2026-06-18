# Vector Db Search

GPU-accelerated vector database with HNSW indexing.

## Features
- HNSW index with GPU-accelerated distance computation
- IVF-PQ for billion-scale datasets
- Metadata filtering with pre/post filtering
- REST API and gRPC

## Performance (1M vectors, dim=768)
| Method | QPS | Recall@10 |
|--------|-----|----------|
| HNSW (GPU) | 45K | 98.2% |
| IVF-PQ (GPU) | 120K | 95.1% |

## License
MIT
