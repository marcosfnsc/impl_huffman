use std::fs;
use std::io::{BufWriter, Write};

mod huff;
mod utils;

pub fn menu() {
    let space = " ";
    println!("Compressão de Huffman – Análise de frequência símbolos e compressão de Huffman");
    println!("Uso: huff [-options] <file>\n");
    println!("Options:\n");
    println!("-h {space:>10} Mostra este texto de ajuda");
    println!("-c {space:>10} Realiza a compressão");
    println!("-d {space:>10} Realiza a descompressão");
    println!(
        "-s {space:>10} Realiza apenas a análise de frequência e imprime a tabela de símbolos"
    );
    println!("-f <file> {space:>3} Indica o arquivo a ser processado (comprimido, descomprimido ou para apresentar a tabela de símbolos)");
}

pub fn compress(filename: &str) -> Result<(), std::io::Error> {
    let array_file = fs::read(filename)?;
    let frequency = huff::frequency(&array_file);
    let node_root = huff::create_tree(&frequency);

    let mut file = BufWriter::new(fs::File::create(format!("{filename}.huff"))?);
    huff::save_tree(&node_root, &mut file);

    let bytes: Vec<u8> = array_file
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
    file.write_all(&[residual as u8])?;

    let bytes = vec![(0..residual as u8).collect(), bytes];
    let bytes: Vec<u8> = bytes.into_iter().flatten().collect();

    let mut idx_begin = 0;
    let mut idx_last = 8;
    while bytes.len() >= idx_last {
        file.write_all(&[utils::bitvec_to_decimal(&bytes[idx_begin..idx_last])])?;
        idx_begin += 8;
        idx_last += 8;
    }
    Ok(())
}

pub fn decompress(filename: &str) -> Result<(), std::io::Error> {
    let array_file = fs::read(filename)?;
    let mut array_iter = array_file.into_iter();
    let node_root = huff::restore_tree(&mut array_iter);

    //let filename = filename.replace(".huff", "");
    //let mut file = fs::File::create(filename)?;

    let residual = array_iter.next().unwrap();
    let mut array_file_converted = array_iter
        .flat_map(utils::decimal_to_bitvec)
        .skip(residual as usize) // remove residual  bits
        .peekable();
    while array_file_converted.peek().is_some() {
        let x = huff::decode_element(&mut array_file_converted, &node_root) as char;
        print!("{x}");
    }
    Ok(())
}

pub fn analyze(filename: &str) -> Result<(), std::io::Error> {
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
