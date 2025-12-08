# CRM.HEY.SH - Module Ownership Plan

A systematic approach to taking ownership of the codebase, working middle-out.

## Philosophy: Middle-Out Approach

```
                    ┌─────────────────┐
                    │    Frontend     │  ← Phase 4
                    │   (React/RN)    │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │    API Layer    │  ← Phase 2
                    │  (Handlers/REST)│
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
┌───────▼───────┐   ┌────────▼────────┐   ┌───────▼───────┐
│   Services    │   │  Domain Models  │   │  AI Services  │  ← Phase 1 (START HERE)
│ (Business     │   │  (Core Types)   │   │  (Stubs →     │
│  Logic)       │   │                 │   │   Real)       │
└───────┬───────┘   └────────┬────────┘   └───────┬───────┘
        │                    │                    │
        └────────────────────┼────────────────────┘
                             │
                    ┌────────▼────────┐
                    │    Storage      │  ← Phase 3
                    │   (SurrealDB)   │
                    │ Indexes/Graphs  │
                    │ Vector Search   │
                    └─────────────────┘
```

---

## Phase 1: Core Domain & Testing Foundation

**Goal**: Own the domain models, business logic, and establish testing patterns.

### Week 1-2: Domain Models & Unit Tests

#### 1.1 Contact Domain
```
backend/src/models/contact.rs
backend/src/services/contact_service.rs (new)
backend/tests/unit/contact_test.rs (new)
```

**Tasks**:
- [ ] Add validation logic to Contact model (email format, required fields)
- [ ] Create `ContactService` with pure business logic
- [ ] Implement engagement score calculation algorithm
- [ ] Write unit tests for all validation rules

**Unit Test Examples**:
```rust
// backend/tests/unit/contact_test.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(Contact::validate_email("user@example.com").is_ok());
        assert!(Contact::validate_email("invalid").is_err());
    }

    #[test]
    fn test_engagement_score_bounds() {
        let score = calculate_engagement_score(&timeline_entries);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_status_transitions() {
        // Lead -> Customer is valid
        // Customer -> Lead should warn/require confirmation
    }
}
```

#### 1.2 Campaign Domain
```
backend/src/models/campaign.rs
backend/src/services/campaign_service.rs (new)
backend/tests/unit/campaign_test.rs (new)
```

**Tasks**:
- [ ] Define campaign state machine (draft → scheduled → running → completed)
- [ ] Implement segment query builder with validation
- [ ] Create asset generation orchestration logic
- [ ] Write property tests for segment builder

**Property Test Example**:
```rust
// Using proptest
use proptest::prelude::*;

proptest! {
    #[test]
    fn segment_query_never_panics(
        filters in prop::collection::vec(any::<SegmentFilter>(), 0..10)
    ) {
        let definition = SegmentDefinition { filters, logic: LogicOperator::And };
        // Should never panic, always produce valid SurrealQL or error
        let _ = SegmentBuilder::build_query(&definition);
    }

    #[test]
    fn campaign_status_transitions_are_valid(
        from in any::<CampaignStatus>(),
        to in any::<CampaignStatus>()
    ) {
        let result = CampaignStateMachine::can_transition(from, to);
        // Verify state machine rules
    }
}
```

#### 1.3 Timeline & Events Domain
```
backend/src/services/timeline_service.rs (new)
backend/src/services/event_service.rs (new)
```

**Tasks**:
- [ ] Implement timeline aggregation logic
- [ ] Build event capacity and RSVP management
- [ ] Create notification triggers (who gets notified when)

---

### Week 3-4: Integration Tests

#### 1.4 API Integration Tests
```
backend/tests/integration/
├── mod.rs
├── contacts_api_test.rs
├── campaigns_api_test.rs
├── events_api_test.rs
└── helpers.rs
```

