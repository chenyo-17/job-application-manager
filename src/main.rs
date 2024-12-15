use dialoguer::{theme::ColorfulTheme, Input};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use toml;

#[derive(Serialize, Deserialize)]
struct Config {
    applications_path: String,
    system_engineer_template_path: String,
    software_engineer_template_path: String,
}

#[derive(Serialize)]
struct Application {
    company: String,
    role: String,
    url: String,
    date: String,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = home_dir().unwrap().join(".application-manager/config.toml");
    let config_str = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

fn copy_folder(src: &Path, dst: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let target = dst.join(path.strip_prefix(src).unwrap());
        if path.is_dir() {
            fs::create_dir_all(&target)?;
            copy_folder(&path, &target)?;
        } else {
            fs::copy(&path, &target)?;
        }
    }
    Ok(())
}

fn record_application(
    config: &Config,
    app: &Application,
) -> Result<(), Box<dyn std::error::Error>> {
    let application_csv = PathBuf::from(&config.applications_path).join("applications.csv");
    let file_exists = std::path::Path::new(&application_csv).exists();
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&application_csv)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(!file_exists)
        .from_writer(file);
    writer.serialize(app)?;
    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = load_config()?;
    let theme = ColorfulTheme::default();

    let company: String = Input::with_theme(&theme)
        .with_prompt("Company name")
        .interact()?;
    let role: String = Input::with_theme(&theme)
        .with_prompt("Job role")
        .interact()?;
    let role_type: String = Input::with_theme(&theme)
        .with_prompt("Role type (software/system)")
        .validate_with(|input: &String| match input.to_lowercase().as_str() {
            "software" | "system" => Ok(()),
            _ => Err("Enter 'software' or 'system'"),
        })
        .interact()?;
    let url: String = Input::with_theme(&theme)
        .with_prompt("Job description URL")
        .interact()?;

    let job_folder = PathBuf::from(&config.applications_path).join(format!("{}", company));
    fs::create_dir_all(&job_folder)?;

    fs::write(job_folder.join("job-description.txt"), "")?;
    let template_path = match role_type.to_lowercase().as_str() {
        "software" => &config.software_engineer_template_path,
        "system" => &config.system_engineer_template_path,
        _ => unreachable!(),
    };
    copy_folder(Path::new(&template_path), &job_folder)?;

    let app = Application {
        company,
        role,
        url,
        date: chrono::Local::now().format("%Y-%m-%d").to_string(),
    };
    record_application(&config, &app)?;
    println!("Application recorded at {}", job_folder.display());

    Ok(())
}
