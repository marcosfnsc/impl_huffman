use std::env;
use std::fs;
use std::io::{BufWriter, Write};

mod huff;
mod utils;

fn menu() {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "-h" {
        menu();
    } else if args.len() == 4 && args[1] == "-c" && args[2] == "-f" {
        //compress

        let array_file = fs::read(&args[3]).unwrap();
        let frequency = huff::frequency(&array_file);
        let node_root = huff::create_tree(&frequency);

        let mut file = BufWriter::new(fs::File::create(args[3].clone() + ".huff").unwrap());
        huff::save_tree(&node_root, &mut file);

        let mut bytes = Vec::new();
        for byte in array_file {
            let mut new_byte = huff::encode_element(byte, &node_root);
            bytes.append(&mut new_byte);
        }

        let residual = {
            let mut size_array = bytes.len();
            while size_array % 8 != 0 {
                size_array += 1;
            }
            size_array - bytes.len()
        };

        file.write(&[residual as u8]).unwrap();
        for _ in 0..residual {
            bytes.push(0);
        }

        let mut idx_begin = 0;
        let mut idx_last = 8;
        while bytes.len() - 1 > idx_last {
            file.write(&[utils::bitvec_to_decimal(&bytes[idx_begin..idx_last])])
                .unwrap();
            idx_begin += 8;
            idx_last += 8;
        }
        file.write(&[residual as u8]).unwrap();

    } else if args.len() == 4 && args[1] == "-d" && args[2] == "-f" {
        // descompress

        let mut array_file = fs::read(&args[3]).unwrap();
        array_file.reverse();
        let node_root = huff::restore_tree(&mut array_file);
        array_file.reverse();

        let residual = array_file.pop().unwrap();
        let mut array_file_converted = {
            let mut bitvec = Vec::new();
            for byte in array_file {
                bitvec.extend_from_slice(&utils::decimal_to_bitvec(byte));
            }
            bitvec
        };

        // remove residual  bits
        for _ in 0..residual {
            array_file_converted.pop().unwrap();
        }

        let mut file = fs::File::create(&args[3][..args[3].len() - 5]).unwrap();

        array_file_converted.reverse();
        while array_file_converted.len() != 0 {
            file.write(&[huff::decode_element(&mut array_file_converted, &node_root)])
                .unwrap();
        }

    } else if args.len() == 4 && args[1] == "-s" && args[2] == "-f" {
        // tabela de simbolos

        let array_file = fs::read(&args[3]).unwrap();
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
                _ => print!("{:<10}", *key as char),
            }

            print!("{:<12} {:<11} {:<11} {}", value, value, key, " ");
            for bit in huff::encode_element(*key, &node_root) {
                print!("{}", bit)
            }
            println!(""); // pular linha
        }
    }
}