**Test Setup Pattern**:
```rust
// backend/tests/integration/helpers.rs
use testcontainers::{clients::Cli, images::surrealdb::SurrealDb};

pub struct TestApp {
    pub client: reqwest::Client,
    pub base_url: String,
    pub db: TestDatabase,
}

impl TestApp {
    pub async fn spawn() -> Self {
        // Spin up SurrealDB in container
        // Start app on random port
        // Return configured test client
    }

    pub async fn create_test_contact(&self) -> Contact {
        // Helper to seed test data
    }
}

// Usage in tests
#[tokio::test]
async fn test_create_and_retrieve_contact() {
    let app = TestApp::spawn().await;

    let contact = app.client
        .post(&format!("{}/api/contacts", app.base_url))
        .json(&json!({
            "first_name": "Test",
            "last_name": "User",
            "email": "test@example.com"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(contact.status(), 201);

    let created: ContactResponse = contact.json().await.unwrap();
    assert_eq!(created.email, "test@example.com");

    // Verify retrieval
    let fetched = app.client
        .get(&format!("{}/api/contacts/{}", app.base_url, created.id))
        .send()
        .await
        .unwrap();

    assert_eq!(fetched.status(), 200);
}
```

#### 1.5 Service Integration Tests
```rust
// Test services with real database
#[tokio::test]
async fn test_campaign_execution_creates_timeline_entries() {
    let app = TestApp::spawn().await;

    // Create contacts
    let contacts = app.seed_contacts(10).await;

    // Create campaign targeting those contacts
    let campaign = app.create_campaign_with_segment(
        SegmentDefinition::all_leads()
    ).await;

    // Execute campaign
    app.execute_campaign(campaign.id).await;

    // Verify timeline entries created for each contact
    for contact in contacts {
        let timeline = app.get_contact_timeline(contact.id).await;
        assert!(timeline.iter().any(|e| e.entry_type == "email_sent"));
    }
}
```

---

## Phase 2: API Layer Ownership

**Goal**: Own every handler, understand request/response flows, add middleware.

### Week 5-6: Handler Refinement

#### 2.1 Add Request Validation Middleware
```
backend/src/middleware/
├── mod.rs
├── validation.rs
├── auth.rs
└── rate_limit.rs
```

**Tasks**:
- [ ] Implement request body validation with detailed errors
- [ ] Add authentication middleware (JWT verification)
- [ ] Implement rate limiting per endpoint
- [ ] Add request/response logging

#### 2.2 Improve Error Responses
```rust
// Structured error responses
#[derive(Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<Vec<FieldError>>,
    pub request_id: String,
}

#[derive(Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}
```

#### 2.3 Add OpenAPI Documentation
```
backend/src/docs/
├── mod.rs
└── openapi.rs
```

Use `utoipa` to generate OpenAPI spec from code.

---

## Phase 3: Storage Layer Mastery

**Goal**: Optimize SurrealDB usage with proper indexes, graph relations, and vector search.

### Week 7-8: Index Optimization

#### 3.1 Analyze Query Patterns
```surql
-- Add to backend/schema/indexes.surql

-- Contact queries
DEFINE INDEX contact_email ON TABLE contact COLUMNS email UNIQUE;
DEFINE INDEX contact_status ON TABLE contact COLUMNS status;
DEFINE INDEX contact_company ON TABLE contact COLUMNS company;
DEFINE INDEX contact_engagement ON TABLE contact COLUMNS engagement_score;
DEFINE INDEX contact_created ON TABLE contact COLUMNS created_at;

-- Composite indexes for common filters
DEFINE INDEX contact_status_engagement ON TABLE contact COLUMNS status, engagement_score;
DEFINE INDEX contact_company_status ON TABLE contact COLUMNS company, status;

-- Full-text search
DEFINE INDEX contact_name_search ON TABLE contact
  COLUMNS first_name, last_name
  SEARCH ANALYZER ascii BM25;

-- Timeline queries (most frequent)
DEFINE INDEX timeline_contact_time ON TABLE timeline_entry
  COLUMNS contact, timestamp DESC;
DEFINE INDEX timeline_type_time ON TABLE timeline_entry
  COLUMNS type, timestamp DESC;
```

