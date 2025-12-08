-- CRM.HEY.SH Database Schema
-- SurrealDB Schema Definition

-- Contact table
DEFINE TABLE contact SCHEMAFULL;

DEFINE FIELD first_name ON TABLE contact TYPE string;
DEFINE FIELD last_name ON TABLE contact TYPE string;
DEFINE FIELD email ON TABLE contact TYPE string;
DEFINE FIELD phone ON TABLE contact TYPE option<string>;
DEFINE FIELD linkedin_url ON TABLE contact TYPE option<string>;
DEFINE FIELD tags ON TABLE contact TYPE array DEFAULT [];
DEFINE FIELD status ON TABLE contact TYPE string DEFAULT 'lead'
    ASSERT $value IN ['lead', 'customer', 'partner', 'investor', 'other'];
DEFINE FIELD engagement_score ON TABLE contact TYPE float DEFAULT 0;
DEFINE FIELD company ON TABLE contact TYPE option<record<company>>;
DEFINE FIELD created_at ON TABLE contact TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE contact TYPE datetime DEFAULT time::now();

DEFINE INDEX contact_email ON TABLE contact COLUMNS email UNIQUE;
DEFINE INDEX contact_status ON TABLE contact COLUMNS status;
DEFINE INDEX contact_engagement ON TABLE contact COLUMNS engagement_score;

-- Company table
DEFINE TABLE company SCHEMAFULL;

DEFINE FIELD name ON TABLE company TYPE string;
DEFINE FIELD domain ON TABLE company TYPE option<string>;
DEFINE FIELD industry ON TABLE company TYPE option<string>;
DEFINE FIELD size ON TABLE company TYPE option<string>;
DEFINE FIELD tags ON TABLE company TYPE array DEFAULT [];
DEFINE FIELD created_at ON TABLE company TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE company TYPE datetime DEFAULT time::now();

DEFINE INDEX company_name ON TABLE company COLUMNS name;
DEFINE INDEX company_domain ON TABLE company COLUMNS domain;

-- Timeline Entry table
DEFINE TABLE timeline_entry SCHEMAFULL;

DEFINE FIELD contact ON TABLE timeline_entry TYPE record<contact>;
DEFINE FIELD company ON TABLE timeline_entry TYPE option<record<company>>;
DEFINE FIELD type ON TABLE timeline_entry TYPE string
    ASSERT $value IN ['email_sent', 'email_open', 'email_click', 'social_touch', 'note', 'event_invite', 'event_attend', 'landing_page_visit', 'task', 'call'];
DEFINE FIELD content ON TABLE timeline_entry TYPE string;
DEFINE FIELD metadata ON TABLE timeline_entry TYPE object DEFAULT {};
DEFINE FIELD timestamp ON TABLE timeline_entry TYPE datetime DEFAULT time::now();

DEFINE INDEX timeline_contact ON TABLE timeline_entry COLUMNS contact;
DEFINE INDEX timeline_timestamp ON TABLE timeline_entry COLUMNS timestamp;
DEFINE INDEX timeline_type ON TABLE timeline_entry COLUMNS type;

-- Campaign table
DEFINE TABLE campaign SCHEMAFULL;

DEFINE FIELD name ON TABLE campaign TYPE string;
DEFINE FIELD objective ON TABLE campaign TYPE string
    ASSERT $value IN ['awareness', 'lead_gen', 'event', 'investor', 'early_adopters'];
DEFINE FIELD status ON TABLE campaign TYPE string DEFAULT 'draft'
    ASSERT $value IN ['draft', 'scheduled', 'running', 'completed'];
DEFINE FIELD channels ON TABLE campaign TYPE array DEFAULT [];
DEFINE FIELD prompt ON TABLE campaign TYPE option<string>;
DEFINE FIELD segment_definition ON TABLE campaign TYPE object DEFAULT {};
DEFINE FIELD created_at ON TABLE campaign TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE campaign TYPE datetime DEFAULT time::now();

DEFINE INDEX campaign_status ON TABLE campaign COLUMNS status;
DEFINE INDEX campaign_objective ON TABLE campaign COLUMNS objective;

-- Campaign Asset table
DEFINE TABLE campaign_asset SCHEMAFULL;

DEFINE FIELD campaign ON TABLE campaign_asset TYPE record<campaign>;
DEFINE FIELD type ON TABLE campaign_asset TYPE string
    ASSERT $value IN ['email', 'social_post', 'landing_page', 'event_invite'];
DEFINE FIELD generated_content ON TABLE campaign_asset TYPE object DEFAULT {};
DEFINE FIELD url ON TABLE campaign_asset TYPE option<string>;
DEFINE FIELD created_at ON TABLE campaign_asset TYPE datetime DEFAULT time::now();

DEFINE INDEX asset_campaign ON TABLE campaign_asset COLUMNS campaign;
DEFINE INDEX asset_type ON TABLE campaign_asset COLUMNS type;

-- Event table
DEFINE TABLE event SCHEMAFULL;

DEFINE FIELD campaign ON TABLE event TYPE option<record<campaign>>;
DEFINE FIELD name ON TABLE event TYPE string;
DEFINE FIELD type ON TABLE event TYPE string
    ASSERT $value IN ['webinar', 'meetup', 'ama', 'demo', 'other'];
DEFINE FIELD description ON TABLE event TYPE string;
DEFINE FIELD start_time ON TABLE event TYPE datetime;
DEFINE FIELD end_time ON TABLE event TYPE datetime;
DEFINE FIELD location ON TABLE event TYPE string;
DEFINE FIELD created_at ON TABLE event TYPE datetime DEFAULT time::now();

DEFINE INDEX event_start ON TABLE event COLUMNS start_time;
DEFINE INDEX event_campaign ON TABLE event COLUMNS campaign;

-- RSVP table
DEFINE TABLE rsvp SCHEMAFULL;

DEFINE FIELD event ON TABLE rsvp TYPE record<event>;
DEFINE FIELD contact ON TABLE rsvp TYPE record<contact>;
DEFINE FIELD status ON TABLE rsvp TYPE string DEFAULT 'invited'
    ASSERT $value IN ['invited', 'registered', 'attended', 'no_show'];
DEFINE FIELD timestamp ON TABLE rsvp TYPE datetime DEFAULT time::now();

DEFINE INDEX rsvp_event ON TABLE rsvp COLUMNS event;
DEFINE INDEX rsvp_contact ON TABLE rsvp COLUMNS contact;
DEFINE INDEX rsvp_event_contact ON TABLE rsvp COLUMNS event, contact UNIQUE;
