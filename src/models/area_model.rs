use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use tokio_postgres::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleProvince {
    id: u32,
    name: String,
}

pub async fn list_all_simple_provinces(client: &Client) -> Result<Vec<SimpleProvince>, Error> {
    //todo!()
    let mut simple_privinces: Vec<SimpleProvince> = Vec::new();
    let rows = 
        client.query("SELECT DISTINCT province_id, province FROM public.base_region WHERE level = 1 ORDER BY province_id",&[])
        .await?;
    for row in &rows{
        let id: i32 = row.get("province_id");
        let name: String = row.get("province");
        simple_privinces.push(SimpleProvince{id: id as u32, name,})
    }
    Ok(simple_privinces)
}
