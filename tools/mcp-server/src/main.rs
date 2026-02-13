use ligen_mcp_server::Registry;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    AnnotateAble, Implementation, InitializeRequestParams, InitializeResult, ListResourcesResult,
    PaginatedRequestParams, ProtocolVersion, RawResource, ReadResourceRequestParams,
    ReadResourceResult, ResourceContents, ServerCapabilities, ServerInfo,
};
use rmcp::service::RequestContext;
use rmcp::transport::stdio;
use rmcp::{ErrorData, tool, tool_handler, tool_router};
use rmcp::{RoleServer, ServerHandler, ServiceExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;

#[derive(Deserialize, Serialize, JsonSchema)]
#[schemars(crate = "rmcp::schemars")]
pub struct AddProjectArgs {
    pub path: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[schemars(crate = "rmcp::schemars")]
pub struct RemoveProjectArgs {
    pub name: String,
}

#[derive(Clone)]
pub struct McpServer {
    pub registry: Arc<Mutex<Registry>>,
    pub tool_router: ToolRouter<Self>,
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(Registry::new())),
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl McpServer {
    #[tool(description = "Add a project to the registry by its filesystem path")]
    async fn add_project(&self, Parameters(args): Parameters<AddProjectArgs>) -> String {
        let mut registry = self.registry.lock().await;
        match registry.add_project(args.path) {
            Ok(name) => format!("Project '{}' added successfully", name),
            Err(e) => format!("Failed to add project: {}", e),
        }
    }

    #[tool(description = "Remove a project from the registry")]
    async fn remove_project(&self, Parameters(args): Parameters<RemoveProjectArgs>) -> String {
        let mut registry = self.registry.lock().await;
        if registry.remove_project(&args.name) {
            format!("Project '{}' removed successfully", args.name)
        } else {
            format!("Project '{}' not found", args.name)
        }
    }

    #[tool(description = "List all registered projects")]
    async fn list_projects(&self) -> String {
        let registry = self.registry.lock().await;
        let projects = registry.list_projects();
        if projects.is_empty() {
            "No projects registered".to_string()
        } else {
            format!("Registered projects: {}", projects.join(", "))
        }
    }
}

#[tool_handler]
impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Ligen MCP Server for project and IDL management".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, ErrorData> {
        Ok(self.get_info())
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        let registry = self.registry.lock().await;
        let mut resources = Vec::new();
        for (name, _project) in registry.projects.iter() {
            let mut raw =
                RawResource::new(format!("ligen://{}/idl", name), format!("{} IDL", name));
            raw.description = Some(format!("IDL documentation for project {}", name));
            raw.mime_type = Some("application/json".to_string());
            resources.push(raw.no_annotation());
        }
        Ok(ListResourcesResult::with_all_items(resources))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        let registry = self.registry.lock().await;
        let uri = &request.uri;

        if let Some(part) = uri.strip_prefix("ligen://")
            && let Some(slash_idx) = part.find('/')
        {
            let project_name = &part[..slash_idx];
            let path = &part[slash_idx..];

            if path == "/idl"
                && let Some(project) = registry.get_project(project_name)
                && let Ok(idl_json) = serde_json::to_string_pretty(&project.library)
            {
                return Ok(ReadResourceResult {
                    contents: vec![ResourceContents::TextResourceContents {
                        uri: uri.clone(),
                        mime_type: Some("application/json".to_string()),
                        text: idl_json,
                        meta: None,
                    }],
                });
            }
        }

        Ok(ReadResourceResult { contents: vec![] })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting ligen-mcp-server");

    let server = McpServer::new();
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
