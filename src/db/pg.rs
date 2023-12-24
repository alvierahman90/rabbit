use super::{Error, Storage};
use crate::{models::*, schema};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub struct PgDb {
    conn: PgConnection,
}

impl Storage for PgDb {
    fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self {
            conn: PgConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
        }
    }

    fn add_user(&mut self, user: NewUser) -> Result<i32, Error> {
        use schema::users;
        let ret = diesel::insert_into(users::table)
            .values(&user)
            .returning(users::dsl::id)
            .get_result(&mut self.conn);

        match ret {
            Ok(val) => Ok(val),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn add_category(&mut self, category: NewCategory) -> Result<i32, Error> {
        use schema::categories;
        let ret = diesel::insert_into(categories::table)
            .values(&category)
            .returning(categories::dsl::id)
            .get_result(&mut self.conn);

        match ret {
            Ok(val) => Ok(val),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn add_series(&mut self, series: NewSeries) -> Result<i32, Error> {
        use schema::series;
        let ret = diesel::insert_into(series::table)
            .values(&series)
            .returning(series::dsl::id)
            .get_result(&mut self.conn);

        match ret {
            Ok(val) => Ok(val),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn add_series_point(&mut self, series_point: NewSeriesPoint) -> Result<i32, Error> {
        use schema::series_points;
        let ret = diesel::insert_into(series_points::table)
            .values(&series_point)
            .returning(series_points::dsl::id)
            .get_result(&mut self.conn);

        match ret {
            Ok(val) => Ok(val),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn get_categories(&mut self, filter: CategoryFilter) -> Result<Vec<Category>, Error> {
        use schema::categories;

        let mut query = categories::table.into_boxed();

        if let Some(val) = filter.id {
            query = query.filter(categories::id.eq(val));
        }

        if let Some(val) = filter.name {
            query = query.filter(categories::name.eq(val));
        }

        if let Some(val) = filter.user_id {
            query = query.filter(categories::user_id.eq(val));
        }

        match query.select(Category::as_select()).load(&mut self.conn) {
            Ok(q) => Ok(q),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn get_series(&mut self, filter: SeriesFilter) -> Result<Vec<Series>, Error> {
        use schema::series;

        let mut query = series::table.into_boxed();

        if let Some(val) = filter.id {
            query = query.filter(series::id.eq(val));
        }

        if let Some(val) = filter.name {
            query = query.filter(series::name.eq(val));
        }

        if let Some(val) = filter.repeat {
            query = query.filter(series::repeat.eq(val));
        }

        if let Some(val) = filter.good {
            query = query.filter(series::good.eq(val));
        }

        if let Some(val) = filter.category_id {
            query = query.filter(series::category_id.eq(val));
        }

        match query.select(Series::as_select()).load(&mut self.conn) {
            Ok(q) => Ok(q),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn get_series_points(&mut self, filter: SeriesPointFilter) -> Result<Vec<SeriesPoint>, Error> {
        use schema::series_points;

        let mut query = series_points::table.into_boxed();

        if let Some(val) = filter.id {
            query = query.filter(series_points::id.eq(val));
        }

        if let Some(val) = filter.timestamp_millis {
            match chrono::NaiveDateTime::from_timestamp_millis(val) {
                Some(val) => query = query.filter(series_points::timestamp.eq(val)),
                _ => return Err(Error::Parsing("Failed to parse timestamp".to_owned())),
            }
        }

        if let Some(val) = filter.value {
            query = query.filter(series_points::value.eq(val));
        }

        if let Some(val) = filter.series_id {
            query = query.filter(series_points::series_id.eq(val));
        }

        match query.select(SeriesPoint::as_select()).load(&mut self.conn) {
            Ok(q) => Ok(q),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn get_users(&mut self, filter: UserFilter) -> Result<Vec<User>, Error> {
        use schema::users;

        let mut query = users::table.into_boxed();

        if let Some(val) = filter.id {
            query = query.filter(users::id.eq(val));
        }

        if let Some(val) = filter.name {
            query = query.filter(users::name.eq(val));
        }

        if let Some(val) = filter.email {
            query = query.filter(users::email.eq(val));
        }

        match query.select(User::as_select()).load(&mut self.conn) {
            Ok(q) => Ok(q),
            Err(e) => Err(Error::from(e)),
        }
    }
}
