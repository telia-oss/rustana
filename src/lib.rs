use reqwest::Client;
use std::collections::HashMap;

pub mod rustana_types;
use rustana_types::RootInterface;
use rustana_types::Dashboard;
use rustana_types::Panels;
use rustana_types::MutateDashboardResponse;

use std::error::Error;

pub struct GrafanaClient {
    url: &'static str,
    token: &'static str,
}

fn get_dashboard_by_id(base_url: &str, token: &str, _id: &str) -> Result<RootInterface, Box<Error>>  {
    let url = format!("{}{}{}", base_url, "/api/dashboards/uid/", _id);
    let client = Client::new();
    let res: RootInterface = client.get(&url)
        .header("Authorization", format!("{} {}", "Bearer", token))
        .send()?
        .json()?;

    
    Ok(res)
}

fn create_or_update_dashboard(base_url: &str, token: &str, dashboard: RootInterface) -> Result<MutateDashboardResponse, Box<Error>> {
    let url = format!("{}{}", base_url, "/api/dashboards/db/");
    let client = Client::new();
    let res: MutateDashboardResponse = client.post(&url)
        .header("Authorization", format!("{} {}", "Bearer", token))
        .json(&dashboard)
        .send()?
        .json()?;

    Ok(res)
}

impl GrafanaClient {
    pub fn new(url: &'static str, token: &'static str) -> GrafanaClient {
        GrafanaClient {
            url: url,
            token: token,
        }
    }

    pub fn get_dashboard_by_id(&mut self, _id: &str) -> Result<RootInterface, Box<Error>> {
        match get_dashboard_by_id(self.url, self.token, _id) {
            Ok(res) => Ok(res),
            Err(e) => Err(e)
        }
    }

    pub fn update_dashboard_by_id(&mut self, _id: &str, panels: Vec<Panels>) -> Result<MutateDashboardResponse, Box<Error>> {
        let dashboard = get_dashboard_by_id(self.url, self.token, _id);
        match dashboard {
            Ok(res) => { 
                let new_dashboard: RootInterface = RootInterface {
                    meta: res.meta,
                    dashboard: Dashboard {
                        panels: panels,
                        ..res.dashboard
                    }
                };
                let updated_dashboard = create_or_update_dashboard(self.url, self.token, new_dashboard);
                match updated_dashboard {
                    Ok(res) => Ok(res),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }
}
