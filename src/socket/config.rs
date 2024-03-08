use crate::socket::Result;
use rustls::{
    client::danger,
    crypto::{ring as provider, CryptoProvider},
    pki_types::{CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
    ClientConfig, RootCertStore, ServerConfig,
};
use serde::Deserialize;
use std::{fs, io::BufReader, sync::Arc};

#[derive(Deserialize)]
pub struct Config {
    pub listen: String,
    pub certs: String,
    pub server_key: String,
    pub client_key: String,
    pub maxconnections: usize,
    pub database: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

pub fn read_config(path: &str) -> Config {
    let data = fs::read_to_string(path).unwrap();
    toml::from_str(&data).unwrap()
}

fn load_certs(filename: &str) -> Vec<CertificateDer<'static>> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .map(|result| result.unwrap())
        .collect()
}

fn load_private_key(filename: &str) -> PrivateKeyDer<'static> {
    let keyfile = fs::File::open(filename).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);

    loop {
        match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file") {
            Some(rustls_pemfile::Item::Pkcs1Key(key)) => return key.into(),
            Some(rustls_pemfile::Item::Pkcs8Key(key)) => return key.into(),
            Some(rustls_pemfile::Item::Sec1Key(key)) => return key.into(),
            None => break,
            _ => {}
        }
    }

    panic!(
        "no keys found in {:?} (encrypted keys not supported)",
        filename
    );
}

pub fn build_tls_config(certs_path: &str) -> Result<Arc<rustls::ClientConfig>> {
    let mut root_store = RootCertStore::empty();
    let certs = load_certs(certs_path);

    root_store.add_parsable_certificates(certs);

    let config = ClientConfig::builder_with_provider(
        CryptoProvider {
            cipher_suites: provider::ALL_CIPHER_SUITES.to_vec(),
            ..provider::default_provider()
        }
        .into(),
    )
    .with_protocol_versions(rustls::ALL_VERSIONS)
    .expect("inconsistent cipher-suites/versions specified")
    .with_root_certificates(root_store)
    .with_no_client_auth();

    Ok(Arc::new(config))
}