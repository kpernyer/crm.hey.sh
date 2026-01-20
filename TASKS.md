# CRM.HEY.SH Deployment & MCP Integration Tasks

## Overview
This document outlines the tasks required to deploy the CRM.HEY.SH server online and make it accessible via MCP (Model Context Protocol) from Claude Desktop.

## Current Status
- ✅ MCP Server implementation exists in `backend/mcp-server/`
- ✅ MCP server supports stdio transport (for Claude Desktop)
- ⚠️ HTTP transport is NOT implemented (as seen in `main.rs`)
- ✅ Multiple CRM tools already implemented (search_contacts, get_contact_details, etc.)
- ✅ Database connection to SurrealDB works
- ⚠️ No production deployment configuration exists yet

## Target Architecture
```
┌─────────────────────────────────────────────────────────────────────┐
│                         CLOUD INFRASTRUCTURE                        │
├─────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐ │
│  │ MCP Server      │  │ CRM Backend     │  │ SurrealDB           │ │
│  │ (port 3001)     │  │ (port 8080)     │  │ (port 8000)         │ │
│  │ (MCP over HTTP) │  │ (REST API)      │  │ (Database)          │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────────┘ │
│         │                       │                       │           │
│         ▼                       ▼                       ▼           │
│  Claude Desktop           Web Clients            Persistence       │
└─────────────────────────────────────────────────────────────────────┘
```

## Phase 1: Implement Missing HTTP Transport for MCP

### Task 1.1: Implement HTTP+SSE Transport in MCP Server
**Status:** Not Started
**Priority:** Critical
**Owner:** Backend Team

**Description:** The MCP server currently only supports stdio transport but needs HTTP+SSE for online deployment and Claude Desktop HTTP access.

**Steps:**
1. Add axum and related dependencies to `backend/mcp-server/Cargo.toml`
2. Implement HTTP transport in `main.rs` (replace `Err(McpError::NotImplemented)` in `run_http_transport`)
3. Create endpoints:
   - `POST /mcp/messages` - for tool calls
   - `GET /mcp/sse` - for server-sent events
4. Implement MCP protocol over HTTP
5. Add proper error handling for HTTP transport
6. Add tests for HTTP transport

**Files to modify:**
- `backend/mcp-server/Cargo.toml`
- `backend/mcp-server/src/main.rs`
- `backend/mcp-server/src/handlers.rs` (potentially)

**Success criteria:**
- MCP server can run in HTTP mode
- HTTP endpoints properly handle MCP JSON-RPC requests
- SSE streams work for Claude Desktop integration

### Task 1.2: Add Authentication to MCP Server
**Status:** Not Started
**Priority:** High
**Owner:** Security Team

**Description:** Add authentication to protect MCP endpoints when deployed online.

**Steps:**
1. Add API key authentication middleware
2. Add token-based authentication option
3. Implement rate limiting
4. Add audit logging for all MCP tool calls
5. Add environment variables for auth config

**Files to modify:**
- `backend/mcp-server/src/main.rs`
- `backend/mcp-server/src/config.rs`
- Add new authentication module if needed

**Success criteria:**
- MCP endpoints require authentication
- API keys can be configured via environment
- Rate limiting is implemented
- All requests are logged for audit

## Phase 2: Containerization and Deployment Configuration

### Task 2.1: Create Docker Image for MCP Server
**Status:** Not Started
**Priority:** High
**Owner:** DevOps Team

**Description:** Create Dockerfile for MCP server to enable containerized deployment.

**Steps:**
1. Create `backend/mcp-server/Dockerfile`
2. Multi-stage build for optimized image size
3. Include proper environment configuration
4. Add health checks
5. Create docker-compose override for MCP server

**Files to create:**
- `backend/mcp-server/Dockerfile`
- `docker-compose.mcp.yml` or extend existing `docker-compose.yml`

**Success criteria:**
- MCP server builds in a Docker container
- Container image is pushed to registry
- MCP server can run standalone in container

### Task 2.2: Update Infrastructure Configuration
**Status:** Not Started
**Priority:** High
**Owner:** DevOps Team

**Description:** Update Kubernetes and Terraform configurations to deploy MCP server.

**Steps:**
1. Add MCP server deployment to `infra/k8s/mcp-server.yaml`
2. Add MCP service and ingress configuration
3. Update Terraform to provision MCP server resources
4. Configure load balancing for MCP endpoints
5. Set up SSL/TLS certificates for MCP endpoints
6. Add monitoring and alerting for MCP server

**Files to create/modify:**
- `infra/k8s/mcp-server.yaml`
- `infra/k8s/mcp-service.yaml`
- `infra/k8s/mcp-ingress.yaml`
- `infra/terraform/mcp.tf`
- `infra/terraform/variables.tf` (add MCP variables)

**Success criteria:**
- MCP server can be deployed to Kubernetes
- MCP endpoints are accessible via HTTPS
- MCP server is monitored and logged

