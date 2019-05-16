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
    pub gnetId: Value,
    pub graphTooltip: Option<i64>,
    pub hideControls: Option<bool>,
    pub id: i64,
    pub links: Option<Vec<Value>>,
    pub panels: Vec<Panels>,
    pub schemaVersion: i64,
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
    FunctionName: String,
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
    builtIn: i64,
    datasource: String,
    enable: bool,
    hide: bool,
    iconColor: String,
    name: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    #[serde(rename = "type")]
    _type: String,
    canSave: bool,
    canEdit: bool,
    canAdmin: bool,
    canStar: bool,
    slug: String,
    url: String,
    expires: String,
    created: String,
    updated: String,
    updatedBy: String,
    createdBy: String,
    version: i64,
    hasAcl: bool,
    isFolder: bool,
    folderId: i64,
    folderTitle: String,
    folderUrl: String,
    provisioned: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Panels {
    aliasColors: Option<Value>,
    bars: Option<bool>,
    dashLength: Option<i64>,
    dashes: Option<bool>,
    datasource: Option<String>,
    fill: Option<i64>,
    gridPos: Option<GridPos>,
    id: Option<i64>,
    legend: Option<Legend>,
    lines: Option<bool>,
    linewidth: Option<i64>,
    nullPointMode: Option<String>,
    percentage: Option<bool>,
    pointradius: Option<i64>,
    points: Option<bool>,
    renderer: Option<String>,
    seriesOverrides: Option<Vec<Value>>,
    spaceLength: Option<i64>,
    stack: Option<bool>,
    steppedLine: Option<bool>,
    targets: Option<Vec<Targets>>,
    thresholds: Option<Vec<Value>>,
    timeFrom: Option<Value>,
    timeShift: Option<Value>,
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
    highResolution: bool,
    metricName: String,
    namespace: String,
    period: String,
    refId: String,
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
    logBase: i64,
    max: Value,
    min: Value,
    show: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Yaxis {
    align: bool,
    alignLevel: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MutateDashboardResponse {
    id: i64,
    slug: String,
    status: String,
    uid: String,
    url: String,
    version: i64
}