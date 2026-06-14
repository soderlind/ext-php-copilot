use std::sync::{Arc, Mutex};

use ext_php_rs::exception::PhpResult;
use ext_php_rs::prelude::*;
use github_copilot_sdk::EventSubscription;
use github_copilot_sdk::session::Session;

use crate::options::{message_options, set_model_options, to_json, to_php_error};
use crate::runtime::runtime;

#[php_class]
#[php(name = "Copilot\\Session")]
pub struct CopilotSession {
    inner: Arc<Mutex<Option<Session>>>,
    events: Arc<Mutex<Option<EventSubscription>>>,
}

#[php_impl]
impl CopilotSession {
    pub fn id(&self) -> PhpResult<String> {
        let guard = self
            .inner
            .lock()
            .map_err(|_| to_php_error("session mutex poisoned"))?;
        let session = guard
            .as_ref()
            .ok_or_else(|| to_php_error("Copilot session is disconnected"))?;

        Ok(session.id().to_string())
    }

    pub fn workspace_path(&self) -> PhpResult<Option<String>> {
        let guard = self
            .inner
            .lock()
            .map_err(|_| to_php_error("session mutex poisoned"))?;
        let session = guard
            .as_ref()
            .ok_or_else(|| to_php_error("Copilot session is disconnected"))?;

        Ok(session
            .workspace_path()
            .map(|path| path.to_string_lossy().into_owned()))
    }

    pub fn remote_url(&self) -> PhpResult<Option<String>> {
        let guard = self
            .inner
            .lock()
            .map_err(|_| to_php_error("session mutex poisoned"))?;
        let session = guard
            .as_ref()
            .ok_or_else(|| to_php_error("Copilot session is disconnected"))?;

        Ok(session.remote_url().map(ToString::to_string))
    }

    pub fn capabilities_json(&self) -> PhpResult<String> {
        let guard = self
            .inner
            .lock()
            .map_err(|_| to_php_error("session mutex poisoned"))?;
        let session = guard
            .as_ref()
            .ok_or_else(|| to_php_error("Copilot session is disconnected"))?;

        to_json(&session.capabilities())
    }

    pub fn send(&self, prompt: String, options_json: Option<String>) -> PhpResult<String> {
        let options = message_options(prompt, options_json)?;
        let guard = self
            .inner
            .lock()
            .map_err(|_| to_php_error("session mutex poisoned"))?;
        let session = guard
            .as_ref()
            .ok_or_else(|| to_php_error("Copilot session is disconnected"))?;

        runtime()
            .block_on(session.send(options))
            .map_err(to_php_error)
    }

    pub fn send_and_wait_json(
        &self,
        prompt: String,
        options_json: Option<String>,
    ) -> PhpResult<String> {
        let options = message_options(prompt, options_json)?;
        let guard = self
            .inner
            .lock()
            .map_err(|_| to_php_error("session mutex poisoned"))?;
        let session = guard
            .as_ref()
            .ok_or_else(|| to_php_error("Copilot session is disconnected"))?;
        let event = runtime()
            .block_on(session.send_and_wait(options))
            .map_err(to_php_error)?;

        to_json(&event)
    }

    pub fn messages_json(&self) -> PhpResult<String> {
        let guard = self
            .inner
            .lock()
            .map_err(|_| to_php_error("session mutex poisoned"))?;
        let session = guard
            .as_ref()
            .ok_or_else(|| to_php_error("Copilot session is disconnected"))?;
        let messages = runtime()
            .block_on(session.get_events())
            .map_err(to_php_error)?;

        to_json(&messages)
    }

    pub fn next_event_json(&self, timeout_ms: Option<u64>) -> PhpResult<Option<String>> {
        let mut guard = self
            .events
            .lock()
            .map_err(|_| to_php_error("event subscription mutex poisoned"))?;
        let events = guard
            .as_mut()
            .ok_or_else(|| to_php_error("Copilot session is disconnected"))?;
        let timeout = std::time::Duration::from_millis(timeout_ms.unwrap_or(0));

        let event = runtime()
            .block_on(async { tokio::time::timeout(timeout, events.recv()).await })
            .ok()
            .and_then(Result::ok);

        event.map(|event| to_json(&event)).transpose()
    }

    pub fn abort(&self) -> PhpResult<()> {
        let guard = self
            .inner
            .lock()
            .map_err(|_| to_php_error("session mutex poisoned"))?;
        let session = guard
            .as_ref()
            .ok_or_else(|| to_php_error("Copilot session is disconnected"))?;

        runtime().block_on(session.abort()).map_err(to_php_error)
    }

    pub fn set_model(&self, model: String, options_json: Option<String>) -> PhpResult<()> {
        let options = set_model_options(options_json)?;
        let guard = self
            .inner
            .lock()
            .map_err(|_| to_php_error("session mutex poisoned"))?;
        let session = guard
            .as_ref()
            .ok_or_else(|| to_php_error("Copilot session is disconnected"))?;

        runtime()
            .block_on(session.set_model(&model, options))
            .map_err(to_php_error)
    }

    pub fn disconnect(&self) -> PhpResult<()> {
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| to_php_error("session mutex poisoned"))?;
        let Some(session) = guard.take() else {
            return Ok(());
        };

        if let Ok(mut events) = self.events.lock() {
            events.take();
        }

        runtime()
            .block_on(session.disconnect())
            .map_err(to_php_error)
    }
}

impl CopilotSession {
    pub fn new(session: Session) -> Self {
        let events = session.subscribe();

        Self {
            inner: Arc::new(Mutex::new(Some(session))),
            events: Arc::new(Mutex::new(Some(events))),
        }
    }
}
