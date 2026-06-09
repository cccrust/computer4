use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcpayConfig {
    pub merchant_id: String,
    pub hash_key: String,
    pub hash_iv: String,
    pub is_production: bool,
}

impl EcpayConfig {
    pub fn sandbox() -> Self {
        Self {
            merchant_id: "3002607".to_string(),
            hash_key: "pwFHCqoQJGmAcJKhWkMNyqkKIcSj8cKj".to_string(),
            hash_iv: "vYoehmxEN0mADb5D".to_string(),
            is_production: false,
        }
    }

    pub fn production(merchant_id: &str, hash_key: &str, hash_iv: &str) -> Self {
        Self {
            merchant_id: merchant_id.to_string(),
            hash_key: hash_key.to_string(),
            hash_iv: hash_iv.to_string(),
            is_production: true,
        }
    }

    pub fn api_url(&self) -> &str {
        if self.is_production {
            "https://payment.ecpay.com.tw/AioCheckOut/V5"
        } else {
            "https://payment-stage.ecpay.com.tw/AioCheckOut/V5"
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEcpayOrderParams {
    pub merchant_trade_no: String,
    pub total_amount: i64,
    pub trade_desc: String,
    pub item_name: String,
    pub return_url: String,
    pub client_back_url: String,
}

pub struct EcpayClient {
    config: EcpayConfig,
    client: reqwest::Client,
}

impl EcpayClient {
    pub fn new(config: EcpayConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn create_credit_card_order(
        &self,
        params: CreateEcpayOrderParams,
    ) -> Result<String, EcpayError> {
        let mut data = BTreeMap::new();
        data.insert("MerchantID".to_string(), self.config.merchant_id.clone());
        data.insert("MerchantTradeNo".to_string(), params.merchant_trade_no.clone());
        data.insert("MerchantTradeDate".to_string(), chrono::Local::now().format("%Y/%m/%d %H:%M:%S").to_string());
        data.insert("PaymentType".to_string(), "aio".to_string());
        data.insert("TotalAmount".to_string(), params.total_amount.to_string());
        data.insert("TradeDesc".to_string(), params.trade_desc.clone());
        data.insert("ItemName".to_string(), params.item_name.clone());
        data.insert("ReturnURL".to_string(), params.return_url.clone());
        data.insert("ClientBackURL".to_string(), params.client_back_url.clone());
        data.insert("OrderResultURL".to_string(), params.return_url.clone());
        data.insert("PaymentToken".to_string(), "".to_string());

        let check_mac_value = self.generate_check_mac_value(&data);

        let form_html = format!(
            r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Processing Payment...</title>
</head>
<body onload="document.ecpay.submit();">
    <form name="ecpay" method="POST" action="{}">
        <input type="hidden" name="MerchantID" value="{}">
        <input type="hidden" name="MerchantTradeNo" value="{}">
        <input type="hidden" name="MerchantTradeDate" value="{}">
        <input type="hidden" name="PaymentType" value="aio">
        <input type="hidden" name="TotalAmount" value="{}">
        <input type="hidden" name="TradeDesc" value="{}">
        <input type="hidden" name="ItemName" value="{}">
        <input type="hidden" name="ReturnURL" value="{}">
        <input type="hidden" name="ClientBackURL" value="{}">
        <input type="hidden" name="OrderResultURL" value="{}">
        <input type="hidden" name="PaymentToken" value="">
        <input type="hidden" name="CheckMacValue" value="{}">
    </form>
</body>
</html>"##,
            self.config.api_url(),
            self.config.merchant_id,
            params.merchant_trade_no,
            chrono::Local::now().format("%Y/%m/%d %H:%M:%S"),
            params.total_amount,
            html_escape(&params.trade_desc),
            html_escape(&params.item_name),
            html_escape(&params.return_url),
            html_escape(&params.client_back_url),
            html_escape(&params.return_url),
            check_mac_value
        );

        Ok(form_html)
    }

    fn generate_check_mac_value(&self, data: &BTreeMap<String, String>) -> String {
        let hash_key = &self.config.hash_key;
        let hash_iv = &self.config.hash_iv;

        let mut combined = String::new();
        for (key, value) in data.iter() {
            if key != "CheckMacValue" {
                combined.push_str(key);
                combined.push('=');
                combined.push_str(value);
                combined.push('&');
            }
        }
        combined.push_str("HashKey=");
        combined.push_str(hash_key);
        combined.push('&');
        combined.push_str("HashIV=");
        combined.push_str(hash_iv);

        let encoded = url_encode(&combined);
        sha256_encoded(&encoded).to_uppercase()
    }
}

fn sha256_encoded(s: &str) -> String {
    use sha2::Digest;
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    hex_encode(&result)
}

fn hex_encode(data: &[u8]) -> String {
    let mut s = String::with_capacity(data.len() * 2);
    for &b in data {
        s.push(hex_char(b >> 4));
        s.push(hex_char(b & 0xf));
    }
    s
}

fn hex_char(n: u8) -> char {
    if n < 10 {
        (b'0' + n) as char
    } else {
        (b'a' + n - 10) as char
    }
}

fn url_encode(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
            ' ' => result.push_str("%20"),
            _ => {
                let mut buf = [0u8; 4];
                let encoded = c.encode_utf8(&mut buf);
                for b in encoded.bytes() {
                    write!(&mut result, "%{:02X}", b).unwrap();
                }
            }
        }
    }
    result
}

fn html_escape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => result.push_str("&"),
            '<' => result.push_str("<"),
            '>' => result.push_str(">"),
            '"' => result.push_str("&#34;"),
            _ => result.push(c),
        }
    }
    result
}

impl fmt::Display for EcpayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EcpayError::InvalidParams(msg) => write!(f, "Invalid parameters: {}", msg),
            EcpayError::ApiError(msg) => write!(f, "ECPay API error: {}", msg),
            EcpayError::Network(e) => write!(f, "Network error: {}", e),
        }
    }
}

impl std::error::Error for EcpayError {}

#[derive(Debug)]
pub enum EcpayError {
    InvalidParams(String),
    ApiError(String),
    Network(String),
}

#[derive(Debug, Deserialize)]
pub struct EcpayReturnData {
    pub merchant_id: String,
    pub merchant_trade_no: String,
    pub store_id: String,
    pub rtn_code: i32,
    pub rtn_msg: String,
    pub trade_no: String,
    pub trade_amount: i64,
    pub payment_date: String,
    pub payment_type: String,
    pub payment_type_charge_fee: String,
    pub trade_date: String,
    pub simulate_payment: String,
    pub check_mac_value: String,
    pub payment_token: Option<String>,
    pub gwsr: Option<String>,
    pub processor_id: Option<String>,
    pub card4no: Option<String>,
    pub card6no: Option<String>,
}
