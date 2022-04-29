use clap::{arg, command};
use huff::{analyze, compress, decompress};

fn main() {
    let matches = command!()
        .about("Análise de frequência de símbolos e compressão de Huffman")
        .version("1.0")
        .arg(arg!(-c --compress   <filename> "compress the file").required(false))
        .arg(arg!(-d --decompress <filename> "decompress the file").required(false))
        .arg(arg!(-s --analyze    <filename> "analyze the file").required(false))
        .get_matches();

    if let Some(filename) = matches.value_of("compress") {
        compress(filename).unwrap();
    } else if let Some(filename) = matches.value_of("decompress") {
        decompress(filename).unwrap();
    } else if let Some(filename) = matches.value_of("analyze") {
        analyze(filename).unwrap();
    }
}
