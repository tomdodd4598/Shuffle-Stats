use std::fs::File;
use std::io::{BufWriter, Write};
use std::thread::JoinHandle;

pub fn join_all<T>(threads: Vec<JoinHandle<T>>) -> Vec<T> {
    threads.into_iter().map(|x| x.join().unwrap()).collect()
}

pub fn vec_chunks<T: Clone>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    vec.chunks(chunk_size).map(|x| x.to_vec()).collect()
}

pub fn file_write(path: &str, string: &str) {
    BufWriter::new(File::create(path).unwrap()).write_all(string.as_bytes()).unwrap()
}
