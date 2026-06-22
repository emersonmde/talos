pub struct ProbeHandler;

impl russh::server::Handler for ProbeHandler {
    type Error = russh::Error;
}

pub fn default_server_config_probe() -> russh::server::Config {
    russh::server::Config::default()
}

pub fn fail_closed_auth_probe() -> russh::server::Auth {
    russh::server::Auth::reject()
}
