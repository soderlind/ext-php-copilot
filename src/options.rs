use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

use ext_php_rs::exception::{PhpException, PhpResult};
use github_copilot_sdk::types::{
    Attachment, CustomAgentConfig, DefaultAgentConfig, DeliveryMode, InfiniteSessionConfig,
    MessageOptions, ProviderConfig, ResumeSessionConfig, SessionConfig, SessionId, SetModelOptions,
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
        options = options.with_base_directory(PathBuf::from(home));
    }
    if let Some(remote) = bool_at(&value, "remote") {
        options = options.with_enable_remote_sessions(remote);
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
    let mut config = session_config_from_value(&value)?;

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
    let value = parse_json_option(input)?;

    let permission_policy =
        string_at(&value, "permissionPolicy").or_else(|| string_at(&value, "permission_policy"));
    let mut config = resume_config_from_value(SessionId::from(session_id), &value)?;

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
    let connection_token = string_at(value, "connectionToken")
        .or_else(|| string_at(value, "connection_token"))
        .or_else(|| {
            string_at(value, "tcpConnectionToken")
                .or_else(|| string_at(value, "tcp_connection_token"))
        });
    match kind.as_str() {
        "stdio" => Ok(Transport::Stdio),
        "tcp" => Ok(Transport::Tcp {
            port: u16::try_from(u64_at(value, "port").unwrap_or(0)).map_err(to_php_error)?,
            connection_token,
        }),
        "external" => Ok(Transport::External {
            host: string_at(value, "host").unwrap_or_else(|| "127.0.0.1".to_string()),
            port: u16::try_from(u64_at(value, "port").unwrap_or(0)).map_err(to_php_error)?,
            connection_token,
        }),
        other => Err(to_php_error(format!("unsupported transport type: {other}"))),
    }
}

fn session_config_from_value(value: &Value) -> PhpResult<SessionConfig> {
    let mut config = SessionConfig::default();
    apply_session_config(value, &mut config)?;
    Ok(config)
}

fn resume_config_from_value(
    session_id: SessionId,
    value: &Value,
) -> PhpResult<ResumeSessionConfig> {
    let mut config = ResumeSessionConfig::new(session_id);

    if let Some(client_name) = string_at(value, "clientName") {
        config = config.with_client_name(client_name);
    }
    if let Some(reasoning_effort) = string_at(value, "reasoningEffort") {
        config = config.with_reasoning_effort(reasoning_effort);
    }
    if let Some(context_tier) = string_at(value, "contextTier") {
        config = config.with_context_tier(context_tier);
    }
    if let Some(streaming) = bool_at(value, "streaming") {
        config = config.with_streaming(streaming);
    }
    if let Some(system_message) = value.get("systemMessage") {
        config = config.with_system_message(from_value(system_message)?);
    }
    if let Some(tools) = string_vec_at(value, "availableTools") {
        config = config.with_available_tools(tools);
    }
    if let Some(tools) = string_vec_at(value, "excludedTools") {
        config = config.with_excluded_tools(tools);
    }
    if let Some(servers) = value.get("mcpServers") {
        config = config.with_mcp_servers(from_value(servers)?);
    }
    if let Some(enabled) = bool_at(value, "enableConfigDiscovery") {
        config = config.with_enable_config_discovery(enabled);
    }
    if let Some(paths) = string_vec_at(value, "skillDirectories") {
        config = config.with_skill_directories(paths.into_iter().map(PathBuf::from));
    }
    if let Some(paths) = string_vec_at(value, "instructionDirectories") {
        config = config.with_instruction_directories(paths.into_iter().map(PathBuf::from));
    }
    if let Some(skills) = string_vec_at(value, "disabledSkills") {
        config = config.with_disabled_skills(skills);
    }
    if let Some(agents) = value.get("customAgents") {
        config = config.with_custom_agents(from_value::<Vec<CustomAgentConfig>>(agents)?);
    }
    if let Some(agent) = value.get("defaultAgent") {
        config = config.with_default_agent(from_value::<DefaultAgentConfig>(agent)?);
    }
    if let Some(agent) = string_at(value, "agent") {
        config = config.with_agent(agent);
    }
    if let Some(sessions) = value.get("infiniteSessions") {
        config = config.with_infinite_sessions(from_value::<InfiniteSessionConfig>(sessions)?);
    }
    if let Some(provider) = value.get("provider") {
        config = config.with_provider(from_value::<ProviderConfig>(provider)?);
    }
    if let Some(enabled) = bool_at(value, "enableSessionTelemetry") {
        config = config.with_enable_session_telemetry(enabled);
    }
    if let Some(config_dir) = string_at(value, "configDir") {
        config = config.with_config_directory(PathBuf::from(config_dir));
    }
    if let Some(working_directory) = string_at(value, "workingDirectory") {
        config = config.with_working_directory(PathBuf::from(working_directory));
    }
    if let Some(token) = string_at(value, "gitHubToken").or_else(|| string_at(value, "githubToken"))
    {
        config = config.with_github_token(token);
    }
    if let Some(include) = bool_at(value, "includeSubAgentStreamingEvents") {
        config = config.with_include_sub_agent_streaming_events(include);
    }

    Ok(config)
}

