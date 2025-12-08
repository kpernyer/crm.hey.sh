use serde::{Deserialize, Serialize};

/// Service for building contact segments based on filter criteria
pub struct SegmentBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    GreaterThan,
    LessThan,
    In,
    NotIn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentDefinition {
    pub filters: Vec<SegmentFilter>,
    pub logic: LogicOperator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogicOperator {
    And,
    Or,
}

impl SegmentBuilder {
    /// Build a SurrealQL WHERE clause from a segment definition
    pub fn build_query(definition: &SegmentDefinition) -> String {
        if definition.filters.is_empty() {
            return String::new();
        }

        let conditions: Vec<String> = definition
            .filters
            .iter()
            .filter_map(|f| Self::filter_to_condition(f))
            .collect();

        if conditions.is_empty() {
            return String::new();
        }

        let connector = match definition.logic {
            LogicOperator::And => " AND ",
            LogicOperator::Or => " OR ",
        };

        format!("WHERE {}", conditions.join(connector))
    }

    fn filter_to_condition(filter: &SegmentFilter) -> Option<String> {
        let field = &filter.field;
        let value = &filter.value;

        let condition = match filter.operator {
            FilterOperator::Equals => {
                format!("{} = {}", field, Self::value_to_surql(value))
            }
            FilterOperator::NotEquals => {
                format!("{} != {}", field, Self::value_to_surql(value))
            }
            FilterOperator::Contains => {
                if let Some(s) = value.as_str() {
                    format!("{} CONTAINS '{}'", field, s)
                } else {
                    return None;
                }
            }
            FilterOperator::NotContains => {
                if let Some(s) = value.as_str() {
                    format!("NOT {} CONTAINS '{}'", field, s)
                } else {
                    return None;
                }
            }
            FilterOperator::GreaterThan => {
                format!("{} > {}", field, Self::value_to_surql(value))
            }
            FilterOperator::LessThan => {
                format!("{} < {}", field, Self::value_to_surql(value))
            }
            FilterOperator::In => {
                if let Some(arr) = value.as_array() {
                    let items: Vec<String> = arr.iter().map(|v| Self::value_to_surql(v)).collect();
                    format!("{} IN [{}]", field, items.join(", "))
                } else {
                    return None;
                }
            }
            FilterOperator::NotIn => {
                if let Some(arr) = value.as_array() {
                    let items: Vec<String> = arr.iter().map(|v| Self::value_to_surql(v)).collect();
                    format!("{} NOT IN [{}]", field, items.join(", "))
                } else {
                    return None;
                }
            }
        };

        Some(condition)
    }

    fn value_to_surql(value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => format!("'{}'", s.replace('\'', "\\'")),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "NONE".to_string(),
            _ => format!("'{}'", value.to_string().replace('\'', "\\'")),
        }
    }
}