## Phase 3: Online Deployment and Public Access

### Task 3.1: Deploy MCP Server to Production
**Status:** Not Started
**Priority:** Critical
**Owner:** DevOps Team

**Description:** Deploy the MCP server to the production environment.

**Steps:**
1. Set up staging environment with MCP server
2. Test MCP server functionality in staging
3. Deploy MCP server to production
4. Configure DNS for MCP endpoint
5. Set up CDN if needed for performance

**Success criteria:**
- MCP server is running in production
- MCP endpoint is publicly accessible
- MCP server can connect to production database

### Task 3.2: Configure Claude Desktop MCP Integration
**Status:** Not Started
**Priority:** Critical
**Owner:** Integration Team

**Description:** Configure Claude Desktop to connect to the deployed MCP server.

**Steps:**
1. Create MCP configuration for Claude Desktop
2. Document connection instructions for users
3. Test MCP integration with Claude Desktop
4. Create troubleshooting documentation
5. Set up connection health monitoring

**Example Claude Desktop configuration:**
```json
{
  "mcpServers": {
    "crm": {
      "url": "https://mcp.crm.hey.sh",
      "headers": {
        "Authorization": "Bearer YOUR_API_KEY"
      }
    }
  }
}
```

**Success criteria:**
- Claude Desktop can connect to MCP server
- All CRM tools are accessible from Claude Desktop
- Integration is stable and performs well

## Phase 4: Security and Production Readiness

### Task 4.1: Security Hardening
**Status:** Not Started
**Priority:** High
**Owner:** Security Team

**Description:** Implement security measures for online MCP server.

**Steps:**
1. Add request validation and sanitization
2. Implement proper CORS configuration
3. Add security headers
4. Set up DDoS protection
5. Implement proper logging and monitoring
6. Conduct security review

**Success criteria:**
- MCP server follows security best practices
- All inputs are properly validated
- Security incidents can be monitored and responded to

### Task 4.2: Performance Optimization
**Status:** Not Started
**Priority:** Medium
**Owner:** Backend Team

**Description:** Optimize MCP server performance for production usage.

**Steps:**
1. Add connection pooling for database
2. Implement caching for frequently accessed data
3. Optimize database queries used by MCP tools
4. Add performance monitoring
5. Conduct load testing
6. Set up auto-scaling

**Success criteria:**
- MCP server performs well under load
- Response times are within acceptable limits
- Server can scale based on demand

## Phase 5: User Documentation and Support

### Task 5.1: Create MCP Usage Documentation
**Status:** Not Started
**Priority:** Medium
**Owner:** Documentation Team

**Description:** Create comprehensive documentation for users.

**Steps:**
1. Document all MCP tools and their parameters
2. Create Claude Desktop integration guide
3. Provide examples of common workflows
4. Create troubleshooting guide
5. Document API limits and rate limits

**Files to create:**
- `docs/MCP-USAGE.md`
- `docs/MCP-TROUBLESHOOTING.md`
- `examples/mcp-workflows.md`

**Success criteria:**
- Users can easily integrate with Claude Desktop
- All tools are well documented with examples
- Users can troubleshoot common issues

### Task 5.2: Create Monitoring Dashboard
**Status:** Not Started
**Priority:** Medium
**Owner:** DevOps Team

**Description:** Set up monitoring for MCP server health and usage.

**Steps:**
1. Add metrics collection to MCP server
2. Set up monitoring dashboard
3. Configure alerts for critical issues
4. Track usage patterns and performance
5. Monitor integration health

**Success criteria:**
- MCP server health is monitored
- Usage patterns are tracked
- Critical issues trigger alerts

## Priority Task List (Immediate Actions)

1. **[CRITICAL]** Implement HTTP transport in MCP server
2. **[HIGH]** Create Docker image for MCP server
3. **[HIGH]** Add authentication to MCP server
4. **[HIGH]** Deploy MCP server to staging for testing
5. **[CRITICAL]** Test MCP integration with Claude Desktop

## Timeline Estimate

- **Week 1-2:** Implement HTTP transport and authentication
- **Week 2-3:** Containerization and staging deployment
- **Week 3-4:** Production deployment and Claude Desktop integration
- **Week 4+:** Performance optimization and monitoring

## Dependencies

- MCP server HTTP transport implementation (Task 1.1) blocks all other tasks
- Database connectivity and SSL certificates required for deployment
- Claude Desktop supports HTTP MCP servers (verify compatibility)

## Risk Assessment

- **High Risk:** HTTP transport not implemented - prevents online deployment
- **Medium Risk:** Security vulnerabilities in MCP endpoints
- **Medium Risk:** Performance issues under load
- **Low Risk:** Claude Desktop compatibility issues (workarounds possible)

## Next Steps

1. Assign team members to critical tasks (especially HTTP transport implementation)
2. Set up development environment for MCP server development
3. Begin implementation of HTTP transport functionality
4. Schedule regular check-ins to track progress on MCP deployment