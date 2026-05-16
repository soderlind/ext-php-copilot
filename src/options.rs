use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

use ext_php_rs::exception::{PhpException, PhpResult};
use github_copilot_sdk::types::{
    Attachment, DeliveryMode, MessageOptions, ResumeSessionConfig, SessionConfig, SetModelOptions,
};
use github_copilot_sdk::{
    CliProgram, ClientOptions, LogLevel, OtelExporterType, TelemetryConfig, Transport,
};
use serde_json::Value;

pub fn parse_json_option(input: Option<String>) -> PhpResult<Value> {
    match input {
        Some(raw) if !raw.trim().is_empty() => serde_json::from_str(&raw).map_err(to_php_error),
        _ => Ok(Value::Object(Default::default())),
    }
}

pub fn to_json<T: serde::Serialize>(value: &T) -> PhpResult<String> {
    serde_json::to_string(value).map_err(to_php_error)
}

pub fn to_php_error(error: impl std::fmt::Display) -> PhpException {
    PhpException::default(error.to_string())
}

pub fn client_options_from_json(input: Option<String>) -> PhpResult<ClientOptions> {
    let value = parse_json_option(input)?;
    let mut options = ClientOptions::new();

    if let Some(path) =
        string_at(&value, "programPath").or_else(|| string_at(&value, "program_path"))
    {
        options = options.with_program(CliProgram::Path(PathBuf::from(path)));
    }
    if let Some(cwd) = string_at(&value, "cwd") {
        options = options.with_cwd(PathBuf::from(cwd));
    }
    if let Some(args) =
        string_vec_at(&value, "prefixArgs").or_else(|| string_vec_at(&value, "prefix_args"))
    {
        options = options.with_prefix_args(args.into_iter().map(OsString::from));
    }
    if let Some(args) =
        string_vec_at(&value, "extraArgs").or_else(|| string_vec_at(&value, "extra_args"))
    {
        options = options.with_extra_args(args);
    }
    if let Some(env) = string_map_at(&value, "env") {
        options = options.with_env(
            env.into_iter()
                .map(|(key, value)| (OsString::from(key), OsString::from(value))),
        );
    }
    if let Some(names) =
        string_vec_at(&value, "envRemove").or_else(|| string_vec_at(&value, "env_remove"))
    {
        options = options.with_env_remove(names.into_iter().map(OsString::from));
    }
    if let Some(token) =
        string_at(&value, "githubToken").or_else(|| string_at(&value, "github_token"))
    {
        options = options.with_github_token(token);
    }
    if let Some(use_logged_in) =
        bool_at(&value, "useLoggedInUser").or_else(|| bool_at(&value, "use_logged_in_user"))
    {
        options = options.with_use_logged_in_user(use_logged_in);
    }
    if let Some(level) = string_at(&value, "logLevel").or_else(|| string_at(&value, "log_level")) {
        options = options.with_log_level(parse_client_log_level(&level)?);
    }
    if let Some(seconds) = u64_at(&value, "sessionIdleTimeoutSeconds")
        .or_else(|| u64_at(&value, "session_idle_timeout_seconds"))
    {
        options = options.with_session_idle_timeout_seconds(seconds);
    }
    if let Some(home) =
        string_at(&value, "copilotHome").or_else(|| string_at(&value, "copilot_home"))
    {
        options = options.with_copilot_home(PathBuf::from(home));
    }
    if let Some(token) = string_at(&value, "tcpConnectionToken")
        .or_else(|| string_at(&value, "tcp_connection_token"))
    {
        options = options.with_tcp_connection_token(token);
    }
    if let Some(remote) = bool_at(&value, "remote") {
        options = options.with_remote(remote);
    }
    if let Some(transport) = value.get("transport") {
        options = options.with_transport(parse_transport(transport)?);
    }
    if let Some(telemetry) = value.get("telemetry")
        && let Some(config) = parse_telemetry(telemetry)?
    {
        options = options.with_telemetry(config);
    }

    Ok(options)
}

