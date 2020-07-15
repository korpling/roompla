use crate::{
    config::Settings,
    models::{Occupancy, Room},
};
use anyhow::Result;
use chrono::{prelude::*, Duration};
use diesel::{prelude::*, r2d2::ConnectionManager, sqlite::SqliteConnection};

pub fn to_csv(file: &str, weeks: u8, settings: Settings) -> Result<()> {
    // Calculate start and end date
    let end = Utc::now().date();
    let start = end - Duration::weeks(weeks as i64);

    info!("Loading database from {}", &settings.database.url);
    let manager = ConnectionManager::<SqliteConnection>::new(&settings.database.url);
    let db_pool = r2d2::Pool::builder().build(manager)?;

    let conn = db_pool.get()?;

    conn.transaction::<_, anyhow::Error, _>(|| {
        use crate::schema::occupancies::dsl as o_dsl;
        use crate::schema::rooms::dsl as r_dsl;

        let start: DateTime<Utc> = start.and_hms(0, 0, 0);
        let end: DateTime<Utc> = end.and_hms(23, 59, 59);

        // Get all overlapping events and join with the room information (to get the timezone)
        let result = o_dsl::occupancies
            .inner_join(r_dsl::rooms)
            .filter(o_dsl::start.ge(start.naive_utc()))
            .filter(o_dsl::end.le(end.naive_utc()))
            .order(o_dsl::start)
            .load::<(Occupancy, Room)>(&conn)?;

        let mut writer = csv::Writer::from_path(file)?;

        // Write header
        writer.write_record(&["id", "name", "contact", "room", "day"])?;

        for (event, room) in result {
            // Convert stored dates to local time and just output the day, not the time
            let tz = if let Some(tz) = &room.timezone {
                let parsed_tz: chrono_tz::Tz = tz.parse().expect("Invalid Time Zone");
                parsed_tz
            } else {
                chrono_tz::Tz::UTC
            };
            let day_utc: DateTime<Utc> = DateTime::from_utc(event.start, Utc);
            let day: Date<chrono_tz::Tz> = day_utc.with_timezone(&tz).date();

            writer.write_record(&[
                event.user_id,
                event.user_name,
                event.user_contact,
                event.room,
                day.to_string(),
            ])?;
        }
        info!("Finished export to {}", file);
        Ok(())
    })?;

    Ok(())
}
