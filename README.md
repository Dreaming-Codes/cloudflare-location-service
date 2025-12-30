# Cloudflare Location Service

A geolocation API service running on Cloudflare Workers that provides location lookup using WiFi access points and cell tower data. It uses Apple's WiFi Positioning System (WPS) via the [GrapheneOS proxy](https://github.com/nickcianciolo/apple-wps-proxy) as the primary data source, with Cloudflare's IP geolocation as a fallback.

**Live endpoint:** https://cloudflare-location-service.dreamingcodes.workers.dev/

## Features

- **WiFi-based geolocation** - Locate devices using nearby WiFi access point BSSIDs (requires at least 2 APs)
- **Cell tower geolocation** - Supports GSM, LTE, WCDMA, and 5G NR cell towers
- **Combined lookups** - Use both WiFi and cell data together for better accuracy
- **IP fallback** - Falls back to Cloudflare's IP-based geolocation when network data is unavailable
- **MLS-compatible API** - Drop-in replacement for Mozilla Location Service / Google Geolocation API

## API Usage

### Endpoint

```
POST https://cloudflare-location-service.dreamingcodes.workers.dev/
```

### Request Format

The API accepts JSON requests compatible with the [Mozilla Location Service](https://ichnaea.readthedocs.io/en/latest/api/geolocate.html) format:

```json
{
  "considerIp": true,
  "radioType": "lte",
  "cellTowers": [
    {
      "radioType": "lte",
      "mobileCountryCode": 310,
      "mobileNetworkCode": 410,
      "locationAreaCode": 12345,
      "cellId": 67890
    }
  ],
  "wifiAccessPoints": [
    {
      "macAddress": "00:11:22:33:44:55",
      "signalStrength": -65
    },
    {
      "macAddress": "66:77:88:99:AA:BB",
      "signalStrength": -70
    }
  ]
}
```

### Request Fields

| Field | Type | Description |
|-------|------|-------------|
| `considerIp` | boolean | Whether to use IP geolocation as fallback (default: `true`) |
| `radioType` | string | Default radio type for cell towers: `gsm`, `lte`, `wcdma` |
| `cellTowers` | array | List of visible cell towers |
| `wifiAccessPoints` | array | List of visible WiFi access points (minimum 2 required) |

#### Cell Tower Object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `radioType` | string | No | Radio type: `gsm`, `lte`, `wcdma` |
| `mobileCountryCode` | integer | Yes | Mobile Country Code (MCC) |
| `mobileNetworkCode` | integer | Yes | Mobile Network Code (MNC) |
| `locationAreaCode` | integer | Yes | Location Area Code (LAC) or Tracking Area Code (TAC) |
| `cellId` | integer | Yes | Cell ID |

#### WiFi Access Point Object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `macAddress` | string | Yes | BSSID in format `XX:XX:XX:XX:XX:XX` or `XX-XX-XX-XX-XX-XX` |
| `signalStrength` | integer | No | Signal strength in dBm (not currently used for weighting) |

### Response Format

#### Success Response (200)

```json
{
  "location": {
    "lat": 37.7749,
    "lng": -122.4194
  },
  "accuracy": 30.0,
  "fallback": null
}
```

| Field | Description |
|-------|-------------|
| `location.lat` | Latitude in degrees |
| `location.lng` | Longitude in degrees |
| `accuracy` | Accuracy radius in meters |
| `fallback` | Fallback method used: `null` (none), `"ipf"` (IP fallback), `"lacf"` (cell tower fallback) |

#### Error Response (404)

```json
{
  "error": {
    "errors": [
      {
        "domain": "geolocation",
        "reason": "notFound",
        "message": "Not found"
      }
    ],
    "code": 404,
    "message": "Not found"
  }
}
```

### Examples

#### WiFi-only lookup

```bash
curl -X POST https://cloudflare-location-service.dreamingcodes.workers.dev/ \
  -H "Content-Type: application/json" \
  -d '{
    "wifiAccessPoints": [
      {"macAddress": "00:11:22:33:44:55"},
      {"macAddress": "66:77:88:99:aa:bb"}
    ]
  }'
```

#### Cell tower lookup

```bash
curl -X POST https://cloudflare-location-service.dreamingcodes.workers.dev/ \
  -H "Content-Type: application/json" \
  -d '{
    "radioType": "lte",
    "cellTowers": [
      {
        "mobileCountryCode": 310,
        "mobileNetworkCode": 410,
        "locationAreaCode": 12345,
        "cellId": 67890
      }
    ]
  }'
```

#### IP-only lookup

```bash
curl -X POST https://cloudflare-location-service.dreamingcodes.workers.dev/ \
  -H "Content-Type: application/json" \
  -d '{}'
```

## How It Works

1. **WiFi/Cell Data** - If WiFi BSSIDs or cell tower information is provided, the service queries Apple's WPS through the GrapheneOS privacy proxy
2. **Position Estimation** - When multiple data points are returned, positions are calculated using weighted averaging based on accuracy values
3. **IP Fallback** - If no network data is provided or the lookup fails, Cloudflare's edge-computed IP geolocation is used (with reduced accuracy)

### Accuracy Levels

| Source | Typical Accuracy |
|--------|------------------|
| WiFi (multiple APs) | 10-100 meters |
| Cell towers | 100-1000+ meters |
| IP (postal code available) | ~5 km |
| IP (city available) | ~20 km |
| IP (region available) | ~100 km |
| IP (country only) | ~500 km |

## Privacy

This service uses the [GrapheneOS Apple WPS proxy](https://github.com/nickcianciolo/apple-wps-proxy) to anonymize requests to Apple's location services. Your IP address is not sent to Apple - only the WiFi/cell data you provide.

When using IP fallback, location is determined by Cloudflare at the edge and no external requests are made.

## License

MIT

## Credits

- [GrapheneOS](https://grapheneos.org/) for the Apple WPS proxy
- [Cloudflare Workers](https://workers.cloudflare.com/) for the serverless runtime
