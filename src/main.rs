use std::env;
use std::fs;
//use std::io::Write;

mod huff;
mod utils;

fn menu() {
    println!("Compressão de Huffman – Análise de frequência símbolos e compressão de Huffman");
    println!("Uso: huff [-options] <file>\n");
    println!("Options:\n");
    println!("-h {:>10} Mostra este texto de ajuda", " ");
    println!("-c {:>10} Realiza a compressão", " ");
    println!("-d {:>10} Realiza a descompressão", " ");
    println!("-s {:>10} Realiza apenas a análise de frequência e imprime a tabela de símbolos", " ");
    println!("-f <file> {:>3} Indica o arquivo a ser processado (comprimido, descomprimido ou para apresentar a tabela de símbolos)", " ");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "-h" {
        menu();
    } else if args.len() == 4 && args[1] == "-c" && args[2] == "-f" {
        //compress

        /*
        let array_file = fs::read(&args[3]).unwrap();
        let mut array_nodes = huff::frequency(&mut array_file.clone());
        let node_root = huff::create_tree(&mut array_nodes);

        let mut file = fs::File::create(args[3].clone()+".huff").unwrap();
        huff::tree_to_file(&node_root, &mut file);

        let mut bytes = Vec::new();
        for byte in array_file {
            let mut new_byte = huff::encode_elt(byte, &node_root);
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
            bytes.insert(0, 0);
        }

        while bytes.len() > 1 {
            file.write(&[utils::bitvec_to_decimal(&bytes[0..8])]).unwrap();
            bytes.drain(0..8);
        }
        */

    } else if  args.len() == 4 && args[1] == "-d" && args[2] == "-f" {
        // descompress

        /*
        let mut array_file = fs::read(&args[3]).unwrap();
        let node_root = huff::file_to_tree(&mut array_file).unwrap();

        let residual = array_file.remove(0);
        let mut array_file_converted = {
            let mut bitvec = Vec::new();
            for byte in array_file {
                bitvec.append(&mut utils::decimal_to_bitvec(byte));
            }
            bitvec
        };
        array_file_converted.drain(0..residual as usize);
        let mut file = fs::File::create(&args[3][..args[3].len()-5]).unwrap();

        while array_file_converted.len() != 0 {
            file.write(&[huff::decode_elt(&mut array_file_converted, &node_root)]).unwrap();
        }
        */

    } else if  args.len() == 4 && args[1] == "-s" && args[2] == "-f" {
        // tabela de simbolos

        let mut array_file = fs::read(&args[3]).unwrap();
        let array_nodes = huff::frequency(&mut array_file);
        let node_root = huff::create_tree(&mut array_nodes.clone());

        println!("Simbolo | Freq. Abs. | Freq. Rel | Cod. ASCII | Cod. Huffman");
        for node in &array_nodes {

            match node.get_elt() {
                0   => print!("{:<10}", "NULL"),
                8   => print!("{:<10}", "BACKSPACE"),
                9   => print!("{:<10}", "TAB"),
                10  => print!("{:<10}", "\\n"),
                11  => print!("{:<10}", "VTAB"),
                27  => print!("{:<10}", "ESC"),
                32  => print!("{:<10}", "SPACE"),
                127 => print!("{:<10}", "DEL"),
                _   => print!("{:<10}", node.get_elt() as char)
            }

            print!("{:<12} {:<11} {:<11} {}",
                     node.get_freq(),
                     node.get_freq() as f32/array_nodes.len() as f32,
                     node.get_elt(),
                     " ");
            for bit in huff::encode_element(node.get_elt(), &node_root) {
                print!("{}", bit)
            }
            println!(""); // pular linha
        }
    }
}