#### 3.2 Graph Relations
```surql
-- backend/schema/graphs.surql

-- Define graph edges for relationships
DEFINE TABLE works_at SCHEMAFULL;
DEFINE FIELD in ON TABLE works_at TYPE record<contact>;
DEFINE FIELD out ON TABLE works_at TYPE record<company>;
DEFINE FIELD role ON TABLE works_at TYPE string;
DEFINE FIELD started ON TABLE works_at TYPE datetime;

-- Query: Find all contacts at a company
-- SELECT <-works_at<-contact FROM company:xyz

-- Define campaign targeting relation
DEFINE TABLE targeted_by SCHEMAFULL;
DEFINE FIELD in ON TABLE targeted_by TYPE record<contact>;
DEFINE FIELD out ON TABLE targeted_by TYPE record<campaign>;
DEFINE FIELD sent_at ON TABLE targeted_by TYPE datetime;
DEFINE FIELD opened ON TABLE targeted_by TYPE bool DEFAULT false;
DEFINE FIELD clicked ON TABLE targeted_by TYPE bool DEFAULT false;

-- Query: Find all campaigns a contact was part of
-- SELECT ->targeted_by->campaign FROM contact:abc

-- Query: Find contacts who opened campaign emails
-- SELECT <-targeted_by[WHERE opened = true]<-contact FROM campaign:xyz
```

#### 3.3 Graph Traversal Queries
```rust
// backend/src/services/graph_service.rs

impl GraphService {
    /// Find contacts within N degrees of a given contact
    pub async fn find_connections(&self, contact_id: &str, depth: u8) -> Vec<Contact> {
        let query = format!(r#"
            SELECT ->works_at->company<-works_at<-contact
            FROM contact:{}
            WHERE $this != contact:{}
        "#, contact_id, contact_id);
        // ...
    }

    /// Find campaign performance path
    pub async fn campaign_funnel(&self, campaign_id: &str) -> FunnelMetrics {
        let query = r#"
            LET $targeted = SELECT <-targeted_by<-contact FROM campaign:$id;
            LET $opened = SELECT <-targeted_by[WHERE opened=true]<-contact FROM campaign:$id;
            LET $clicked = SELECT <-targeted_by[WHERE clicked=true]<-contact FROM campaign:$id;
            RETURN {
                targeted: count($targeted),
                opened: count($opened),
                clicked: count($clicked)
            }
        "#;
        // ...
    }
}
```

### Week 9-10: Vector/Semantic Search

#### 3.4 Add Embeddings for Semantic Search
```surql
-- backend/schema/vectors.surql

-- Add embedding field to contacts for semantic search
DEFINE FIELD embedding ON TABLE contact TYPE array<float>;
DEFINE INDEX contact_embedding ON TABLE contact COLUMNS embedding MTREE DIMENSION 1536;

-- Add embedding to timeline entries for semantic search
DEFINE FIELD embedding ON TABLE timeline_entry TYPE array<float>;
DEFINE INDEX timeline_embedding ON TABLE timeline_entry COLUMNS embedding MTREE DIMENSION 1536;

-- Campaign content embeddings
DEFINE FIELD content_embedding ON TABLE campaign_asset TYPE array<float>;
DEFINE INDEX asset_embedding ON TABLE campaign_asset COLUMNS content_embedding MTREE DIMENSION 1536;
```