fn apply_session_config(value: &Value, config: &mut SessionConfig) -> PhpResult<()> {
    if let Some(session_id) = string_at(value, "sessionId") {
        config.session_id = Some(SessionId::from(session_id));
    }
    if let Some(model) = string_at(value, "model") {
        *config = std::mem::take(config).with_model(model);
    }
    if let Some(client_name) = string_at(value, "clientName") {
        *config = std::mem::take(config).with_client_name(client_name);
    }
    if let Some(reasoning_effort) = string_at(value, "reasoningEffort") {
        *config = std::mem::take(config).with_reasoning_effort(reasoning_effort);
    }
    if let Some(context_tier) = string_at(value, "contextTier") {
        *config = std::mem::take(config).with_context_tier(context_tier);
    }
    if let Some(streaming) = bool_at(value, "streaming") {
        *config = std::mem::take(config).with_streaming(streaming);
    }
    if let Some(system_message) = value.get("systemMessage") {
        *config = std::mem::take(config).with_system_message(from_value(system_message)?);
    }
    if let Some(tools) = string_vec_at(value, "availableTools") {
        *config = std::mem::take(config).with_available_tools(tools);
    }
    if let Some(tools) = string_vec_at(value, "excludedTools") {
        *config = std::mem::take(config).with_excluded_tools(tools);
    }
    if let Some(servers) = value.get("mcpServers") {
        *config = std::mem::take(config).with_mcp_servers(from_value(servers)?);
    }
    if let Some(enabled) = bool_at(value, "enableConfigDiscovery") {
        *config = std::mem::take(config).with_enable_config_discovery(enabled);
    }
    if let Some(paths) = string_vec_at(value, "skillDirectories") {
        *config =
            std::mem::take(config).with_skill_directories(paths.into_iter().map(PathBuf::from));
    }
    if let Some(paths) = string_vec_at(value, "instructionDirectories") {
        *config = std::mem::take(config)
            .with_instruction_directories(paths.into_iter().map(PathBuf::from));
    }
    if let Some(skills) = string_vec_at(value, "disabledSkills") {
        *config = std::mem::take(config).with_disabled_skills(skills);
    }
    if let Some(agents) = value.get("customAgents") {
        *config = std::mem::take(config)
            .with_custom_agents(from_value::<Vec<CustomAgentConfig>>(agents)?);
    }
    if let Some(agent) = value.get("defaultAgent") {
        *config =
            std::mem::take(config).with_default_agent(from_value::<DefaultAgentConfig>(agent)?);
    }
    if let Some(agent) = string_at(value, "agent") {
        *config = std::mem::take(config).with_agent(agent);
    }
    if let Some(sessions) = value.get("infiniteSessions") {
        *config = std::mem::take(config)
            .with_infinite_sessions(from_value::<InfiniteSessionConfig>(sessions)?);
    }
    if let Some(provider) = value.get("provider") {
        *config = std::mem::take(config).with_provider(from_value::<ProviderConfig>(provider)?);
    }
    if let Some(enabled) = bool_at(value, "enableSessionTelemetry") {
        *config = std::mem::take(config).with_enable_session_telemetry(enabled);
    }
    if let Some(config_dir) = string_at(value, "configDir") {
        *config = std::mem::take(config).with_config_directory(PathBuf::from(config_dir));
    }
    if let Some(working_directory) = string_at(value, "workingDirectory") {
        *config = std::mem::take(config).with_working_directory(PathBuf::from(working_directory));
    }
    if let Some(token) = string_at(value, "gitHubToken").or_else(|| string_at(value, "githubToken"))
    {
        *config = std::mem::take(config).with_github_token(token);
    }
    if let Some(include) = bool_at(value, "includeSubAgentStreamingEvents") {
        *config = std::mem::take(config).with_include_sub_agent_streaming_events(include);
    }

    Ok(())
}

fn from_value<T: serde::de::DeserializeOwned>(value: &Value) -> PhpResult<T> {
    serde_json::from_value(value.clone()).map_err(to_php_error)
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

        assert_eq!(options.working_directory, PathBuf::from("/tmp"));
        assert_eq!(options.extra_args, vec!["--quiet".to_string()]);
        assert!(matches!(options.transport, Transport::Tcp { port: 0, .. }));
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
