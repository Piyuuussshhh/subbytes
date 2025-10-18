use std::io;

mod polynomial;
use polynomial::object::GF2NPolynomial;

fn main() {
    // Take user input plaintext.
    // Divide it into 128-bit (16 byte) chunks.
    // For each chunk, make a 4x4 state matrix.
    // For each byte in the state matrix, column wise, do the following:
    //   1. Convert byte to polynomial.
    //   2. Get the inverse of the polynomial in GF(2^8) wrt the AES irreducible polynomial.
    //   3. Display the inverse.
    //   4. Perform the affine transformations.
    //   5. Store it in the state matrix.
    // Finally, display the state matrix.

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't take input");
    input = input.trim().to_string();

    let chunks = get_128bit_chunks(input);
    display_128bit_chunks(&chunks);

    for chunk in chunks {
        let state_matrix = get_state_matrix(chunk);

        for byte in state_matrix {

        }
    }
}

fn get_128bit_chunks(input: String) -> Vec<[u8; 16]> {
    let bytes = input.as_bytes();
    let mut chunked = bytes
        .chunks_exact(16)
        .map(|chunk| {
            let mut arr = [0u8; 16];
            arr.copy_from_slice(chunk);
            arr
        })
        .collect::<Vec<[u8; 16]>>();

    let remainder = bytes.chunks_exact(16).remainder();
    if !remainder.is_empty() {
        // Here we apply padding (e.g., PKCS#7 padding) to the last block
        let mut last_chunk = [0u8; 16];
        let pad_len = 16 - remainder.len();
        last_chunk[..remainder.len()].copy_from_slice(remainder);
        for i in remainder.len()..16 {
            last_chunk[i] = pad_len as u8;
        }
        chunked.push(last_chunk);
    }

    chunked
}

fn display_128bit_chunks(chunks: &Vec<[u8; 16]>) {
    let mut idx = 1;
    for chunk in chunks {
        match str::from_utf8(chunk) {
            Ok(s) => println!("Chunk {idx}: {}", s),
            Err(e) => println!("Conversion failed: {}", e),
        }
        idx += 1;
    }
}

fn get_state_matrix(chunk: [u8; 16]) -> [[u8; 4]; 4] {
    let mut state_matrix = [[0u8; 4]; 4];
    for col in 0..4 {
        for row in 0..4 {
            state_matrix[row][col] = chunk[col * 4 + row];
        }
    }
    state_matrix
}

fn display_state_matrix() {}