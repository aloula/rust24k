use std::path::Path;
use std::process::Command as ProcessCommand;

pub fn generate_slideshow(duration: f64, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let converted_dir = Path::new("converted");
    
    if !converted_dir.exists() {
        return Err("Diretório 'converted' não existe. Execute o programa primeiro para converter as imagens.".into());
    }

    // Coletar todas as imagens UHD convertidas
    let mut image_files = Vec::new();
    for entry in std::fs::read_dir(converted_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if let Some(extension) = path.extension() {
            if extension.to_string_lossy().to_lowercase() == "jpg" 
               || extension.to_string_lossy().to_lowercase() == "jpeg" {
                if let Some(filename) = path.file_name() {
                    if filename.to_string_lossy().contains("_uhd") {
                        image_files.push(path);
                    }
                }
            }
        }
    }

    if image_files.is_empty() {
        return Err("Nenhuma imagem UHD encontrada no diretório 'converted'.".into());
    }

    // Ordenar arquivos por nome (que inclui timestamp)
    image_files.sort();

    println!("Encontradas {} imagens UHD para o slideshow", image_files.len());
    println!("Duração por imagem: {} segundos", duration);
    println!("Arquivo de saída: {}", output_file);

    // Usar ffmpeg via linha de comando para criar o slideshow
    generate_slideshow_with_ffmpeg(&image_files, duration, output_file)?;

    println!("Slideshow gerado com sucesso!");
    Ok(())
}

fn generate_slideshow_with_ffmpeg(
    image_files: &[std::path::PathBuf], 
    duration: f64, 
    output_file: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // Criar um arquivo temporário com a lista de imagens
    let list_file = "temp_image_list.txt";
    let mut list_content = String::new();
    
    for image_path in image_files {
        list_content.push_str(&format!("file '{}'\n", image_path.display()));
        list_content.push_str(&format!("duration {}\n", duration));
    }
    
    // Adicionar a última imagem novamente sem duração para que a última frame seja exibida
    if let Some(last_image) = image_files.last() {
        list_content.push_str(&format!("file '{}'\n", last_image.display()));
    }
    
    std::fs::write(list_file, list_content)?;

    // Executar ffmpeg
    let output = ProcessCommand::new("ffmpeg")
        .arg("-y") // Sobrescrever arquivo de saída se existir
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(list_file)
        .arg("-vsync")
        .arg("vfr")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-c:v")
        .arg("libx264")
        .arg("-preset")
        .arg("medium")
        .arg("-crf")
        .arg("23")
        .arg(output_file)
        .output()?;

    // Limpar arquivo temporário
    std::fs::remove_file(list_file).ok();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Erro do ffmpeg: {}", stderr).into());
    }

    Ok(())
}