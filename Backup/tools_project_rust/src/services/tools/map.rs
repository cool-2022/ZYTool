use crate::models::{RoutePoint, RouteRequest, RouteResponse, RouteStep};

const EARTH_RADIUS: f64 = 6371000.0;
const AVG_SPEED_MS: f64 = 30000.0 / 3600.0;

fn to_radians(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

fn calculate_distance(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let lat1 = to_radians(lat1);
    let lng1 = to_radians(lng1);
    let lat2 = to_radians(lat2);
    let lng2 = to_radians(lng2);

    let dlat = lat2 - lat1;
    let dlon = lng2 - lng1;
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    c * EARTH_RADIUS
}

pub fn generate_mock_route(request: &RouteRequest) -> RouteResponse {
    let origin = &request.origin;
    let destination = &request.destination;
    let distance = calculate_distance(origin.lat, origin.lng, destination.lat, destination.lng);
    let duration = distance / AVG_SPEED_MS;

    let mut polyline = vec![RoutePoint {
        lat: origin.lat,
        lng: origin.lng,
        name: origin.name.clone(),
    }];
    let mut steps = vec![];

    let num_points = (distance / 1000.0) as usize + 1;
    let num_points = num_points.clamp(2, 20);

    for i in 1..num_points {
        let fraction = i as f64 / num_points as f64;
        let lat = origin.lat + (destination.lat - origin.lat) * fraction;
        let lng = origin.lng + (destination.lng - origin.lng) * fraction;
        let point = RoutePoint { lat, lng, name: None };
        let prev = polyline.last().unwrap().clone();
        steps.push(RouteStep {
            instruction: format!("沿当前方向行驶约 {} 米", (distance / num_points as f64) as i64),
            distance: distance / num_points as f64,
            duration: duration / num_points as f64,
            start_location: prev,
            end_location: point.clone(),
        });
        polyline.push(point);
    }

    let final_distance = calculate_distance(
        polyline.last().unwrap().lat,
        polyline.last().unwrap().lng,
        destination.lat,
        destination.lng,
    );
    steps.push(RouteStep {
        instruction: "到达目的地".to_string(),
        distance: final_distance,
        duration: duration / num_points as f64,
        start_location: polyline.last().unwrap().clone(),
        end_location: RoutePoint {
            lat: destination.lat,
            lng: destination.lng,
            name: destination.name.clone(),
        },
    });
    polyline.push(RoutePoint {
        lat: destination.lat,
        lng: destination.lng,
        name: destination.name.clone(),
    });

    RouteResponse {
        distance,
        duration,
        steps,
        polyline,
    }
}
