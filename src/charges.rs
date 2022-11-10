use crate::ZebedeeClient;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct InvoiceData {
    request: String,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChargesData {
    id: String, //uuid::Uuid,
    unit: String,
    amount: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
    #[serde(rename = "internalId")]
    internal_id: String,
    #[serde(rename = "callbackUrl")]
    callback_url: String,
    description: String,
    #[serde(rename = "expiresAt")]
    expires_at: DateTime<Utc>,
    #[serde(rename = "confirmedAt")]
    confirmed_at: Option<DateTime<Utc>>,
    status: String,
    invoice: InvoiceData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllChargesRes {
    success: bool,
    data: Vec<ChargesData>,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChargesRes {
    success: bool,
    data: ChargesData,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Charge {
    #[serde(rename = "expiresIn")]
    pub expires_in: u32,
    pub amount: String,
    pub description: String,
    #[serde(rename = "internalId")]
    pub internal_id: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
}

impl Default for Charge {
    fn default() -> Charge {
        Charge {
            expires_in: 300,
            amount: String::from("0"),
            description: String::from("using zebedee rust sdk"),
            internal_id: String::from(""),
            callback_url: String::from(""),
        }
    }
}

#[tokio::main]
pub async fn create_charge(
    client: ZebedeeClient,
    charge: Charge,
) -> Result<ChargesRes, anyhow::Error> {
    let resp = client
        .reqw_cli
        .post("https://api.zebedee.io/v0/charges")
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .json(&charge)
        .send()
        .await?;

    let status_code = resp.status();

    let resp_text = resp.text().await?;

    match status_code {
        reqwest::StatusCode::OK => dbg!("OK status:"),
        s => {
            return Err(anyhow::anyhow!(
                "Error: status {}, message: {}",
                s,
                resp_text.clone()
            ));
        }
    };

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2: ChargesRes = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text.clone(),
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[tokio::main]
pub async fn get_charges(client: ZebedeeClient) -> Result<AllChargesRes, anyhow::Error> {
    let resp = client
        .reqw_cli
        .get("https://api.zebedee.io/v0/charges")
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();
    let resp_text = resp.text().await?;

    match status_code {
        reqwest::StatusCode::OK => dbg!("OK status:"),
        s => {
            return Err(anyhow::anyhow!(
                "Error: status {}, message: {}",
                s,
                resp_text.clone()
            ));
        }
    };

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2: AllChargesRes = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text.clone(),
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[tokio::main]
pub async fn get_charge(
    client: ZebedeeClient,
    charge_id: String,
) -> Result<ChargesRes, anyhow::Error> {
    let url = format!("https://api.zebedee.io/v0/charges/{}", charge_id);
    let resp = client
        .reqw_cli
        .get(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();

    let resp_text = resp.text().await?;

    match status_code {
        reqwest::StatusCode::OK => dbg!("OK status:"),
        s => {
            return Err(anyhow::anyhow!(
                "Error: status {}, message: {}, charge_id: {}, url: {}",
                s,
                resp_text.clone(),
                charge_id,
                &url,
            ));
        }
    };

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2: ChargesRes = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\nstatus code: {}\ncharge_id: {}\n url: {}",
                e,
                resp_text.clone(),
                status_code,
                charge_id,
                &url,
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_create_charge() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);
        let charge = Charge {
            amount: String::from("1000"),
            ..Default::default()
        };

        let r = create_charge(zebedee_client, charge).unwrap();
        assert_eq!(r.success, true);
    }
    #[test]
    fn test_get_charges() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let r = get_charges(zebedee_client).unwrap();
        assert_eq!(r.success, true);
    }
    #[test]
    fn test_get_charge() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zebedee_client = ZebedeeClient::new(apikey);

        let charge = Charge {
            amount: String::from("1000"),
            ..Default::default()
        };

        let r = create_charge(zebedee_client.clone(), charge).unwrap();
        let r2 = get_charge(zebedee_client, r.data.id).unwrap();
        assert_eq!(r2.success, true);
    }
}
