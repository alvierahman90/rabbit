use super::*;

pub struct JsonDb {
    pub value: String,
}

/// JsonDb is single user.
impl JsonDb {
    fn load(&self) -> Result<Vec<Category>, Error> {
        match serde_json::from_str::<Vec<Category>>(&self.value) {
            Ok(d) => Ok(d),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn save(&mut self, data: Vec<Category>) -> Result<(), Error> {
        let data = serde_json::to_string_pretty(&data);
        match data {
            Ok(d) => {
                self.value = d;
                Ok(())
            }
            Err(e) => Err(Error::from(e)),
        }
    }
}

impl Storage for JsonDb {
    fn new() -> Self {
        JsonDb {
            value: String::from("[]"),
        }
    }

    fn add_category(&mut self, category: NewCategory) -> Result<i32, Error> {
        let mut max_id = 1;
        let mut data = self.load()?;
        for cat in data.iter().clone() {
            if cat.id > max_id {
                max_id = cat.id
            }
        }
        let created = Category::new(max_id + 1, category);
        data.push(created);
        self.save(data)?;

        Ok(max_id + 1)
    }

    fn add_series(&mut self, category_id: i32, series: NewSeries) -> Result<i32, Error> {
        let mut max_id = 1;
        let mut cat: Option<&mut Category> = None;

        let mut data = self.load()?;
        for cat_opt in data.iter_mut() {
            if cat_opt.id == category_id {
                cat = Some(cat_opt);
                break;
            }
        }

        if let None = cat {
            return Err(Error::Generic("Unable to find category".to_owned()));
        }
        let cat = cat.unwrap();

        for series in cat.series.iter().clone() {
            if series.id > max_id {
                max_id = series.id;
            }
        }

        let created = Series::new(max_id + 1, series);
        cat.series.push(created);
        self.save(data)?;

        Ok(max_id + 1)
    }

    fn add_series_point(
        &mut self,
        category_id: i32,
        series_id: i32,
        series_point: NewSeriesPoint,
    ) -> Result<i32, Error> {
        let mut max_id = 1;
        let mut cat: Option<&mut Category> = None;
        let mut series: Option<&mut Series> = None;

        let mut data = self.load()?;
        for cat_opt in data.iter_mut() {
            if cat_opt.id == category_id {
                cat = Some(cat_opt);
                break;
            }
        }

        if let None = cat {
            return Err(Error::Generic("Unable to find category".to_owned()));
        }
        let cat = cat.unwrap();

        for series_opt in cat.series.iter_mut() {
            if series_opt.id == series_id {
                series = Some(series_opt);
                break;
            }
        }

        if let None = series {
            return Err(Error::Generic("Unable to find series".to_owned()));
        }
        let series: &mut Series = series.unwrap();

        for point in series.points.iter().clone() {
            if point.id > max_id {
                max_id = point.id;
            }
        }

        let created = SeriesPoint::new(max_id + 1, series_point);
        series.points.push(created);
        self.save(data)?;

        Ok(max_id + 1)
    }

    fn get_category(&self, id: i32) -> Option<Category> {
        let data = self.load().ok()?;

        for cat_opt in data {
            if cat_opt.id == id {
                return Some(cat_opt);
            }
        }

        None
    }

    fn get_series(&self, category_id: i32, series_id: i32) -> Option<Series> {
        let data = self.load().ok()?;
        let mut cat: Option<Category> = None;

        for cat_opt in data {
            if cat_opt.id == category_id {
                cat = Some(cat_opt);
            }
        }

        let cat = cat.unwrap();

        for series_opt in cat.series {
            if series_opt.id == series_id {
                return Some(series_opt);
            }
        }

        None
    }
}
