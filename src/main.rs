use rand::Rng;
use std::fs::File;
use std::io::{self, Write};

fn generate_vectors(dims: usize, count_vectors: usize) -> Vec<Vec<f64>> {

    println!("Generating {count_vectors} vectors of {dims} dimenssions each");

    let mut rng = rand::thread_rng();
    let mut vectors = Vec::with_capacity(count_vectors);

    for _ in 0..count_vectors {
        let vector: Vec<f64> = (0..dims)
            .map(|_| rng.gen_range(0.0..1.0))
            .collect();
        vectors.push(vector);
    }
    return vectors;
}

fn write_vectors_to_file(vectors: &[Vec<f64>], filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    
    for vector in vectors {
        // Convert each number to a string and join with commas
        let line: String = vector
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
            
        // Write the line with a newline character
        writeln!(file, "{}", line)?;
    }
    
    Ok(())
}

fn main() -> io::Result<()> {
    // Generate 3 vectors of size 4 with values between 0.0 and 1.0
    let random_vectors = generate_vectors(1024, 40);
    
    // Write the generated vectors to file
    write_vectors_to_file(&random_vectors, "vectors_1024_40.txt")?;
    
    Ok(())
}
