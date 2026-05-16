use std::sync::{Arc, Mutex};

use ext_php_rs::exception::PhpResult;
use ext_php_rs::prelude::*;
use github_copilot_sdk::Client;

use crate::options::{
    client_options_from_json, resume_config_from_json, session_config_from_json, to_json,
    to_php_error,
};
use crate::runtime::runtime;
use crate::session::CopilotSession;

#[php_class]
#[php(name = "Copilot\\Client")]
pub struct CopilotClient {
    inner: Arc<Mutex<Option<Client>>>,
}

#[php_impl]
impl CopilotClient {
    pub fn __construct(options_json: Option<String>) -> PhpResult<Self> {
        let options = client_options_from_json(options_json)?;
        let client = runtime()
            .block_on(Client::start(options))
            .map_err(to_php_error)?;

        Ok(Self {
            inner: Arc::new(Mutex::new(Some(client))),
        })
    }

    pub fn ping(&self, message: Option<String>) -> PhpResult<String> {
        let client = self.client()?;
        let response = runtime()
            .block_on(client.ping(message.as_deref()))
            .map_err(to_php_error)?;

        to_json(&response)
    }

    pub fn models_json(&self) -> PhpResult<String> {
        let client = self.client()?;
        let models = runtime()
            .block_on(client.list_models())
            .map_err(to_php_error)?;

        to_json(&models)
    }

    pub fn status_json(&self) -> PhpResult<String> {
        let client = self.client()?;
        let status = runtime()
            .block_on(client.get_status())
            .map_err(to_php_error)?;

        to_json(&status)
    }

    pub fn auth_status_json(&self) -> PhpResult<String> {
        let client = self.client()?;
        let status = runtime()
            .block_on(client.get_auth_status())
            .map_err(to_php_error)?;

        to_json(&status)
    }

    pub fn call_json(&self, method: String, params_json: Option<String>) -> PhpResult<String> {
        let params = match params_json {
            Some(raw) if !raw.trim().is_empty() => {
                Some(serde_json::from_str(&raw).map_err(to_php_error)?)
            }
            _ => None,
        };
        let client = self.client()?;
        let response = runtime()
            .block_on(client.call(&method, params))
            .map_err(to_php_error)?;

        to_json(&response)
    }

    pub fn create_session(&self, config_json: Option<String>) -> PhpResult<CopilotSession> {
        let config = session_config_from_json(config_json)?;
        let client = self.client()?;
        let session = runtime()
            .block_on(client.create_session(config))
            .map_err(to_php_error)?;

        Ok(CopilotSession::new(session))
    }

    pub fn resume_session(
        &self,
        session_id: String,
        config_json: Option<String>,
    ) -> PhpResult<CopilotSession> {
        let config = resume_config_from_json(session_id, config_json)?;
        let client = self.client()?;
        let session = runtime()
            .block_on(client.resume_session(config))
            .map_err(to_php_error)?;

        Ok(CopilotSession::new(session))
    }

    pub fn stop(&self) -> PhpResult<()> {
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| to_php_error("client mutex poisoned"))?;
        let Some(client) = guard.take() else {
            return Ok(());
        };

        runtime().block_on(client.stop()).map_err(to_php_error)
    }
}

impl CopilotClient {
    fn client(&self) -> PhpResult<Client> {
        self.inner
            .lock()
            .map_err(|_| to_php_error("client mutex poisoned"))?
            .as_ref()
            .cloned()
            .ok_or_else(|| to_php_error("Copilot client is stopped"))
    }
}
