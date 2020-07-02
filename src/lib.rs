#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(target_device = "esp8266")]
mod esp8266 {
  use super::*;

  pub type BaseType_t = libc::c_long;
  pub type UBaseType_t = libc::c_ulong;

  pub type nvs_open_mode_t = nvs_open_mode;
  pub type nvs_handle_t = nvs_handle;

  pub const ESP_ERR_NVS_NEW_VERSION_FOUND: u32 = ESP_ERR_NVS_BASE + 0x10;

  pub type wifi_scan_threshold_t = wifi_fast_scan_threshold_t;
}

#[cfg(target_device = "esp8266")]
pub use esp8266::*;

#[cfg(all(feature = "defaults", target_device = "esp8266"))]
impl Default for wifi_init_config_t {
  fn default() -> Self {
    Self {
      event_handler: Some(esp_event_send),
      osi_funcs: core::ptr::null_mut(),
      qos_enable: WIFI_QOS_ENABLED as _,
      ampdu_rx_enable: WIFI_AMPDU_RX_ENABLED as _,
      rx_ampdu_buf_len: WIFI_AMPDU_RX_AMPDU_BUF_LEN,
      rx_ampdu_buf_num: WIFI_AMPDU_RX_AMPDU_BUF_NUM as _,
      amsdu_rx_enable: WIFI_AMSDU_RX_ENABLED as _,
      rx_ba_win: WIFI_AMPDU_RX_BA_WIN as _,
      rx_max_single_pkt_len: WIFI_RX_MAX_SINGLE_PKT_LEN,
      rx_buf_len: WIFI_HW_RX_BUFFER_LEN,
      rx_buf_num: CONFIG_ESP8266_WIFI_RX_BUFFER_NUM as _,
      left_continuous_rx_buf_num: CONFIG_ESP8266_WIFI_LEFT_CONTINUOUS_RX_BUFFER_NUM as _,
      rx_pkt_num: CONFIG_ESP8266_WIFI_RX_PKT_NUM as _,
      tx_buf_num: CONFIG_ESP8266_WIFI_TX_PKT_NUM as _,
      nvs_enable: WIFI_NVS_ENABLED as _,
      nano_enable: 0,
      magic: WIFI_INIT_CONFIG_MAGIC as _,
    }
  }
}

#[cfg(all(feature = "defaults", target_device = "esp32"))]
impl Default for wifi_init_config_t {
  fn default() -> Self {
    Self {
      event_handler: Some(esp_event_send_internal),
      osi_funcs: unsafe { &mut g_wifi_osi_funcs },
      wpa_crypto_funcs: unsafe { g_wifi_default_wpa_crypto_funcs },
      static_rx_buf_num: CONFIG_ESP32_WIFI_STATIC_RX_BUFFER_NUM as _,
      dynamic_rx_buf_num: CONFIG_ESP32_WIFI_DYNAMIC_RX_BUFFER_NUM as _,
      tx_buf_type: CONFIG_ESP32_WIFI_TX_BUFFER_TYPE as _,
      static_tx_buf_num: WIFI_STATIC_TX_BUFFER_NUM as _,
      dynamic_tx_buf_num: WIFI_DYNAMIC_TX_BUFFER_NUM as _,
      csi_enable: WIFI_CSI_ENABLED as _,
      ampdu_rx_enable: WIFI_AMPDU_RX_ENABLED as _,
      ampdu_tx_enable: WIFI_AMPDU_TX_ENABLED as _,
      nvs_enable: WIFI_NVS_ENABLED as _,
      nano_enable: WIFI_NANO_FORMAT_ENABLED as _,
      tx_ba_win: WIFI_DEFAULT_TX_BA_WIN as _,
      rx_ba_win: WIFI_DEFAULT_RX_BA_WIN as _,
      wifi_task_core_id: WIFI_TASK_CORE_ID as _,
      beacon_max_len: WIFI_SOFTAP_BEACON_MAX_LEN as _,
      mgmt_sbuf_num: WIFI_MGMT_SBUF_NUM as _,
      feature_caps: unsafe { g_wifi_feature_caps },
      magic: WIFI_INIT_CONFIG_MAGIC as _,
    }
  }
}