#### 3.5 Embedding Service
```rust
// backend/src/services/embedding_service.rs

pub struct EmbeddingService {
    client: OpenAIClient, // or local model
}

impl EmbeddingService {
    pub async fn embed_text(&self, text: &str) -> Vec<f32> {
        // Call embedding API or local model
    }

    pub async fn embed_contact(&self, contact: &Contact) -> Vec<f32> {
        let text = format!(
            "{} {} - {} - Tags: {}",
            contact.first_name,
            contact.last_name,
            contact.email,
            contact.tags.join(", ")
        );
        self.embed_text(&text).await
    }

    pub async fn semantic_search_contacts(
        &self,
        query: &str,
        limit: usize
    ) -> Vec<(Contact, f32)> {
        let query_embedding = self.embed_text(query).await;

        let results = self.db.query(r#"
            SELECT *, vector::similarity::cosine(embedding, $embedding) AS score
            FROM contact
            WHERE embedding != NONE
            ORDER BY score DESC
            LIMIT $limit
        "#)
        .bind(("embedding", query_embedding))
        .bind(("limit", limit))
        .await?;

        // Return contacts with similarity scores
    }

    pub async fn find_similar_contacts(&self, contact_id: &str, limit: usize) -> Vec<Contact> {
        let results = self.db.query(r#"
            LET $source = SELECT embedding FROM contact:$id;
            SELECT *, vector::similarity::cosine(embedding, $source.embedding) AS score
            FROM contact
            WHERE id != contact:$id AND embedding != NONE
            ORDER BY score DESC
            LIMIT $limit
        "#)
        .bind(("id", contact_id))
        .bind(("limit", limit))
        .await?;
        // ...
    }
}
```

#### 3.6 Hybrid Search (Text + Vector)
```rust
// Combine BM25 text search with vector similarity
pub async fn hybrid_search(
    &self,
    query: &str,
    filters: Option<ContactFilters>,
) -> Vec<ScoredContact> {
    let embedding = self.embed_text(query).await;

    let results = self.db.query(r#"
        -- Text search score
        LET $text_results = SELECT
            id,
            search::score(1) AS text_score
        FROM contact
        WHERE first_name @1@ $query OR last_name @1@ $query;

        -- Vector search score
        LET $vector_results = SELECT
            id,
            vector::similarity::cosine(embedding, $embedding) AS vector_score
        FROM contact
        WHERE embedding != NONE;

        -- Combine scores (RRF - Reciprocal Rank Fusion)
        SELECT
            contact.*,
            (1 / (60 + $text_rank)) + (1 / (60 + $vector_rank)) AS combined_score
        FROM (
            -- Join and rank
        )
        ORDER BY combined_score DESC
        LIMIT $limit
    "#)
    .bind(("query", query))
    .bind(("embedding", embedding))
    .await?;

    // ...
}
```

---

## Phase 4: Frontend Ownership

**Goal**: Implement all features with smooth UX, proper state management, optimistic updates.

### Week 11-12: State Management & Data Flow

#### 4.1 Query Patterns with React Query
```typescript
// frontend/src/lib/queries/contacts.ts

export const contactKeys = {
  all: ['contacts'] as const,
  lists: () => [...contactKeys.all, 'list'] as const,
  list: (filters: ContactFilters) => [...contactKeys.lists(), filters] as const,
  details: () => [...contactKeys.all, 'detail'] as const,
  detail: (id: string) => [...contactKeys.details(), id] as const,
  timeline: (id: string) => [...contactKeys.detail(id), 'timeline'] as const,
};

export function useContacts(filters: ContactFilters) {
  return useQuery({
    queryKey: contactKeys.list(filters),
    queryFn: () => api.contacts.list(filters),
    staleTime: 30_000,
  });
}

export function useContact(id: string) {
  return useQuery({
    queryKey: contactKeys.detail(id),
    queryFn: () => api.contacts.get(id),
  });
}

export function useContactTimeline(id: string) {
  return useInfiniteQuery({
    queryKey: contactKeys.timeline(id),
    queryFn: ({ pageParam = 0 }) =>
      api.contacts.timeline(id, { offset: pageParam, limit: 20 }),
    getNextPageParam: (lastPage, pages) =>
      lastPage.length === 20 ? pages.length * 20 : undefined,
  });
}
```

