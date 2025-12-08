# Contact Module Learning Guide

Welcome to your ownership journey! This guide will help you understand and own the Contact module from the ground up.

## Your Mission

You need to understand the **core domain logic** before touching the database or UI. This means:

1. Understanding pure business rules (no I/O)
2. Writing and passing all tests
3. Implementing the missing pieces
4. Only then connecting to storage

## Quick Start

```bash
cd backend

# Run all domain tests
cargo test domain::

# Run specific module tests
cargo test domain::validation::
cargo test domain::contact::
cargo test domain::engagement::

# Run with output
cargo test domain:: -- --nocapture

# Run only your implementations (currently will fail)
cargo test validate_engagement_score
cargo test validate_company_domain
cargo test contact_updater
cargo test engagement_velocity
```

## Learning Path

### Step 1: Understand the Architecture (30 min)

Read these files in order:

1. `src/domain/mod.rs` - Overview of the domain layer
2. `src/domain/errors.rs` - How we handle business errors
3. `src/domain/validation.rs` - Pure validation functions

**Key Insight**: Everything in `domain/` is PURE. No database, no HTTP, no side effects. This makes it trivially testable.

### Step 2: Pass the Existing Tests (15 min)

Run the tests to see what's already working:

```bash
cargo test domain:: 2>&1 | head -50
```

You should see many tests passing. The failing ones are marked `#[ignore]` - those are for YOUR implementations.

### Step 3: Implement Missing Validations (45 min)

Open `src/domain/validation.rs` and implement:

#### 3.1: `validate_engagement_score`

```rust
pub fn validate_engagement_score(score: f64) -> DomainResult<()> {
    // YOUR CODE HERE
}
```

**Requirements**:
- Score must be between 0.0 and 100.0 (inclusive)
- NaN and Infinity are not allowed
- Return `DomainError::InvalidField` for violations

**Verify**:
```bash
cargo test test_engagement_score_validation
```

#### 3.2: `validate_company_domain`

```rust
pub fn validate_company_domain(domain: Option<&str>) -> DomainResult<()> {
    // YOUR CODE HERE
}
```

**Requirements**:
- None is valid
- If provided, must be a valid domain (example.com)
- No protocol prefix (reject http://)
- Must have at least one dot
- TLD must be 2-10 characters

**Verify**:
```bash
cargo test test_company_domain_validation
```

### Step 4: Understand the Contact Entity (30 min)

Read `src/domain/contact.rs` carefully:

1. **ContactStatus** - A state machine with specific transition rules
2. **Contact** - The entity with helper methods
3. **ContactBuilder** - The safe way to create contacts

**Exercise**: On paper, draw the state machine diagram for ContactStatus. Verify it matches the `can_transition_to` implementation.

### Step 5: Implement ContactUpdater (45 min)

Open `src/domain/contact.rs` and implement the `ContactUpdater` methods:

```rust
pub fn email(mut self, email: &str) -> DomainResult<Self> {
    // YOUR CODE HERE
}

pub fn phone(mut self, phone: Option<&str>) -> DomainResult<Self> {
    // YOUR CODE HERE
}

pub fn add_tag(mut self, tag: &str) -> DomainResult<Self> {
    // YOUR CODE HERE
}

pub fn status(mut self, new_status: ContactStatus) -> DomainResult<Self> {
    // YOUR CODE HERE
}
```

**Verify**:
```bash
# First, remove #[ignore] from the tests, then:
cargo test test_contact_updater
```

### Step 6: Understand Engagement Scoring (30 min)

Read `src/domain/engagement.rs`:

1. **InteractionType** - Different ways contacts engage
2. **Interaction** - A single engagement event
3. **calculate_engagement_score** - The core algorithm

**Exercise**: Calculate by hand what score these interactions would produce:
- EmailOpen today (base: 3.0)
- EmailClick 15 days ago (base: 5.0, ~50% decay)
- MeetingAttended 30 days ago (base: 20.0, ~50% decay)

### Step 7: Implement Engagement Functions (60 min)

#### 7.1: `calculate_engagement_velocity`

```rust
pub fn calculate_engagement_velocity(
    interactions: &[Interaction],
    config: &EngagementConfig,
) -> f64 {
    // YOUR CODE HERE
}
```

**Algorithm**:
1. Split interactions into 3 periods: 0-15 days, 15-30 days, 30-45 days
2. Calculate score for each period
3. recent_change = score_0_15 - score_15_30
4. older_change = score_15_30 - score_30_45
5. velocity = recent_change - older_change

#### 7.2: `identify_top_interaction_types`

```rust
pub fn identify_top_interaction_types(
    interactions: &[Interaction],
    config: &EngagementConfig,
    top_n: usize,
) -> Vec<(InteractionType, f64)> {
    // YOUR CODE HERE
}
```

**Algorithm**:
1. Group interactions by type
2. Calculate decayed score contribution for each type
3. Sort by contribution descending
4. Return top N

**Verify**:
```bash
# Remove #[ignore] from tests first
cargo test test_engagement_velocity
cargo test test_top_interaction_types
```

## Checkpoint: Are You Ready?

Before moving to the database layer, verify:

```bash
# ALL domain tests must pass
cargo test domain::

# Check test coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html -- domain::
# Open tarpaulin-report.html
```

**Target**: 90%+ coverage on the domain module.

## Questions to Answer

Before proceeding, write answers to these questions:

1. Why do we separate DomainError from database errors?
2. Why does ContactBuilder return a Result instead of panicking?
3. What's the benefit of the state machine pattern for ContactStatus?
4. Why is engagement scoring a pure function?
5. How would you add a new InteractionType? What would you need to update?

## Next Steps

Once ALL tests pass and you can answer all questions:

1. **Repository Layer**: Connect domain to SurrealDB
2. **Service Layer**: Orchestrate domain + repository
3. **Handler Layer**: HTTP endpoints calling services
4. **Integration Tests**: Test the full stack

But NOT before you own the domain layer.

---

## Getting Help

If you're stuck:

1. Read the existing test cases - they document expected behavior
2. Use `cargo test -- --nocapture` to see println! output
3. Add `#[test]` functions to experiment
4. The compiler errors are often very helpful in Rust

Good luck! 🚀
