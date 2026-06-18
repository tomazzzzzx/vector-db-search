mod query;
pub struct VectorDB { dim: usize, vecs: Vec<(u64, Vec<f32>)> }
impl VectorDB {
    pub fn new(dim: usize) -> Self { Self { dim, vecs: Vec::new() } }
    pub fn insert(&mut self, id: u64, v: Vec<f32>) { self.vecs.push((id, v)); }
    pub fn search(&self, q: &[f32], k: usize) -> Vec<query::Result> {
        let mut r: Vec<_> = self.vecs.iter().map(|(id,v)| {
            let d: f32 = q.iter().zip(v.iter()).map(|(a,b)| a*b).sum();
            query::Result { id: *id, score: d }
        }).collect();
        r.sort_by(|a,b| b.score.partial_cmp(&a.score).unwrap()); r.truncate(k); r
    }
}
fn main() { println!("VectorDB ready"); }
