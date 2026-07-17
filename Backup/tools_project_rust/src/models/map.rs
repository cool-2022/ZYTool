use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoutePoint {
    pub lat: f64,
    pub lng: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteRequest {
    pub origin: RoutePoint,
    pub destination: RoutePoint,
    #[serde(default)]
    pub waypoints: Vec<RoutePoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteStep {
    pub instruction: String,
    pub distance: f64,
    pub duration: f64,
    pub start_location: RoutePoint,
    pub end_location: RoutePoint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteResponse {
    pub distance: f64,
    pub duration: f64,
    pub steps: Vec<RouteStep>,
    pub polyline: Vec<RoutePoint>,
}
