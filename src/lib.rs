use std::fs;
use std::io::{self, BufWriter, Write};

mod huff;
mod utils;

pub fn compress(filename: &str) -> io::Result<()> {
    let array_file = fs::read(filename)?;
    let frequency = huff::frequency(&array_file);
    let node_root = huff::create_tree(&frequency);
    let mut bytes: Vec<u8> = array_file
        .into_iter()
        .flat_map(|byte| huff::encode_element(byte, &node_root))
        .collect();
    let residual = {
        let mut size_array = bytes.len();
        while size_array % 8 != 0 {
            size_array += 1;
        }
        size_array - bytes.len()
    };
    let bytes = {
        let mut v = vec![0; residual];
        v.append(&mut bytes);
        v
    };

    let filename = format!("{filename}.huff");
    let mut file = BufWriter::new(fs::File::create(filename)?);
    huff::save_tree(&node_root, &mut file);
    file.write_all(&[residual as u8])?;
    for idx in (0..bytes.len()).step_by(8) {
        file.write_all(&[utils::bitvec_to_decimal(&bytes[idx..idx + 8])])?;
    }
    Ok(())
}

pub fn decompress(filename: &str) -> io::Result<()> {
    let array_file = fs::read(filename)?;
    let mut array_iter = array_file.into_iter();
    let node_root = huff::restore_tree(&mut array_iter);

    let residual = array_iter.next().unwrap();
    let mut array_iter = array_iter
        .flat_map(utils::decimal_to_bitvec)
        .skip(residual as usize) // remove residual  bits
        .peekable();

    let filename = filename.replace(".huff", "");
    let mut file = BufWriter::new(fs::File::create(filename)?);
    while array_iter.peek().is_some() {
        let byte = huff::decode_element(&mut array_iter, &node_root);
        file.write_all(&[byte])?;
    }
    Ok(())
}

pub fn analyze(filename: &str) -> io::Result<()> {
    let array_file = fs::read(filename)?;
    let array_nodes = huff::frequency(&array_file);
    let node_root = huff::create_tree(&array_nodes);

    println!("Simbolo | Freq. Abs. | Freq. Rel | Cod. ASCII | Cod. Huffman");
    for (key, value) in array_nodes {
        match key {
            0 => print!("{:<10}", "NULL"),
            8 => print!("{:<10}", "BACKSPACE"),
            9 => print!("{:<10}", "TAB"),
            10 => print!("{:<10}", "\\n"),
            11 => print!("{:<10}", "VTAB"),
            27 => print!("{:<10}", "ESC"),
            32 => print!("{:<10}", "SPACE"),
            127 => print!("{:<10}", "DEL"),
            _ => print!("{:<10}", key as char),
        }

        print!("{value:<12} {value:<11} {key:<12} ");
        for bit in huff::encode_element(key, &node_root) {
            print!("{bit}")
        }
        println!(); // pular linha
    }
    Ok(())
}
