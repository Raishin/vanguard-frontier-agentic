//! Coverage matrix data models — cells, rows, and the full matrix.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::provider::Provider;

/// Asset type discriminator shared by the coverage and scanner modules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    Agent,
    Skill,
    Rule,
    McpRef,
}

/// Classification of a single (asset, workspace) coverage cell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CellStatus {
    /// Content hash matches the canonical hash — asset is current.
    Installed,
    /// Installed version is behind the canonical version.
    Outdated,
    /// Content hash does not match regardless of version — local modification detected.
    Drifted,
    /// Asset is not present in this workspace.
    NotInstalled,
}

/// Data for a single cell in the coverage matrix (one asset × one workspace).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageCell {
    /// Classification of the installed asset relative to the canonical asset.
    pub status: CellStatus,
    /// Version string extracted from the installed copy, if detectable.
    pub installed_version: Option<String>,
    /// Version string from the canonical catalog entry.
    pub canonical_version: String,
    /// SHA-256 hash of the installed file, if present.
    pub installed_hash: Option<String>,
    /// SHA-256 hash recorded in the canonical asset-integrity manifest.
    pub canonical_hash: Option<String>,
}

/// One row of the coverage matrix — represents a single catalog asset across all workspaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageRow {
    /// Unique catalog asset identifier.
    pub asset_id: String,
    /// Discriminator for the asset kind.
    pub asset_type: AssetType,
    /// Human-readable asset name.
    pub asset_name: String,
    /// Provider that owns this asset.
    pub provider: Provider,
}

/// Full coverage matrix: rows are catalog assets, columns are workspace names.
///
/// `cells` is keyed by `(asset_id, workspace_name)`.  In JSON the tuple key is
/// serialized as `"<asset_id>\x1f<workspace_name>"` (ASCII Unit Separator 0x1F)
/// so it remains a valid JSON string key.
/// `workspace_scores` contains computed coverage percentages per workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageMatrix {
    /// One row per catalog asset.
    pub rows: Vec<CoverageRow>,
    /// Ordered workspace names (column headers).
    pub columns: Vec<String>,
    /// Cell values keyed by `(asset_id, workspace_name)`.
    #[serde(with = "cell_map_serde")]
    pub cells: HashMap<(String, String), CoverageCell>,
    /// Computed coverage percentage per workspace (0.0–100.0).
    pub workspace_scores: HashMap<String, f64>,
}

/// Custom serde module for `HashMap<(String, String), CoverageCell>`.
///
/// JSON object keys must be strings.  We encode the `(asset_id, workspace_name)`
/// tuple as `"<asset_id>\x1f<workspace_name>"` (ASCII Unit Separator 0x1F),
/// which guarantees reversibility as long as neither component contains 0x1F.
mod cell_map_serde {
    use std::collections::HashMap;

    use serde::de::{Deserializer, MapAccess, Visitor};
    use serde::ser::{SerializeMap, Serializer};

    use super::CoverageCell;

    const SEP: char = '\x1f';

    pub fn serialize<S>(
        map: &HashMap<(String, String), CoverageCell>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(map.len()))?;
        for ((asset_id, workspace), cell) in map {
            let key = format!("{}{}{}", asset_id, SEP, workspace);
            s.serialize_entry(&key, cell)?;
        }
        s.end()
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<(String, String), CoverageCell>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MapVisitor;

        impl<'de> Visitor<'de> for MapVisitor {
            type Value = HashMap<(String, String), CoverageCell>;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a map with \"<asset_id>\\x1f<workspace>\" string keys")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut map = HashMap::new();
                while let Some((key, value)) = access.next_entry::<String, CoverageCell>()? {
                    let sep_pos = key.find('\x1f').ok_or_else(|| {
                        serde::de::Error::custom(format!(
                            "cell map key missing separator: {:?}",
                            key
                        ))
                    })?;
                    let asset_id = key[..sep_pos].to_string();
                    let workspace = key[sep_pos + '\x1f'.len_utf8()..].to_string();
                    map.insert((asset_id, workspace), value);
                }
                Ok(map)
            }
        }

        deserializer.deserialize_map(MapVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cell(status: CellStatus) -> CoverageCell {
        CoverageCell {
            status,
            installed_version: Some("1.0.0".to_string()),
            canonical_version: "1.0.0".to_string(),
            installed_hash: Some("abc123".to_string()),
            canonical_hash: Some("abc123".to_string()),
        }
    }

    #[test]
    fn cell_status_round_trip() {
        for status in [
            CellStatus::Installed,
            CellStatus::Outdated,
            CellStatus::Drifted,
            CellStatus::NotInstalled,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            let decoded: CellStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(decoded, status);
        }
    }

    #[test]
    fn cell_status_serialized_strings() {
        assert_eq!(
            serde_json::to_string(&CellStatus::Installed).unwrap(),
            "\"installed\""
        );
        assert_eq!(
            serde_json::to_string(&CellStatus::NotInstalled).unwrap(),
            "\"not_installed\""
        );
        assert_eq!(
            serde_json::to_string(&CellStatus::Drifted).unwrap(),
            "\"drifted\""
        );
        assert_eq!(
            serde_json::to_string(&CellStatus::Outdated).unwrap(),
            "\"outdated\""
        );
    }

    #[test]
    fn coverage_cell_round_trip() {
        let cell = make_cell(CellStatus::Outdated);
        let json = serde_json::to_string(&cell).unwrap();
        let decoded: CoverageCell = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.status, CellStatus::Outdated);
        assert_eq!(decoded.canonical_version, "1.0.0");
    }

    #[test]
    fn coverage_row_round_trip() {
        let row = CoverageRow {
            asset_id: "aws-iam-scanner".to_string(),
            asset_type: AssetType::Agent,
            asset_name: "AWS IAM Scanner".to_string(),
            provider: Provider::Aws,
        };
        let json = serde_json::to_string(&row).unwrap();
        let decoded: CoverageRow = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.asset_id, row.asset_id);
        assert_eq!(decoded.asset_type, AssetType::Agent);
    }

    #[test]
    fn asset_type_serialized_strings() {
        assert_eq!(
            serde_json::to_string(&AssetType::Agent).unwrap(),
            "\"agent\""
        );
        assert_eq!(
            serde_json::to_string(&AssetType::McpRef).unwrap(),
            "\"mcp_ref\""
        );
    }

    #[test]
    fn coverage_matrix_round_trip() {
        let mut cells = HashMap::new();
        cells.insert(
            ("aws-iam-scanner".to_string(), "prod".to_string()),
            make_cell(CellStatus::Installed),
        );
        let mut scores = HashMap::new();
        scores.insert("prod".to_string(), 100.0_f64);

        let matrix = CoverageMatrix {
            rows: vec![CoverageRow {
                asset_id: "aws-iam-scanner".to_string(),
                asset_type: AssetType::Agent,
                asset_name: "AWS IAM Scanner".to_string(),
                provider: Provider::Aws,
            }],
            columns: vec!["prod".to_string()],
            cells,
            workspace_scores: scores,
        };

        let json = serde_json::to_string(&matrix).unwrap();
        let decoded: CoverageMatrix = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.columns, vec!["prod"]);
        assert!(decoded
            .cells
            .contains_key(&("aws-iam-scanner".to_string(), "prod".to_string())));
    }
}
