use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotations {
    list: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dashboard {
    pub annotations: Option<Annotations>,
    pub editable: bool,
    #[serde(rename = "gnetId")]
    pub gnet_id: Value,
    #[serde(rename = "graphTooltip")]
    pub graph_tooltip: Option<i64>,
    #[serde(rename = "hideControls")]
    pub hide_controls: Option<bool>,
    pub id: i64,
    pub links: Option<Vec<Value>>,
    pub panels: Vec<Panels>,
    #[serde(rename = "schemaVersion")]
    pub schema_version: i64,
    pub style: Option<String>,
    pub tags: Option<Vec<Value>>,
    pub templating: Option<Annotations>,
    pub time: Option<Time>,
    pub timepicker: Option<Timepicker>,
    pub timezone: Option<String>,
    pub title: String,
    pub uid: String,
    pub version: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dimensions {
    #[serde(rename = "FunctionName")]
    function_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GridPos {
    h: i64,
    w: i64,
    x: i64,
    y: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Legend {
    avg: bool,
    current: bool,
    max: bool,
    min: bool,
    show: bool,
    total: bool,
    values: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    #[serde(rename = "builtIn")]
    built_in: i64,
    datasource: String,
    enable: bool,
    hide: bool,
    #[serde(rename = "iconColor")]
    icon_color: String,
    name: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    #[serde(rename = "type")]
    _type: String,
    #[serde(rename = "canSave")]
    can_save: bool,
    #[serde(rename = "canEdit")]
    can_edit: bool,
    #[serde(rename = "canAdmin")]
    can_admin: bool,
    #[serde(rename = "canStar")]
    can_star: bool,
    slug: String,
    url: String,
    expires: String,
    created: String,
    updated: String,
    #[serde(rename = "updatedBy")]
    updated_by: String,
    #[serde(rename = "createdBy")]
    created_by: String,
    version: i64,
    #[serde(rename = "hasAcl")]
    has_acl: bool,
    #[serde(rename = "isFolder")]
    is_folder: bool,
    #[serde(rename = "folderId")]
    folder_id: i64,
    #[serde(rename = "folderTitle")]
    folder_title: String,
    #[serde(rename = "folderUrl")]
    folder_url: String,
    provisioned: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Panels {
    #[serde(rename = "aliasColors")]
    alias_colors: Option<Value>,
    bars: Option<bool>,
    #[serde(rename = "dashLength")]
    dash_length: Option<i64>,
    dashes: Option<bool>,
    datasource: Option<String>,
    fill: Option<i64>,
    #[serde(rename = "gridPos")]
    grid_pos: Option<GridPos>,
    id: Option<i64>,
    legend: Option<Legend>,
    lines: Option<bool>,
    linewidth: Option<i64>,
    #[serde(rename = "nullPointMode")]
    null_point_mode: Option<String>,
    percentage: Option<bool>,
    pointradius: Option<i64>,
    points: Option<bool>,
    renderer: Option<String>,
    #[serde(rename = "seriesOverrides")]
    series_overrides: Option<Vec<Value>>,
    #[serde(rename = "spaceLength")]
    space_length: Option<i64>,
    stack: Option<bool>,
    #[serde(rename = "steppedLine")]
    stepped_line: Option<bool>,
    targets: Option<Vec<Targets>>,
    thresholds: Option<Vec<Value>>,
    #[serde(rename = "timeFrom")]
    time_from: Option<Value>,
    #[serde(rename = "timeShift")]
    time_shift: Option<Value>,
    title: Option<String>,
    tooltip: Option<Tooltip>,
    #[serde(rename = "type")]
    _type: Option<String>,
    xaxis: Option<Xaxis>,
    yaxes: Option<Vec<Yaxes>>,
    yaxis: Option<Yaxis>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootInterface {
    pub meta: Option<Meta>,
    pub dashboard: Dashboard,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Targets {
    dimensions: Dimensions,
    #[serde(rename = "highResolution")]
    high_resolution: bool,
    #[serde(rename = "metricName")]
    metric_name: String,
    namespace: String,
    period: String,
    #[serde(rename = "refId")]
    ref_id: String,
    region: String,
    statistics: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    from: String,
    to: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timepicker {
    refresh_intervals: Vec<String>,
    time_options: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tooltip {
    shared: bool,
    sort: i64,
    value_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Xaxis {
    buckets: Value,
    mode: String,
    name: Value,
    show: bool,
    values: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Yaxes {
    format: String,
    label: Value,
    #[serde(rename = "logBase")]
    log_base: i64,
    max: Value,
    min: Value,
    show: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Yaxis {
    align: bool,
    #[serde(rename = "alignLevel")]
    align_level: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MutateDashboardResponse {
    pub id: i64,
    pub slug: String,
    pub status: String,
    pub uid: String,
    pub url: String,
    pub version: i64
}