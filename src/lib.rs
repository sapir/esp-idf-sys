#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod libc {
  pub enum c_void {}
  pub type c_uchar = u8;
  pub type c_schar = i8;
  pub type c_char = i8;
  pub type c_short = i16;
  pub type c_ushort = u16;
  pub type c_int = i32;
  pub type c_uint = u32;
  pub type c_long = i32;
  pub type c_ulong = u32;
  pub type c_longlong = i64;
  pub type c_ulonglong = u64;
}

impl Default for wifi_init_config_t {
  fn default() -> Self {
    Self {
      event_handler: Some(esp_event_send),
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
      magic: WIFI_INIT_CONFIG_MAGIC as _,
    }
  }
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
