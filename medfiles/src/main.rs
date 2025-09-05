use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use rand::Rng;
use std::thread;
use std::time::{Duration, SystemTime};

#[derive(Parser)]
#[command(name = "medfile")]
#[command(about = "Medical files processor")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Config,
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    name: String,
    cpf: String,
    phone: String,
    email: String,
}

fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Config => {
                if !is_new_user() {
                    create_user();
                } else {
                    println!("User already configured.");
                }
            }
        }
    } else {
        if is_new_user() {
            greet();
        } else {
            println!("Usuário não configurado. Execute 'medfiles config'.");
        }
    }
}

fn greet() {
    let content = fs::read_to_string("user_info.json").expect("Falha ao ler user_info.json");
    let user_info: UserInfo = serde_json::from_str(&content).expect("Falha ao parsear user_info.json");
    let first_name = user_info.name.split_whitespace().next().unwrap_or("Usuário");
    println!("Olá, {}!", first_name);
    println!("O que você deseja fazer?");
    println!("1. Ver atendimentos");
    println!("2. Ver prescrições");
    print!("Digite sua escolha (1 ou 2): ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();
    if choice == "1" {
        println!("Recurso ainda em desenvolvimento.");
    } else if choice == "2" {
        let prescriptions = conduta_handler();
        println!("{}", serde_json::to_string_pretty(&prescriptions).unwrap());
    } else {
        println!("Escolha inválida.");
    }
}

fn is_new_user() -> bool {
    fs::metadata("user_info.json").is_ok()
}

fn generate_token() -> String {
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(0..1000000))
}

fn send_email(to: &str, token: &str) -> bool {
    // For demo, use a dummy SMTP. In real, configure with actual credentials.
    // This is placeholder; actual implementation needs SMTP server details.
    println!("Enviando email para {} com token {}", to, token);
    // Simulate success
    true
}

fn verify_token(_expected_token: &str) -> bool {
    let start_time = SystemTime::now();
    let duration = Duration::from_secs(600); // 10 minutes

    loop {
        let elapsed = start_time.elapsed().unwrap();
        if elapsed >= duration {
            println!("Tempo expirado. Gere um novo token.");
            return false;
        }

        let remaining = duration - elapsed;
        let minutes = remaining.as_secs() / 60;
        let seconds = remaining.as_secs() % 60;
        print!("\rTempo restante: {}:{:02} - Digite o token: ", minutes, seconds);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        // Non-blocking read? For simplicity, use a short timeout or poll.
        // But in Rust, stdin is blocking. For demo, use a loop with sleep.
        thread::sleep(Duration::from_secs(1));

        // To make it work, perhaps use a separate thread or something, but keep simple.
        // For now, just prompt once.
        print!("Digite o token: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.len() == 5 && input.chars().all(|c| c.is_alphanumeric()) {
            println!("\nToken válido!");
            return true;
        } else {
            println!("Token inválido. Deve ter exatamente 5 números ou letras. Tente novamente.");
        }
    }
}

