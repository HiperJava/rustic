use diesel::prelude::*;
use diesel::sql_types::Date;
use crate::config::error_handler::CustomError;
use crate::config::schema::sensors;
use crate::config::db;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "sensors"]
pub struct Sensor {
    pub id: String,
    pub location: String,
    // pub logs: [Log],
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Sensors {
    pub id: String,
    pub location: String,
//    pub logs: [Log],
}

//#[derive(Serialize, Deserialize)]
//pub struct Log {
//    pub time: Date,
//    pub alertType: AlertType,
//}

enum AlertType {
    TEST,
    ALERT,
    ERROR,
}

// 2D array: 1rows x 2cols
//[[f64; 2]; 1],
#[derive(Serialize, Deserialize,Queryable)]
pub struct Dimensions {
    pub lat: f64,
    pub long: f64,
}


impl Sensors {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let sensorItems = sensors::table.load::<Sensors>(&conn)?;
        Ok(sensorItems)
    }

       pub fn find(id: String) -> Result<Self, CustomError> {
          let conn = db::connection()?;
          let sensor = sensors::table.filter(sensors::id.eq(id)).first(&conn)?;
          Ok(sensor)
      }

      pub fn create(sensor: Sensor) -> Result<Self, CustomError> {
          let conn = db::connection()?;
          let sensor = Sensor::from(sensor);
          let sensor = diesel::insert_into(sensors::table)
              .values(sensor)
              .get_result(&conn)?;
          Ok(sensor)
      }

      pub fn update(id: String, sensor: Sensor) -> Result<Self, CustomError> {
          let conn = db::connection()?;
          let sensor = diesel::update(sensors::table)
              .filter(sensors::id.eq(id))
              .set(sensor)
              .get_result(&conn)?;
          Ok(sensor)
      }

      pub fn delete(id: String) -> Result<usize, CustomError> {
          let conn = db::connection()?;
          let res = diesel::delete(sensors::table.filter(sensors::id.eq(id))).execute(&conn)?;
          Ok(res)
      }
}

impl Sensor {
    fn from(sensor: Sensor) -> Sensor {
        Sensor {
            id: sensor.id.to_string(),
            location: sensor.location,
            //  logs: sensor.logs,
        }
    }
}
