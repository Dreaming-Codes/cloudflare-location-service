// Apple WPS Protobuf messages (manually defined based on GrapheneOS proto)
use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct AlsLocationRequest {
    #[prost(message, repeated, tag = "1")]
    pub gsm_cell_towers: Vec<GsmCellTower>,
    #[prost(message, repeated, tag = "2")]
    pub wireless_aps: Vec<WirelessAp>,
    #[prost(int32, optional, tag = "3")]
    pub number_of_surrounding_gsm_cells: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub number_of_surrounding_wifis: Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub app_bundle_id: Option<String>,
    #[prost(message, repeated, tag = "25")]
    pub lte_cell_towers: Vec<LteCellTower>,
    #[prost(int32, optional, tag = "26")]
    pub number_of_surrounding_lte_cells: Option<i32>,
    #[prost(message, repeated, tag = "27")]
    pub scdma_cell_towers: Vec<ScdmaCellTower>,
    #[prost(int32, optional, tag = "28")]
    pub number_of_surrounding_scdma_cells: Option<i32>,
    #[prost(message, repeated, tag = "29")]
    pub nr5g_cell_towers: Vec<Nr5gCellTower>,
    #[prost(int32, optional, tag = "30")]
    pub number_of_surrounding_nr5g_cells: Option<i32>,
    #[prost(enumeration = "WifiBand", repeated, tag = "31")]
    pub surrounding_wifi_bands: Vec<i32>,
    #[prost(enumeration = "WifiAltitudeScale", optional, tag = "32")]
    pub wifi_altitude_scale: Option<i32>,
    #[prost(message, optional, tag = "33")]
    pub meta: Option<AlsMeta>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AlsMeta {
    #[prost(string, optional, tag = "1")]
    pub software_build: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub product_id: Option<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AlsLocationResponse {
    #[prost(message, repeated, tag = "1")]
    pub gsm_cell_towers: Vec<GsmCellTower>,
    #[prost(message, repeated, tag = "2")]
    pub wireless_aps: Vec<WirelessAp>,
    #[prost(message, repeated, tag = "22")]
    pub lte_cell_towers: Vec<LteCellTower>,
    #[prost(message, repeated, tag = "23")]
    pub scdma_cell_towers: Vec<ScdmaCellTower>,
    #[prost(message, repeated, tag = "24")]
    pub nr5g_cell_towers: Vec<Nr5gCellTower>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AlsLocation {
    #[prost(int64, tag = "1")]
    pub latitude: i64,
    #[prost(int64, tag = "2")]
    pub longitude: i64,
    #[prost(int32, tag = "3")]
    pub accuracy: i32,
    #[prost(int32, optional, tag = "4")]
    pub location_type: Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub altitude: Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub vertical_accuracy: Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub confidence: Option<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct GsmCellTower {
    #[prost(int32, tag = "1")]
    pub mcc: i32,
    #[prost(int32, tag = "2")]
    pub mnc: i32,
    #[prost(int32, tag = "3")]
    pub cell_id: i32,
    #[prost(int32, tag = "4")]
    pub lac_id: i32,
    #[prost(message, optional, tag = "5")]
    pub location: Option<AlsLocation>,
}

#[derive(Clone, PartialEq, Message)]
pub struct WirelessAp {
    #[prost(string, tag = "1")]
    pub mac_id: String,
    #[prost(message, optional, tag = "2")]
    pub location: Option<AlsLocation>,
    #[prost(uint32, optional, tag = "21")]
    pub channel: Option<u32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct LteCellTower {
    #[prost(int32, optional, tag = "1")]
    pub mcc: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub mnc: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cell_id: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub tac_id: Option<i32>,
    #[prost(message, optional, tag = "5")]
    pub location: Option<AlsLocation>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ScdmaCellTower {
    #[prost(int32, tag = "1")]
    pub mcc: i32,
    #[prost(int32, tag = "2")]
    pub mnc: i32,
    #[prost(int32, tag = "3")]
    pub cell_id: i32,
    #[prost(int32, tag = "4")]
    pub lac_id: i32,
    #[prost(message, optional, tag = "5")]
    pub location: Option<AlsLocation>,
}

#[derive(Clone, PartialEq, Message)]
pub struct Nr5gCellTower {
    #[prost(int32, optional, tag = "1")]
    pub mcc: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub mnc: Option<i32>,
    #[prost(int64, optional, tag = "3")]
    pub cell_id: Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub tac_id: Option<i32>,
    #[prost(message, optional, tag = "5")]
    pub location: Option<AlsLocation>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[repr(i32)]
pub enum WifiBand {
    #[default]
    Unknown = 0,
    K2Dot4Ghz = 1,
    K5Ghz = 2,
}

impl From<i32> for WifiBand {
    fn from(v: i32) -> Self {
        match v {
            1 => WifiBand::K2Dot4Ghz,
            2 => WifiBand::K5Ghz,
            _ => WifiBand::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[repr(i32)]
pub enum WifiAltitudeScale {
    #[default]
    Unknown = 0,
    None = 1,
    TenToThe2 = 2,
}

impl From<i32> for WifiAltitudeScale {
    fn from(v: i32) -> Self {
        match v {
            1 => WifiAltitudeScale::None,
            2 => WifiAltitudeScale::TenToThe2,
            _ => WifiAltitudeScale::Unknown,
        }
    }
}

impl AlsLocation {
    /// Returns None if the location is invalid (Apple uses -18000000000 for "not found")
    pub fn to_coordinates(&self) -> Option<(f64, f64, i32)> {
        if self.latitude == -18000000000 {
            return None;
        }
        let lat = self.latitude as f64 * 0.00000001;
        let lng = self.longitude as f64 * 0.00000001;
        Some((lat, lng, self.accuracy))
    }
}

impl AlsLocationRequest {
    pub fn new_wifi_request(bssids: &[String], max_additional: i32) -> Self {
        let wireless_aps: Vec<WirelessAp> = bssids
            .iter()
            .map(|bssid| WirelessAp {
                mac_id: bssid.clone(),
                location: None,
                channel: None,
            })
            .collect();

        AlsLocationRequest {
            wireless_aps,
            number_of_surrounding_wifis: Some(max_additional.max(1)),
            surrounding_wifi_bands: vec![WifiBand::K2Dot4Ghz as i32, WifiBand::K5Ghz as i32],
            wifi_altitude_scale: Some(WifiAltitudeScale::TenToThe2 as i32),
            meta: Some(AlsMeta {
                software_build: Some("macOS15.4/24E248".to_string()),
                product_id: Some("arm64".to_string()),
            }),
            ..Default::default()
        }
    }

    pub fn new_cell_request(cells: Vec<CellRequest>, max_additional: i32) -> Self {
        let mut request = AlsLocationRequest {
            meta: Some(AlsMeta {
                software_build: Some("macOS15.4/24E248".to_string()),
                product_id: Some("arm64".to_string()),
            }),
            ..Default::default()
        };

        let mut gsm_count = 0;
        let mut lte_count = 0;
        let mut wcdma_count = 0;

        for cell in &cells {
            match cell.radio_type.as_str() {
                "gsm" => {
                    request.gsm_cell_towers.push(GsmCellTower {
                        mcc: cell.mcc,
                        mnc: cell.mnc,
                        lac_id: cell.lac,
                        cell_id: cell.cell_id,
                        location: None,
                    });
                    gsm_count += 1;
                }
                "lte" => {
                    request.lte_cell_towers.push(LteCellTower {
                        mcc: Some(cell.mcc),
                        mnc: Some(cell.mnc),
                        tac_id: Some(cell.lac),
                        cell_id: Some(cell.cell_id),
                        location: None,
                    });
                    lte_count += 1;
                }
                "wcdma" => {
                    request.scdma_cell_towers.push(ScdmaCellTower {
                        mcc: cell.mcc,
                        mnc: cell.mnc,
                        lac_id: cell.lac,
                        cell_id: cell.cell_id,
                        location: None,
                    });
                    wcdma_count += 1;
                }
                _ => {}
            }
        }

        let total = gsm_count + lte_count + wcdma_count;
        if total > 0 {
            if gsm_count > 0 {
                request.number_of_surrounding_gsm_cells =
                    Some((max_additional * gsm_count / total).max(1));
            }
            if lte_count > 0 {
                request.number_of_surrounding_lte_cells =
                    Some((max_additional * lte_count / total).max(1));
            }
            if wcdma_count > 0 {
                request.number_of_surrounding_scdma_cells =
                    Some((max_additional * wcdma_count / total).max(1));
            }
        }

        request
    }

    pub fn new_combined_request(
        bssids: &[String],
        cells: Vec<CellRequest>,
        max_wifi_additional: i32,
        max_cell_additional: i32,
    ) -> Self {
        let mut request = Self::new_wifi_request(bssids, max_wifi_additional);
        let cell_request = Self::new_cell_request(cells, max_cell_additional);

        request.gsm_cell_towers = cell_request.gsm_cell_towers;
        request.lte_cell_towers = cell_request.lte_cell_towers;
        request.scdma_cell_towers = cell_request.scdma_cell_towers;
        request.nr5g_cell_towers = cell_request.nr5g_cell_towers;
        request.number_of_surrounding_gsm_cells = cell_request.number_of_surrounding_gsm_cells;
        request.number_of_surrounding_lte_cells = cell_request.number_of_surrounding_lte_cells;
        request.number_of_surrounding_scdma_cells = cell_request.number_of_surrounding_scdma_cells;
        request.number_of_surrounding_nr5g_cells = cell_request.number_of_surrounding_nr5g_cells;

        request
    }
}

pub struct CellRequest {
    pub radio_type: String,
    pub mcc: i32,
    pub mnc: i32,
    pub lac: i32,
    pub cell_id: i32,
}
