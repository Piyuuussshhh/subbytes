use std::io;

mod polynomial;
use polynomial::object::GF2NPolynomial;
use polynomial::irr_poly::get_irreducible_polynomial;

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

    println!("Enter plaintext:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't take input");
    input = input.trim().to_string();

    let chunks = get_128bit_chunks(input);
    display_128bit_chunks(&chunks);
    let ciphertext_chunks = chunks.clone();

    for chunk in chunks.iter() {
        let mut state_matrix = get_state_matrix(*chunk);
        println!("State Matrix before SubBytes: ");
        display_state_matrix(state_matrix);
        let rows = state_matrix.len();
        let cols = state_matrix[0].len();

        for row in 0..rows {
            for col in 0..cols {
                let byte = state_matrix[row][col];
                println!("\nProcessing byte at position ({}, {}) of the state matrix: 0x{byte:02X}", row, col);
                let poly = GF2NPolynomial::from_byte(byte);
                let inv_poly = poly.inverse(&get_irreducible_polynomial(8));
                println!("Inverse of byte 0x{byte:02X} is 0x{}", inv_poly.hex_string());

                // Affine transformation.
                let res_byte = matrix_multiplication_subbytes(byte);
                let res_byte = res_byte ^ 0x63;
                println!("After affine transformation: 0x{res_byte:02X}\n------------------------------------\n");

                state_matrix[row][col] = res_byte;
            }
        }

        println!("State Matrix after SubBytes: ");
        display_state_matrix(state_matrix);
    }

    println!("\nFinal Ciphertext after SubBytes for all chunks:");
    display_final_ciphertext(&chunks);
    chunks_to_string(&chunks);
}

fn matrix_multiplication_subbytes(mut byte: u8) -> u8 {
    let affine_matrix: [[u8; 8]; 8] = [
        [1, 0, 0, 0, 1, 1, 1, 1],
        [1, 1, 0, 0, 0, 1, 1, 1],
        [1, 1, 1, 0, 0, 0, 1, 1],
        [1, 1, 1, 1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 0, 1, 1, 1, 1, 1, 0],
        [0, 0, 0, 1, 1, 1, 1, 1],
    ];
    let byte_vector: [u8; 8] = [
        (byte >> 7) & 1,
        (byte >> 6) & 1,
        (byte >> 5) & 1,
        (byte >> 4) & 1,
        (byte >> 3) & 1,
        (byte >> 2) & 1,
        (byte >> 1) & 1,
        (byte >> 0) & 1,
    ];

    for i in 0..8 {
        let mut sum = 0u8;
        for j in 0..8 {
            sum ^= affine_matrix[i][j] & byte_vector[j];
        }
        byte &= !(1 << (7 - i)); // Clear the bit at position (7 - i)
        byte |= sum << (7 - i);  // Set the bit at position (7 - i) to sum
    }

    byte
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

fn get_state_matrix(chunk: [u8; 16]) -> [[u8; 4]; 4] {
    let mut state_matrix = [[0u8; 4]; 4];
    for col in 0..4 {
        for row in 0..4 {
            state_matrix[row][col] = chunk[col * 4 + row];
        }
    }
    state_matrix
}

fn chunks_to_string(chunks: &[[u8; 16]]) {
    let bytes: Vec<u8> = chunks.iter().flatten().copied().collect();
    println!("{}", String::from_utf8_lossy(&bytes).to_string());
}

fn display_128bit_chunks(chunks: &Vec<[u8; 16]>) {
    for (i, chunk) in chunks.iter().enumerate() {
        print!("Chunk {}: ", i + 1);
        for &byte in chunk {
            print!("{:02X} ", byte);
        }
        println!();
    }
}

fn display_subbed_chunk(chunk: &[u8; 16]) {
    for &byte in chunk {
        print!("{:02X} ", byte);
    }
}

fn display_final_ciphertext(chunks: &Vec<[u8; 16]>) {
    for chunk in chunks {
        display_subbed_chunk(chunk);
    }
    println!();
}

fn display_state_matrix(matrix: [[u8; 4]; 4]) {
    for row in 0..4 {
        for col in 0..4 {
            print!("{:02X}\t", matrix[row][col]);
        }
        println!();
    }
}