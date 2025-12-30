mod apple_wps;

use apple_wps::{AlsLocationRequest, AlsLocationResponse, CellRequest};
use bytes::{BufMut, BytesMut};
use prost::Message;
use serde::{Deserialize, Serialize};
use worker::*;

const GRAPHENEOS_PROXY_URL: &str = "https://gs-loc.apple.grapheneos.org/clls/wloc";

// MLS Request types
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct MlsRequest {
    #[serde(default)]
    consider_ip: Option<bool>,
    #[serde(default)]
    radio_type: Option<String>,
    #[serde(default)]
    cell_towers: Option<Vec<CellTower>>,
    #[serde(default)]
    wifi_access_points: Option<Vec<WifiAccessPoint>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CellTower {
    #[serde(default)]
    radio_type: Option<String>,
    mobile_country_code: i32,
    mobile_network_code: i32,
    location_area_code: i32,
    cell_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct WifiAccessPoint {
    mac_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    signal_strength: Option<i32>,
}

// MLS Response types
#[derive(Debug, Deserialize, Serialize)]
struct MlsResponse {
    location: Location,
    accuracy: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    fallback: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Location {
    lat: f64,
    lng: f64,
}

#[derive(Debug, Serialize)]
struct MlsError {
    error: MlsErrorDetail,
}

#[derive(Debug, Serialize)]
struct MlsErrorDetail {
    errors: Vec<MlsErrorItem>,
    code: u16,
    message: String,
}

#[derive(Debug, Serialize)]
struct MlsErrorItem {
    domain: String,
    reason: String,
    message: String,
}

impl MlsRequest {
    fn has_wifi_data(&self) -> bool {
        self.wifi_access_points
            .as_ref()
            .map(|w| w.len() >= 2)
            .unwrap_or(false)
    }

    fn has_cell_data(&self) -> bool {
        self.cell_towers
            .as_ref()
            .map(|c| !c.is_empty())
            .unwrap_or(false)
    }

    fn has_network_data(&self) -> bool {
        self.has_wifi_data() || self.has_cell_data()
    }

    fn get_bssids(&self) -> Vec<String> {
        self.wifi_access_points
            .as_ref()
            .map(|aps| aps.iter().map(|ap| normalize_bssid(&ap.mac_address)).collect())
            .unwrap_or_default()
    }

    fn get_cells(&self, global_radio_type: &Option<String>) -> Vec<CellRequest> {
        self.cell_towers
            .as_ref()
            .map(|cells| {
                cells
                    .iter()
                    .map(|c| {
                        let radio = c
                            .radio_type
                            .as_ref()
                            .or(global_radio_type.as_ref())
                            .cloned()
                            .unwrap_or_else(|| "lte".to_string());
                        CellRequest {
                            radio_type: radio,
                            mcc: c.mobile_country_code,
                            mnc: c.mobile_network_code,
                            lac: c.location_area_code,
                            cell_id: c.cell_id,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

fn normalize_bssid(mac: &str) -> String {
    // Apple accepts both formats, but normalize to lowercase with colons
    mac.to_lowercase()
        .replace('-', ":")
}

fn build_error_response() -> MlsError {
    MlsError {
        error: MlsErrorDetail {
            errors: vec![MlsErrorItem {
                domain: "geolocation".to_string(),
                reason: "notFound".to_string(),
                message: "Not found".to_string(),
            }],
            code: 404,
            message: "Not found".to_string(),
        },
    }
}

fn build_cloudflare_response(cf: &Cf) -> Option<MlsResponse> {
    let (lat, lng) = cf.coordinates()?;

    let accuracy = if cf.postal_code().is_some() {
        5000.0
    } else if cf.city().is_some() {
        20000.0
    } else if cf.region().is_some() {
        100000.0
    } else {
        500000.0
    };

    Some(MlsResponse {
        location: Location {
            lat: lat as f64,
            lng: lng as f64,
        },
        accuracy,
        fallback: Some("ipf".to_string()),
    })
}

fn build_apple_request_body(request: &AlsLocationRequest) -> Vec<u8> {
    let locale = b"en_US";
    let identifier = b"com.apple.locationd";
    let version = b"15.4.24E248";
    let request_code: i32 = 1;

    let proto_bytes = request.encode_to_vec();

    let mut buf = BytesMut::with_capacity(
        2 + 2 + locale.len() + 2 + identifier.len() + 2 + version.len() + 4 + 4 + proto_bytes.len(),
    );

    buf.put_i16(1); // hardcoded
    buf.put_i16(locale.len() as i16);
    buf.put_slice(locale);
    buf.put_i16(identifier.len() as i16);
    buf.put_slice(identifier);
    buf.put_i16(version.len() as i16);
    buf.put_slice(version);
    buf.put_i32(request_code);
    buf.put_i32(proto_bytes.len() as i32);
    buf.put_slice(&proto_bytes);

    buf.to_vec()
}

async fn query_apple_wps(request: &AlsLocationRequest) -> Result<Option<AlsLocationResponse>> {
    let body = build_apple_request_body(request);

    let body_array = js_sys::Uint8Array::from(body.as_slice());
    
    let new_headers = web_sys::Headers::new().map_err(|e| Error::JsError(format!("{:?}", e)))?;
    new_headers.set("Accept", "*/*").map_err(|e| Error::JsError(format!("{:?}", e)))?;
    new_headers.set("Accept-Language", "en-US,en;q=0.9").map_err(|e| Error::JsError(format!("{:?}", e)))?;
    new_headers.set("Content-Type", "application/x-www-form-urlencoded").map_err(|e| Error::JsError(format!("{:?}", e)))?;
    new_headers.set("User-Agent", "locationd/2960.0.57 CFNetwork/3826.500.111.1.1 Darwin/24.4.0").map_err(|e| Error::JsError(format!("{:?}", e)))?;
    
    let new_init = web_sys::RequestInit::new();
    new_init.set_method("POST");
    new_init.set_body(&body_array);
    new_init.set_headers(&new_headers);
    
    let web_req = web_sys::Request::new_with_str_and_init(GRAPHENEOS_PROXY_URL, &new_init)
        .map_err(|e| Error::JsError(format!("{:?}", e)))?;
    
    let req = Request::from(web_req);
    let mut response = Fetch::Request(req).send().await?;

    if response.status_code() != 200 {
        return Ok(None);
    }

    let response_bytes = response.bytes().await?;
    
    // Skip first 10 bytes (header)
    if response_bytes.len() <= 10 {
        return Ok(None);
    }
    
    let proto_bytes = &response_bytes[10..];
    let als_response = AlsLocationResponse::decode(proto_bytes)
        .map_err(|e| Error::RustError(format!("Protobuf decode error: {}", e)))?;

    Ok(Some(als_response))
}

fn estimate_position_from_aps(response: &AlsLocationResponse) -> Option<MlsResponse> {
    // Collect all valid WiFi positions with their accuracy
    let mut positions: Vec<(f64, f64, i32)> = Vec::new();

    for ap in &response.wireless_aps {
        if let Some(loc) = &ap.location {
            if let Some((lat, lng, acc)) = loc.to_coordinates() {
                positions.push((lat, lng, acc));
            }
        }
    }

    if positions.is_empty() {
        return None;
    }

    // Simple weighted average by inverse accuracy
    let mut total_weight = 0.0;
    let mut weighted_lat = 0.0;
    let mut weighted_lng = 0.0;
    let mut min_accuracy = i32::MAX;

    for (lat, lng, acc) in &positions {
        let weight = 1.0 / (*acc as f64).max(1.0);
        weighted_lat += lat * weight;
        weighted_lng += lng * weight;
        total_weight += weight;
        min_accuracy = min_accuracy.min(*acc);
    }

    if total_weight == 0.0 {
        return None;
    }

    let final_lat = weighted_lat / total_weight;
    let final_lng = weighted_lng / total_weight;
    
    // Use the best accuracy among found APs, but at least the minimum
    let final_accuracy = min_accuracy.max(10) as f64;

    Some(MlsResponse {
        location: Location {
            lat: final_lat,
            lng: final_lng,
        },
        accuracy: final_accuracy,
        fallback: None,
    })
}

fn estimate_position_from_cells(response: &AlsLocationResponse) -> Option<MlsResponse> {
    let mut positions: Vec<(f64, f64, i32)> = Vec::new();

    // Collect from all cell tower types
    for tower in &response.gsm_cell_towers {
        if let Some(loc) = &tower.location {
            if let Some(coords) = loc.to_coordinates() {
                positions.push(coords);
            }
        }
    }
    for tower in &response.lte_cell_towers {
        if let Some(loc) = &tower.location {
            if let Some(coords) = loc.to_coordinates() {
                positions.push(coords);
            }
        }
    }
    for tower in &response.scdma_cell_towers {
        if let Some(loc) = &tower.location {
            if let Some(coords) = loc.to_coordinates() {
                positions.push(coords);
            }
        }
    }
    for tower in &response.nr5g_cell_towers {
        if let Some(loc) = &tower.location {
            if let Some(coords) = loc.to_coordinates() {
                positions.push(coords);
            }
        }
    }

    if positions.is_empty() {
        return None;
    }

    // Weighted average
    let mut total_weight = 0.0;
    let mut weighted_lat = 0.0;
    let mut weighted_lng = 0.0;
    let mut min_accuracy = i32::MAX;

    for (lat, lng, acc) in &positions {
        let weight = 1.0 / (*acc as f64).max(1.0);
        weighted_lat += lat * weight;
        weighted_lng += lng * weight;
        total_weight += weight;
        min_accuracy = min_accuracy.min(*acc);
    }

    if total_weight == 0.0 {
        return None;
    }

    Some(MlsResponse {
        location: Location {
            lat: weighted_lat / total_weight,
            lng: weighted_lng / total_weight,
        },
        accuracy: min_accuracy.max(100) as f64,
        fallback: Some("lacf".to_string()), // Cell tower fallback
    })
}

fn json_response<T: Serialize>(data: &T, status: u16) -> Result<Response> {
    let body = serde_json::to_string(data)?;
    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;

    Ok(Response::builder()
        .with_status(status)
        .with_headers(headers)
        .fixed(body.into_bytes()))
}

#[event(fetch)]
async fn fetch(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let cf = req.cf().cloned();

    // Parse request body if present
    let mls_request: MlsRequest = if req.method() == Method::Post {
        req.json().await.unwrap_or_default()
    } else {
        MlsRequest::default()
    };

    let consider_ip = mls_request.consider_ip.unwrap_or(true);

    // If we have network data, try Apple WPS via GrapheneOS proxy
    if mls_request.has_network_data() {
        let bssids = mls_request.get_bssids();
        let cells = mls_request.get_cells(&mls_request.radio_type);

        let apple_request = if !bssids.is_empty() && !cells.is_empty() {
            AlsLocationRequest::new_combined_request(&bssids, cells, 100, 25)
        } else if !bssids.is_empty() {
            AlsLocationRequest::new_wifi_request(&bssids, 100)
        } else {
            AlsLocationRequest::new_cell_request(cells, 25)
        };

        if let Ok(Some(apple_response)) = query_apple_wps(&apple_request).await {
            // Try WiFi first (more accurate), then cells
            if let Some(response) = estimate_position_from_aps(&apple_response) {
                return json_response(&response, 200);
            }
            if let Some(response) = estimate_position_from_cells(&apple_response) {
                return json_response(&response, 200);
            }
        }
    }

    // Fall back to Cloudflare IP geolocation if allowed
    if consider_ip {
        if let Some(cf) = cf.as_ref() {
            if let Some(response) = build_cloudflare_response(cf) {
                return json_response(&response, 200);
            }
        }
    }

    // No location available
    json_response(&build_error_response(), 404)
}
