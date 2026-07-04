const VAULT_FILE_PATH: &str = "/Users/adam/Documents/PMGR/VAULT.json";

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct VaultEntry {
    pub service: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Vault {
    entries: Vec<VaultEntry>,
}

pub enum VaultResult {
    Found(VaultEntry),
    Added(VaultEntry),
    Updated(VaultEntry),
    Deleted(VaultEntry),
    NotFound(String),
    AlreadyExists(String),
}

pub fn get_entry_data(service: &str) -> VaultResult {

    let vault = read_vault();
    let result = vault.entries.into_iter().find(|e| e.service == service);

    match result {
        Some(entry) => VaultResult::Found(entry),
        None => VaultResult::NotFound(service.to_string())
    }
}

pub fn add_entry_data(service: String, username: String, password: String) -> VaultResult {
    let mut vault = read_vault();

    if vault.entries.iter().any(|e| e.service == service) {
        return VaultResult::AlreadyExists(service.to_string());
    }

    let new_entry = VaultEntry {
        service,
        username: Some(username),
        password: Some(password),
    };

    let cloned_entry = new_entry.clone();
    vault.entries.push(new_entry);
    update_json_file(&vault);

    VaultResult::Added(cloned_entry)
}

pub fn update_entry_data(service: String, username: String, password: String) -> VaultResult {
    let mut vault = read_vault();

    if vault.entries.iter().all(|e| e.service != service) {
        return VaultResult::NotFound(service.to_string());
    }
    let entry = vault.entries.iter_mut().find(|e| e.service == service).unwrap();

    entry.username = Some(username);
    entry.password = Some(password);

    let cloned_entry = entry.clone();
    update_json_file(&vault);

    VaultResult::Updated(cloned_entry)
}

pub fn delete_entry_data(service: String) -> VaultResult {
    let mut vault = read_vault();

    if vault.entries.iter().all(|e| e.service != service) {
        return VaultResult::NotFound(service.to_string());
    }

    let index = vault.entries.iter().position(|e| e.service == service).unwrap();
    let deleted_entry = vault.entries.remove(index);

    update_json_file(&vault);

    VaultResult::Deleted(deleted_entry)
}

fn read_vault() -> Vault {
    let contents = std::fs::read_to_string(VAULT_FILE_PATH)
        .expect("Failed to read VAULT.json");
    serde_json::from_str(&contents).expect("Failed to parse VAULT.json")
}

fn update_json_file(vault: &Vault) {
    let updated_contents = serde_json::to_string_pretty(vault).expect("Failed to serialize VAULT.json");
    std::fs::write(VAULT_FILE_PATH, updated_contents).expect("Failed to write VAULT.json");
}