use clap::{Arg, Command};

mod image_converter;
mod slideshow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("rust24k")
        .version("0.1.0")
        .about("Converte imagens para formato UHD e gera slideshows")
        .arg(
            Arg::new("slideshow")
                .long("slideshow")
                .help("Gera um slideshow das imagens convertidas")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("all")
                .long("all")
                .short('a')
                .help("Converte imagens e gera slideshow em uma única execução")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("duration")
                .long("duration")
                .short('d')
                .value_name("SECONDS")
                .help("Duração de cada imagem no slideshow em segundos")
                .default_value("3.0")
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .value_name("FILE")
                .help("Nome do arquivo de saída do slideshow")
                .default_value("slideshow.mp4")
        )
        .get_matches();

    let duration: f64 = matches.get_one::<String>("duration")
        .unwrap()
        .parse()
        .map_err(|_| "Duração deve ser um número válido")?;
    let output = matches.get_one::<String>("output").unwrap();

    if matches.get_flag("all") {
        // Modo completo: converter imagens e gerar slideshow
        println!("🔄 Iniciando conversão de imagens para UHD...");
        image_converter::convert_images()?;
        
        println!("\n🎬 Gerando slideshow...");
        return slideshow::generate_slideshow(duration, output);
    }

    if matches.get_flag("slideshow") {
        // Modo slideshow apenas
        return slideshow::generate_slideshow(duration, output);
    }

    // Modo padrão: converter imagens para UHD apenas
    image_converter::convert_images()
}
