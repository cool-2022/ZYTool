from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import List, Optional
import math

router = APIRouter()


class RoutePoint(BaseModel):
    lat: float
    lng: float
    name: Optional[str] = None


class RouteRequest(BaseModel):
    origin: RoutePoint
    destination: RoutePoint
    waypoints: Optional[List[RoutePoint]] = []


class RouteStep(BaseModel):
    instruction: str
    distance: float  # in meters
    duration: float  # in seconds
    start_location: RoutePoint
    end_location: RoutePoint


class RouteResponse(BaseModel):
    distance: float  # in meters
    duration: float  # in seconds
    steps: List[RouteStep]
    polyline: List[RoutePoint]  # route coordinates


def calculate_distance(lat1: float, lng1: float, lat2: float, lng2: float) -> float:
    """
    Calculate the great circle distance between two points 
    on the earth (specified in decimal degrees)
    Returns distance in meters
    """
    # Convert decimal degrees to radians
    lat1, lng1, lat2, lng2 = map(math.radians, [lat1, lng1, lat2, lng2])

    # Haversine formula
    dlat = lat2 - lat1
    dlon = lng2 - lng1
    a = math.sin(dlat/2)**2 + math.cos(lat1) * \
        math.cos(lat2) * math.sin(dlon/2)**2
    c = 2 * math.asin(math.sqrt(a))
    r = 6371000  # Radius of earth in meters
    return c * r


def calculate_bearing(lat1: float, lng1: float, lat2: float, lng2: float) -> float:
    """
    Calculate the initial bearing between two points
    """
    lat1, lng1, lat2, lng2 = map(math.radians, [lat1, lng1, lat2, lng2])
    dLng = lng2 - lng1

    y = math.sin(dLng) * math.cos(lat2)
    x = math.cos(lat1) * math.sin(lat2) - math.sin(lat1) * \
        math.cos(lat2) * math.cos(dLng)
    brng = math.atan2(y, x)
    brng = math.degrees(brng)
    return (brng + 360) % 360


def generate_mock_route(origin: RoutePoint, destination: RoutePoint) -> RouteResponse:
    """
    Generate a mock route between origin and destination
    In a real implementation, this would call a mapping service like AMap or Google Maps API
    """
    distance = calculate_distance(
        origin.lat, origin.lng, destination.lat, destination.lng)

    # Estimate duration based on distance (assuming average speed of 30 km/h)
    avg_speed_ms = 30000 / 3600  # meters per second
    duration = distance / avg_speed_ms

    # Generate intermediate points for the route
    steps = []
    polyline = []

    # Add start point
    polyline.append(RoutePoint(lat=origin.lat, lng=origin.lng))

    # Calculate bearing from origin to destination
    bearing = calculate_bearing(
        origin.lat, origin.lng, destination.lat, destination.lng)

    # Generate intermediate points along the route
    # Between 2-20 points based on distance
    num_points = max(2, min(int(distance / 1000) + 1, 20))
    for i in range(1, num_points):
        fraction = i / num_points

        # Calculate intermediate point using bearing and distance
        lat_diff = (destination.lat - origin.lat) * fraction
        lng_diff = (destination.lng - origin.lng) * fraction

        intermediate_lat = origin.lat + lat_diff
        intermediate_lng = origin.lng + lng_diff

        polyline.append(RoutePoint(lat=intermediate_lat, lng=intermediate_lng))

        # Add step instruction
        steps.append(RouteStep(
            instruction=f"沿当前方向行驶约 {int(distance / num_points)} 米",
            distance=distance / num_points,
            duration=duration / num_points,
            start_location=polyline[-2],
            end_location=polyline[-1]
        ))

    # Add final destination
    polyline.append(RoutePoint(lat=destination.lat, lng=destination.lng))

    # Add final step
    steps.append(RouteStep(
        instruction="到达目的地",
        distance=calculate_distance(
            polyline[-2].lat, polyline[-2].lng, destination.lat, destination.lng),
        duration=duration / num_points,
        start_location=polyline[-2],
        end_location=polyline[-1]
    ))

    return RouteResponse(
        distance=distance,
        duration=duration,
        steps=steps,
        polyline=polyline
    )


@router.post("/route", response_model=RouteResponse, tags=["map"])
async def get_route(request: RouteRequest):
    """
    Get route between origin and destination
    """
    try:
        route = generate_mock_route(request.origin, request.destination)
        return route
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))


@router.get("/route/test", tags=["map"])
async def test_route():
    """
    Test endpoint for route functionality
    """
    return {"message": "Route API is working"}