pub fn session_config_from_json(input: Option<String>) -> PhpResult<SessionConfig> {
    let value = parse_json_option(input)?;
    let permission_policy =
        string_at(&value, "permissionPolicy").or_else(|| string_at(&value, "permission_policy"));
    let mut config: SessionConfig = serde_json::from_value(value).map_err(to_php_error)?;

    config = match permission_policy.as_deref() {
        Some("approve_all") | Some("approveAll") => config.approve_all_permissions(),
        Some("deny_all") | Some("denyAll") | None => config.deny_all_permissions(),
        Some(other) => {
            return Err(to_php_error(format!(
                "unsupported permission policy: {other}; expected deny_all or approve_all"
            )));
        }
    };

    Ok(config)
}

pub fn resume_config_from_json(
    session_id: String,
    input: Option<String>,
) -> PhpResult<ResumeSessionConfig> {
    let mut value = parse_json_option(input)?;
    if let Value::Object(ref mut object) = value {
        object.insert("sessionId".to_string(), Value::String(session_id));
    }

    let permission_policy =
        string_at(&value, "permissionPolicy").or_else(|| string_at(&value, "permission_policy"));
    let mut config: ResumeSessionConfig = serde_json::from_value(value).map_err(to_php_error)?;

    config = match permission_policy.as_deref() {
        Some("approve_all") | Some("approveAll") => config.approve_all_permissions(),
        Some("deny_all") | Some("denyAll") | None => config.deny_all_permissions(),
        Some(other) => {
            return Err(to_php_error(format!(
                "unsupported permission policy: {other}; expected deny_all or approve_all"
            )));
        }
    };

    Ok(config)
}

pub fn message_options(prompt: String, input: Option<String>) -> PhpResult<MessageOptions> {
    let value = parse_json_option(input)?;
    let mut options = MessageOptions::new(prompt);

    if let Some(mode) = string_at(&value, "mode") {
        let mode: DeliveryMode =
            serde_json::from_value(Value::String(mode)).map_err(to_php_error)?;
        options = options.with_mode(mode);
    }
    if let Some(timeout) =
        u64_at(&value, "timeoutSeconds").or_else(|| u64_at(&value, "timeout_seconds"))
    {
        options = options.with_wait_timeout(Duration::from_secs(timeout));
    }
    if let Some(timeout) = u64_at(&value, "timeoutMs").or_else(|| u64_at(&value, "timeout_ms")) {
        options = options.with_wait_timeout(Duration::from_millis(timeout));
    }
    if let Some(attachments) = value.get("attachments") {
        let attachments: Vec<Attachment> =
            serde_json::from_value(attachments.clone()).map_err(to_php_error)?;
        options = options.with_attachments(attachments);
    }
    if let Some(headers) =
        string_map_at(&value, "requestHeaders").or_else(|| string_map_at(&value, "request_headers"))
    {
        options = options.with_request_headers(headers);
    }
    if let Some(traceparent) = string_at(&value, "traceparent") {
        options = options.with_traceparent(traceparent);
    }
    if let Some(tracestate) = string_at(&value, "tracestate") {
        options = options.with_tracestate(tracestate);
    }

    Ok(options)
}

pub fn set_model_options(input: Option<String>) -> PhpResult<Option<SetModelOptions>> {
    let value = parse_json_option(input)?;
    let mut options = SetModelOptions::default();
    let mut configured = false;

    if let Some(reasoning_effort) =
        string_at(&value, "reasoningEffort").or_else(|| string_at(&value, "reasoning_effort"))
    {
        options = options.with_reasoning_effort(reasoning_effort);
        configured = true;
    }

    if configured {
        Ok(Some(options))
    } else {
        Ok(None)
    }
}

fn parse_transport(value: &Value) -> PhpResult<Transport> {
    let kind = string_at(value, "type").unwrap_or_else(|| "stdio".to_string());
    match kind.as_str() {
        "stdio" => Ok(Transport::Stdio),
        "tcp" => Ok(Transport::Tcp {
            port: u16::try_from(u64_at(value, "port").unwrap_or(0)).map_err(to_php_error)?,
        }),
        "external" => Ok(Transport::External {
            host: string_at(value, "host").unwrap_or_else(|| "127.0.0.1".to_string()),
            port: u16::try_from(u64_at(value, "port").unwrap_or(0)).map_err(to_php_error)?,
        }),
        other => Err(to_php_error(format!("unsupported transport type: {other}"))),
    }
}