#### 4.2 Optimistic Updates
```typescript
// frontend/src/lib/mutations/contacts.ts

export function useUpdateContact() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: Partial<Contact> }) =>
      api.contacts.update(id, data),

    onMutate: async ({ id, data }) => {
      // Cancel outgoing refetches
      await queryClient.cancelQueries({ queryKey: contactKeys.detail(id) });

      // Snapshot previous value
      const previous = queryClient.getQueryData(contactKeys.detail(id));

      // Optimistically update
      queryClient.setQueryData(contactKeys.detail(id), (old: Contact) => ({
        ...old,
        ...data,
        updated_at: new Date().toISOString(),
      }));

      return { previous };
    },

    onError: (err, { id }, context) => {
      // Rollback on error
      queryClient.setQueryData(contactKeys.detail(id), context?.previous);
      toast.error('Failed to update contact');
    },

    onSettled: (_, __, { id }) => {
      // Refetch to ensure consistency
      queryClient.invalidateQueries({ queryKey: contactKeys.detail(id) });
    },
  });
}
```

#### 4.3 Form Handling
```typescript
// frontend/src/components/contacts/contact-form.tsx

import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';

const contactSchema = z.object({
  first_name: z.string().min(1, 'First name is required'),
  last_name: z.string().min(1, 'Last name is required'),
  email: z.string().email('Invalid email address'),
  phone: z.string().optional(),
  status: z.enum(['lead', 'customer', 'partner', 'investor', 'other']),
  tags: z.array(z.string()),
});

type ContactFormData = z.infer<typeof contactSchema>;

export function ContactForm({ contact, onSubmit }: ContactFormProps) {
  const form = useForm<ContactFormData>({
    resolver: zodResolver(contactSchema),
    defaultValues: contact ?? {
      status: 'lead',
      tags: [],
    },
  });

  return (
    <form onSubmit={form.handleSubmit(onSubmit)}>
      {/* Form fields with proper error handling */}
    </form>
  );
}
```

### Week 13-14: Component Library & UX Polish

#### 4.4 Reusable Components
```
frontend/src/components/ui/
├── button.tsx
├── input.tsx
├── select.tsx
├── dialog.tsx
├── dropdown-menu.tsx
├── data-table.tsx
├── pagination.tsx
├── loading-skeleton.tsx
├── empty-state.tsx
└── toast.tsx
```

#### 4.5 Data Table with Sorting, Filtering, Pagination
```typescript
// frontend/src/components/ui/data-table.tsx

interface DataTableProps<T> {
  data: T[];
  columns: ColumnDef<T>[];
  isLoading?: boolean;
  pagination?: PaginationState;
  onPaginationChange?: (pagination: PaginationState) => void;
  sorting?: SortingState;
  onSortingChange?: (sorting: SortingState) => void;
  filters?: ColumnFiltersState;
  onFiltersChange?: (filters: ColumnFiltersState) => void;
}

// Usage
<DataTable
  data={contacts}
  columns={contactColumns}
  isLoading={isLoading}
  pagination={{ pageIndex: 0, pageSize: 20 }}
  onPaginationChange={setPagination}
  sorting={[{ id: 'created_at', desc: true }]}
  onSortingChange={setSorting}
/>
```

#### 4.6 Real-time Updates (WebSocket)
```typescript
// frontend/src/lib/realtime.ts

export function useRealtimeUpdates() {
  const queryClient = useQueryClient();

  useEffect(() => {
    const ws = new WebSocket(process.env.NEXT_PUBLIC_WS_URL!);

    ws.onmessage = (event) => {
      const message = JSON.parse(event.data);

      switch (message.type) {
        case 'contact_updated':
          queryClient.invalidateQueries({
            queryKey: contactKeys.detail(message.payload.id),
          });
          break;
        case 'timeline_entry_created':
          queryClient.invalidateQueries({
            queryKey: contactKeys.timeline(message.payload.contact_id),
          });
          break;
        case 'campaign_status_changed':
          queryClient.invalidateQueries({
            queryKey: campaignKeys.detail(message.payload.id),
          });
          break;
      }
    };

    return () => ws.close();
  }, [queryClient]);
}
```

---

## Phase 5: System Tests & E2E

### Week 15-16: End-to-End Testing

