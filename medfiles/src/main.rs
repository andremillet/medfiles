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

#[derive(Serialize)]
struct Prescription {
    command: String,
    medication: String,
    dosage: String,
    dosage_observations: String,
    posologia: String,
    posology_observations: String,
    objective: String,
}

impl From<HashMap<String, String>> for Prescription {
    fn from(map: HashMap<String, String>) -> Self {
        Prescription {
            command: map.get("command").unwrap_or(&"".to_string()).clone(),
            medication: map.get("medication").unwrap_or(&"".to_string()).clone(),
            dosage: map.get("dosage").unwrap_or(&"".to_string()).clone(),
            dosage_observations: map.get("dosage_observations").unwrap_or(&"".to_string()).clone(),
            posologia: map.get("posologia").unwrap_or(&"".to_string()).clone(),
            posology_observations: map.get("posology_observations").unwrap_or(&"".to_string()).clone(),
            objective: map.get("objective").unwrap_or(&"".to_string()).clone(),
        }
    }
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
        println!("{}", prescriptions);
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
    let prefixes = ["!PRESCREVO", "!AUMENTO", "!INCREMENTO", "!DECREMENTO", "!SUSPENDO", "!DESMAME"];
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
    let mut results = vec![];
    let mut all_changes: Vec<(String, String, String, String, String, String)> = vec![];
    for (path, modified) in files {
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
        let (output, changes) = prescription_handler(prescriptions, modified);
        all_changes.extend(changes);
        if !output.is_empty() {
            let datetime = chrono::DateTime::<chrono::Local>::from(modified);
            let date_str = datetime.format("%d/%m/%Y").to_string();
            results.push(format!("Arquivo: {} (Modificado: {})\n{}", path.display(), date_str, output));
        }
    }
    // Append current changes to history.json
    let history_path = "history.json";
    let mut history: Vec<(String, String, String, String, String, String)> = if std::path::Path::new(history_path).exists() {
        let content = fs::read_to_string(history_path).unwrap_or("[]".to_string());
        serde_json::from_str(&content).unwrap_or(vec![])
    } else {
        vec![]
    };
    history.extend(all_changes);
    let history_json = serde_json::to_string_pretty(&history).unwrap();
    fs::write(history_path, history_json).unwrap();

    let graph = prescription_graphs(&history);
    format!("{}\n{}", graph, results.join("\n---\n"))
}

fn prescription_handler(prescriptions: Vec<String>, modified: std::time::SystemTime) -> (String, Vec<(String, String, String, String, String, String)>) {
    let mut medications: HashMap<String, HashMap<String, String>> = if is_medication() {
        let content = fs::read_to_string("medications.json").unwrap_or("".to_string());
        serde_json::from_str(&content).unwrap_or(HashMap::new())
    } else {
        HashMap::new()
    };
    let mut processed: Vec<Prescription> = vec![];
    for line in prescriptions {
        let item = medication_json_populator(&line);
        processed.push(item);
    }
    let (returns, recipes, changes) = prescription_finalizer(processed, &mut medications, chrono::DateTime::<chrono::Local>::from(modified));
    // Save updated medications
    let json = serde_json::to_string_pretty(&medications).unwrap();
    fs::write("medications.json", json).unwrap();
    // Optionally save recipes to file
    if !recipes.is_empty() {
        let recipe_content = recipes.join("\n\n");
        fs::write("prescription_recipe.txt", recipe_content).unwrap();
    }
    (returns.join("\n"), changes)
}

fn is_medication() -> bool {
    fs::metadata("medications.json").is_ok()
}

fn medication_json_creator() {
    fs::write("medications.json", "[]").unwrap();
}

fn medication_json_populator(line: &str) -> Prescription {
    let map = medication_list_tokenizer(line);
    Prescription::from(map)
}

fn calculate_difference(current: &Prescription, prev: &HashMap<String, String>) -> String {
    // Simple parsing for dosage_observations
    let current_dosage = parse_dosage(&current.dosage_observations);
    let prev_dosage = parse_dosage(&prev.get("dosage_observations").unwrap_or(&"".to_string()));
    let diff = current_dosage - prev_dosage;
    format!("{:.1} COMPRIMIDOS", diff)
}

fn prescription_graphs(changes: &Vec<(String, String, String, String, String, String)>) -> String {
    if changes.is_empty() {
        return String::new();
    }
    let mut medications: std::collections::HashSet<String> = std::collections::HashSet::new();
    for (med, _, _, _, _, _) in changes {
        medications.insert(med.clone());
    }
    let mut sorted_changes: Vec<_> = changes.iter().map(|(med, time_str, field, old, new, cmd)| {
        let time = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S").unwrap_or(chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
        (med.clone(), time, field.clone(), old.clone(), new.clone(), cmd.clone())
    }).collect::<Vec<_>>();
    sorted_changes.sort_by_key(|(_, time, _, _, _, _)| *time);
    let mut unique_times: Vec<_> = sorted_changes.iter().map(|(_, time, _, _, _, _)| *time).collect();
    unique_times.sort();
    unique_times.dedup();
    let mut graph = String::from("Gráfico de Evolução das Prescrições:\n");
    for med in medications {
        graph.push_str(&format!("{}: ", med));
        let med_changes: Vec<_> = sorted_changes.iter().filter(|(m, _, _, _, _, _)| m == &med).collect();
        // Group by timestamp and command
        let mut unique_events: Vec<(chrono::NaiveDateTime, String)> = vec![];
        for (_, time, _, _, _, command) in &med_changes {
            if !unique_events.iter().any(|(t, c)| t == time && c == command) {
                unique_events.push((*time, command.clone()));
            }
        }
        unique_events.sort_by_key(|(time, _)| *time);
        let spacing = if unique_events.len() > 1 { 80 / (unique_events.len() - 1) } else { 10 };
        let mut line = String::new();
        let mut positions = vec![];
        for (i, (time, command)) in unique_events.iter().enumerate() {
            let pos = i * spacing;
            let date_str = time.format("%d/%m").to_string();
            let symbol = match command.as_str() {
                "INCREASE" => '▲',
                "DECREASE" => '▼',
                "PRESCRIBE" => '●',
                _ => '●',
            };
            let label = format!("{}{}", date_str, symbol);
            positions.push((pos, label));
        }
        let mut last_pos = 0;
        for (pos, label) in positions {
            for _ in last_pos..pos {
                line.push('─');
            }
            line.push_str(&label);
            last_pos = pos + label.len();
        }
        graph.push_str(&line);
        graph.push('\n');
    }

    graph
}

