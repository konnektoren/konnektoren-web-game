use gloo::storage::{LocalStorage, Storage};
use konnektoren_core::certificates::CertificateData;
use serde::{Deserialize, Serialize};

const CERTIFICATE_STORAGE_KEY: &str = "konnektoren_certificates";

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CertificateStorage {
    certificates: Vec<CertificateData>,
}

impl CertificateStorage {
    pub fn load() -> Self {
        LocalStorage::get(CERTIFICATE_STORAGE_KEY).unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        LocalStorage::set(CERTIFICATE_STORAGE_KEY, self).map_err(|e| e.to_string())
    }

    pub fn add_certificate(&mut self, certificate: CertificateData) {
        self.certificates.push(certificate);
        self.save().expect("Failed to save certificates");
    }

    pub fn get_certificates(&self) -> &Vec<CertificateData> {
        &self.certificates
    }
}