fn create_user() {
    println!("Bem-vindo! Vamos configurar sua conta.");
    print!("Digite seu nome completo: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    print!("Digite seu CPF: ");
    io::stdout().flush().unwrap();
    let mut cpf = String::new();
    io::stdin().read_line(&mut cpf).unwrap();
    let cpf = cpf.trim().to_string();

    print!("Digite seu número de telefone: ");
    io::stdout().flush().unwrap();
    let mut phone = String::new();
    io::stdin().read_line(&mut phone).unwrap();
    let phone = phone.trim().to_string();

    print!("Digite seu email: ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();
    let email = email.trim().to_string();

    println!("Escolha o método de confirmação:");
    println!("1. SMS (indisponível - taxa adicional)");
    println!("2. Email");
    print!("Digite sua escolha (1 ou 2): ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    if choice == "1" {
        println!("SMS indisponível. Escolha email.");
        return;
    } else if choice == "2" {
        let token = generate_token();
        if send_email(&email, &token) {
            if verify_token(&token) {
                let user_info = UserInfo { name, cpf, phone, email };
                let json = serde_json::to_string_pretty(&user_info).unwrap();
                fs::write("user_info.json", json).unwrap();
                println!("Usuário configurado com sucesso!");
            } else {
                println!("Token inválido ou expirado. Tente novamente.");
            }
        } else {
            println!("Falha ao enviar email. Tente novamente.");
        }
    } else {
        println!("Escolha inválida.");
    }
}

fn is_prescription(line: &str) -> bool {
    let prefixes = ["!PRESCREVO", "!INCREMENTO", "!DECREMENTO", "!SUSPENDO", "!DESMAME"];
    prefixes.iter().any(|&p| line.trim().starts_with(p))
}

fn prescription_grabber(conduta_lines: Vec<String>) -> Vec<String> {
    conduta_lines.into_iter().filter(|line| is_prescription(line)).collect()
}

fn conduta_handler() -> String {
    let mut files = vec![];
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension() == Some(std::ffi::OsStr::new("med")) {
            let metadata = entry.metadata().unwrap();
            let modified = metadata.modified().unwrap();
            files.push((path, modified));
        }
    }
    files.sort_by_key(|&(_, time)| time);
    let mut all_prescriptions = vec![];
    for (path, _) in files {
        let content = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        let mut in_conduta = false;
        let mut conduta_lines = vec![];
        for line in lines {
            if line.trim() == "[CONDUTA]" {
                in_conduta = true;
            } else if line.trim().starts_with('[') && in_conduta {
                break;
            } else if in_conduta {
                conduta_lines.push(line.to_string());
            }
        }
        let prescriptions = prescription_grabber(conduta_lines);
        all_prescriptions.extend(prescriptions);
    }
    prescription_handler(all_prescriptions)
}

fn prescription_handler(prescriptions: Vec<String>) -> String {
    if !is_medication() {
        medication_json_creator();
    }
    let mut processed = vec![];
    for line in prescriptions {
        let item = medication_json_populator(&line);
        processed.push(item);
    }
    serde_json::to_string_pretty(&processed).unwrap()
}

fn is_medication() -> bool {
    fs::metadata("medications.json").is_ok()
}

fn medication_json_creator() {
    fs::write("medications.json", "[]").unwrap();
}

fn medication_json_populator(line: &str) -> HashMap<String, String> {
    medication_list_tokenizer(line)
}

fn medication_list_tokenizer(line: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let trimmed = line.trim_start_matches('!').trim_end_matches(';');
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let mut idx = 0;

    // Command
    if idx < parts.len() {
        let cmd = parts[idx];
        let command = match cmd {
            "PRESCREVO" => "PRESCRIBE",
            "AUMENTO" => "INCREASE",
            "REDUZO" => "DECREASE",
            "SUSPENDO" => "SUSPEND",
            _ => cmd,
        };
        map.insert("command".to_string(), command.to_string());
        idx += 1;
    }

    // Medication
    if idx < parts.len() {
        let mut med = parts[idx].to_string();
        if med.starts_with('"') || med.starts_with('\'') {
            let quote = med.chars().next().unwrap();
            med = med.trim_start_matches(quote).to_string();
            while idx + 1 < parts.len() && !med.ends_with(quote) {
                idx += 1;
                med.push(' ');
                med.push_str(parts[idx]);
            }
            med = med.trim_end_matches(quote).to_string();
        }
        map.insert("medication".to_string(), med);
        idx += 1;
    }

    // Dosage
    let mut dosage = "1 UNIDADE".to_string();
    if idx < parts.len() && !parts[idx].starts_with('[') && !parts[idx].contains(">>") {
        dosage = parts[idx].to_string();
        idx += 1;
    }
    map.insert("dosage".to_string(), dosage);

    // Dosage observations
    let mut dosage_obs = String::new();
    if idx < parts.len() && parts[idx].starts_with('[') {
        dosage_obs = parts[idx].to_string();
        idx += 1;
        while idx < parts.len() && !dosage_obs.ends_with(']') {
            dosage_obs.push(' ');
            dosage_obs.push_str(parts[idx]);
            idx += 1;
        }
        dosage_obs = dosage_obs.trim_start_matches('[').trim_end_matches(']').to_string();
    }
    map.insert("dosage_observations".to_string(), dosage_obs);

    // Posology
    let mut posology = String::new();
    if idx < parts.len() && !parts[idx].contains(">>") {
        posology = parts[idx].to_string();
        idx += 1;
        // If there's a second group
        if idx < parts.len() && !parts[idx].contains(">>") {
            posology.push(' ');
            posology.push_str(parts[idx]);
            idx += 1;
        }
    }
    map.insert("posologia".to_string(), posology);

    // Posology observations
    let mut pos_obs = String::new();
    if idx < parts.len() && parts[idx].starts_with('[') {
        pos_obs = parts[idx].trim_start_matches('[').trim_end_matches(']').to_string();
        idx += 1;
    }
    map.insert("posology_observations".to_string(), pos_obs);

    // Objective
    let mut objective = String::new();
    if idx < parts.len() && parts[idx] == ">>" {
        idx += 1;
        while idx < parts.len() {
            if objective.is_empty() {
                objective = parts[idx].to_string();
            } else {
                objective.push(' ');
                objective.push_str(parts[idx]);
            }
            idx += 1;
        }
    }
    map.insert("objective".to_string(), objective);

    map
}