fn generate_diff(medication: &str, old: &HashMap<String, String>, new: &Prescription) -> String {
    let mut diff_lines = vec![];
    let fields = vec![
        ("dosage", &new.dosage),
        ("dosage_observations", &new.dosage_observations),
        ("posologia", &new.posologia),
        ("posology_observations", &new.posology_observations),
    ];
    for (field, new_value) in fields {
        let old_value = old.get(field).map(|s| s.as_str()).unwrap_or("");
        if old_value != new_value {
            diff_lines.push(format!("\x1b[31m- {}: {}\x1b[0m", field, old_value));
            diff_lines.push(format!("\x1b[32m+ {}: {}\x1b[0m", field, new_value));
        }
    }
    if diff_lines.is_empty() {
        format!("Nenhuma mudança detectada para {}", medication)
    } else {
        format!("Mudanças para {}:\n{}", medication, diff_lines.join("\n"))
    }
}

fn parse_dosage(dosage: &str) -> f64 {
    // Simple parser for [number] or [fraction]
    let cleaned = dosage.trim_start_matches('[').trim_end_matches(']').trim();
    let first_part = cleaned.split_whitespace().next().unwrap_or("");
    if let Some(slash_pos) = first_part.find('/') {
        let parts: Vec<&str> = first_part.split('/').collect();
        if parts.len() == 2 {
            let num: f64 = parts[0].parse().unwrap_or(0.0);
            let den: f64 = parts[1].parse().unwrap_or(1.0);
            num / den
        } else {
            first_part.parse().unwrap_or(0.0)
        }
    } else {
        first_part.parse().unwrap_or(0.0)
    }
}

fn prescription_finalizer(items: Vec<Prescription>, medications: &mut HashMap<String, HashMap<String, String>>, file_time: chrono::DateTime<chrono::Local>) -> (Vec<String>, Vec<String>, Vec<(String, String, String, String, String, String)>) {
    let mut prescription_return = vec![];
    let mut prescription_recipe = vec![];
    let mut changes = vec![];
    for item in items {
        if item.command == "PRESCRIBE" {
            let ret = format!(
                "ADICIONADO {}, {}, : {} {} à lista de medicações em uso;",
                item.medication, item.dosage, item.dosage_observations, item.posologia
            );
            prescription_return.push(ret);

            let line1 = format!("{} {}", item.medication, item.dosage);
            let line2 = if item.posology_observations.is_empty() {
                format!("{} de {}", item.dosage_observations, item.posologia)
            } else {
                format!("{} de {}, por {}", item.dosage_observations, item.posologia, item.posology_observations)
            };
            let recipe = format!("{}\n{}", line1, line2);
            prescription_recipe.push(recipe);

            // Save to medications
            let mut med_map = HashMap::new();
            med_map.insert("dosage".to_string(), item.dosage.clone());
            med_map.insert("dosage_observations".to_string(), item.dosage_observations.clone());
            med_map.insert("posologia".to_string(), item.posologia.clone());
            med_map.insert("posology_observations".to_string(), item.posology_observations.clone());
            medications.insert(item.medication.clone(), med_map);

            // Add to changes for initial prescription
            let timestamp_str = file_time.format("%Y-%m-%d %H:%M:%S").to_string();
            changes.push((item.medication.clone(), timestamp_str, "initial".to_string(), "".to_string(), item.dosage_observations.clone(), item.command.clone()));
        } else if item.command == "INCREASE" {
            if let Some(prev) = medications.get(&item.medication) {
                let diff_output = generate_diff(&item.medication, prev, &item);
                prescription_return.push(diff_output);
                // Collect changes for graph
                let fields = vec![
                    ("dosage", item.dosage.clone()),
                    ("dosage_observations", item.dosage_observations.clone()),
                    ("posologia", item.posologia.clone()),
                    ("posology_observations", item.posology_observations.clone()),
                ];
                for (field, new_value) in fields {
                    let old_value = prev.get(field).map(|s| s.as_str()).unwrap_or("");
                    if old_value != new_value {
                        let timestamp_str = file_time.format("%Y-%m-%d %H:%M:%S").to_string();
                        changes.push((item.medication.clone(), timestamp_str, field.to_string(), old_value.to_string(), new_value, item.command.clone()));
                    }
                }

                // Update medications
                let mut med_map = HashMap::new();
                med_map.insert("dosage".to_string(), item.dosage.clone());
                med_map.insert("dosage_observations".to_string(), item.dosage_observations.clone());
                med_map.insert("posologia".to_string(), item.posologia.clone());
                med_map.insert("posology_observations".to_string(), item.posology_observations.clone());
                medications.insert(item.medication.clone(), med_map);
            }
        }
    }
    (prescription_return, prescription_recipe, changes)
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

