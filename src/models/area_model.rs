use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use tokio_postgres::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleProvince {
    id: u32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleCity{
    id: u32,
    name: String, 
    pid: u32,
    province: String,
    lgt: f32,
    lat: f32,
}

/*
impl From<Row> for SimpleCity{
    fn from(row: Row) -> Self {
        Self{
            id: row.get("")
        }
    }
}*/

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


pub async fn list_cities_by_province_id(client: & Client, province_id: i32) -> Result<Vec<SimpleCity>, Error>{
    let mut cities: Vec<SimpleCity> = Vec::new();
    let rows = 
        client.query("SELECT DISTINCT city_id, city, province_id, province, longitude, latitude FROM public.base_region WHERE province_id = $1 AND level = 2 ORDER BY city_id", &[&province_id])
        .await?;
    for row in &rows{
        let id: i32 = row.get("city_id");
        let name: String = row.get("city");
        let province_id: i32 = row.get("province_id");
        let province: String = row.get("province");
        let lgt: f32 = row.get("longitude");
        let lat: f32 = row.get("latitude");
        cities.push(SimpleCity{id: id as u32, name, pid: province_id as u32, province, lgt, lat,});
    }
    Ok(cities)
}