#### 5.1 Playwright E2E Tests
```typescript
// frontend/e2e/contacts.spec.ts

import { test, expect } from '@playwright/test';

test.describe('Contacts', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/contacts');
  });

  test('can create a new contact', async ({ page }) => {
    await page.click('button:has-text("Add Contact")');

    await page.fill('input[name="first_name"]', 'John');
    await page.fill('input[name="last_name"]', 'Doe');
    await page.fill('input[name="email"]', 'john@example.com');
    await page.selectOption('select[name="status"]', 'lead');

    await page.click('button:has-text("Create")');

    await expect(page.locator('text=John Doe')).toBeVisible();
    await expect(page.locator('text=john@example.com')).toBeVisible();
  });

  test('can search contacts', async ({ page }) => {
    await page.fill('input[placeholder="Search contacts..."]', 'john');

    await expect(page.locator('tbody tr')).toHaveCount(1);
    await expect(page.locator('text=John Doe')).toBeVisible();
  });

  test('can view contact timeline', async ({ page }) => {
    await page.click('text=John Doe');

    await expect(page.locator('h1')).toContainText('John Doe');
    await expect(page.locator('text=Timeline')).toBeVisible();
  });
});
```

#### 5.2 System Tests (Full Stack)
```typescript
// tests/system/campaign_workflow.spec.ts

test.describe('Campaign Workflow', () => {
  test('full campaign lifecycle', async ({ page, api }) => {
    // 1. Create contacts
    const contacts = await api.seedContacts(50, { status: 'lead' });

    // 2. Create campaign
    await page.goto('/campaigns/new');
    await page.fill('input[name="name"]', 'Test Campaign');
    await page.click('button:has-text("Lead Generation")');
    await page.click('button:has-text("Email")');
    await page.fill('textarea[name="prompt"]', 'Introduce our new product');
    await page.click('button:has-text("Create Campaign")');

    // 3. Generate assets
    await page.click('button:has-text("Generate Content")');
    await expect(page.locator('text=Email generated')).toBeVisible({ timeout: 30000 });

    // 4. Execute campaign
    await page.click('button:has-text("Launch Campaign")');
    await page.click('button:has-text("Confirm")');

    // 5. Verify timeline entries created
    await page.goto(`/contacts/${contacts[0].id}`);
    await expect(page.locator('text=Email sent')).toBeVisible();

    // 6. Verify analytics
    await page.goto('/analytics');
    await expect(page.locator('text=50')).toBeVisible(); // emails sent
  });
});
```

---

## Testing Commands

```bash
# Backend
cd backend

# Unit tests
cargo test --lib

# Integration tests (requires Docker)
cargo test --test '*' -- --test-threads=1

# Property tests
cargo test proptest

# Coverage
cargo tarpaulin --out Html

# Frontend
cd frontend

# Unit tests
npm run test

# E2E tests
npm run test:e2e

# E2E with UI
npm run test:e2e:ui

# Full system tests
npm run test:system
```

---

## Ownership Checklist

### Phase 1: Core Domain ✓
- [ ] Contact validation & business rules
- [ ] Campaign state machine
- [ ] Timeline aggregation
- [ ] Unit tests (>80% coverage)
- [ ] Property tests for edge cases

### Phase 2: API Layer ✓
- [ ] All handlers reviewed & refined
- [ ] Request validation middleware
- [ ] Error handling standardized
- [ ] OpenAPI documentation
- [ ] Integration tests for all endpoints

### Phase 3: Storage ✓
- [ ] Index optimization complete
- [ ] Graph relations defined
- [ ] Vector embeddings implemented
- [ ] Semantic search working
- [ ] Query performance benchmarked

### Phase 4: Frontend ✓
- [ ] State management patterns established
- [ ] Optimistic updates implemented
- [ ] Component library complete
- [ ] Real-time updates working
- [ ] All pages functional

### Phase 5: System Tests ✓
- [ ] E2E tests for critical paths
- [ ] System tests for workflows
- [ ] Performance tests
- [ ] Chaos/resilience tests

---

## Next Steps

1. Start with **Phase 1, Week 1**: Create `ContactService` and unit tests
2. Set up test infrastructure (`cargo-tarpaulin`, `testcontainers`)
3. Establish PR review checklist requiring tests
4. Track coverage metrics in CI

Would you like to start with any specific module?