fn parse_telemetry(value: &Value) -> PhpResult<Option<TelemetryConfig>> {
    if !value.is_object() {
        return Err(to_php_error("telemetry must be an object"));
    }

    let mut config = TelemetryConfig::new();
    let mut configured = false;

    if let Some(endpoint) =
        string_at(value, "otlpEndpoint").or_else(|| string_at(value, "otlp_endpoint"))
    {
        config = config.with_otlp_endpoint(endpoint);
        configured = true;
    }
    if let Some(file_path) = string_at(value, "filePath").or_else(|| string_at(value, "file_path"))
    {
        config = config.with_file_path(PathBuf::from(file_path));
        configured = true;
    }
    if let Some(source_name) =
        string_at(value, "sourceName").or_else(|| string_at(value, "source_name"))
    {
        config = config.with_source_name(source_name);
        configured = true;
    }
    if let Some(capture) =
        bool_at(value, "captureContent").or_else(|| bool_at(value, "capture_content"))
    {
        config = config.with_capture_content(capture);
        configured = true;
    }
    if let Some(exporter) =
        string_at(value, "exporterType").or_else(|| string_at(value, "exporter_type"))
    {
        let exporter = match exporter.as_str() {
            "otlp-http" | "otlpHttp" => OtelExporterType::OtlpHttp,
            "file" => OtelExporterType::File,
            other => {
                return Err(to_php_error(format!(
                    "unsupported telemetry exporter: {other}"
                )));
            }
        };
        config = config.with_exporter_type(exporter);
        configured = true;
    }

    Ok(configured.then_some(config))
}

fn parse_client_log_level(level: &str) -> PhpResult<LogLevel> {
    match level {
        "none" => Ok(LogLevel::None),
        "error" => Ok(LogLevel::Error),
        "warning" | "warn" => Ok(LogLevel::Warning),
        "info" => Ok(LogLevel::Info),
        "debug" => Ok(LogLevel::Debug),
        "all" | "trace" => Ok(LogLevel::All),
        other => Err(to_php_error(format!("unsupported log level: {other}"))),
    }
}

fn string_at(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn bool_at(value: &Value, key: &str) -> Option<bool> {
    value.get(key).and_then(Value::as_bool)
}

fn u64_at(value: &Value, key: &str) -> Option<u64> {
    value.get(key).and_then(Value::as_u64)
}

fn string_vec_at(value: &Value, key: &str) -> Option<Vec<String>> {
    value.get(key).and_then(Value::as_array).map(|items| {
        items
            .iter()
            .filter_map(Value::as_str)
            .map(ToString::to_string)
            .collect()
    })
}

fn string_map_at(value: &Value, key: &str) -> Option<HashMap<String, String>> {
    value.get(key).and_then(Value::as_object).map(|object| {
        object
            .iter()
            .filter_map(|(key, value)| value.as_str().map(|value| (key.clone(), value.to_string())))
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use github_copilot_sdk::types::SessionId;

    #[test]
    fn parses_client_options() {
        let options = client_options_from_json(Some(
            r#"{"cwd":"/tmp","extraArgs":["--quiet"],"transport":{"type":"tcp","port":0},"logLevel":"debug"}"#.to_string(),
        ))
        .unwrap();

        assert_eq!(options.cwd, PathBuf::from("/tmp"));
        assert_eq!(options.extra_args, vec!["--quiet".to_string()]);
        assert!(matches!(options.transport, Transport::Tcp { port: 0 }));
        assert_eq!(options.log_level, Some(LogLevel::Debug));
    }

    #[test]
    fn parses_message_options() {
        let options = message_options(
            "hello".to_string(),
            Some(r#"{"mode":"immediate","timeoutMs":250}"#.to_string()),
        )
        .unwrap();

        assert_eq!(options.prompt, "hello");
        assert_eq!(options.mode, Some(DeliveryMode::Immediate));
        assert_eq!(options.wait_timeout, Some(Duration::from_millis(250)));
    }

    #[test]
    fn rejects_unknown_permission_policy() {
        assert!(
            session_config_from_json(Some(r#"{"permissionPolicy":"later"}"#.to_string())).is_err()
        );
    }

    #[test]
    fn injects_resume_session_id() {
        let config = resume_config_from_json("session-1".to_string(), None).unwrap();

        assert_eq!(config.session_id, SessionId::from("session-1"));
    }
}
