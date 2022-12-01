use chrono::{Duration, NaiveTime};
use itertools::Itertools;

pub(crate) struct BellTimes {
    pub(crate) monday: Day,
    pub(crate) tuesday: Day,
    pub(crate) wednesday: Day,
    pub(crate) thursday: Day,
    pub(crate) friday: Day,
}

impl BellTime {
    pub(crate) fn start(&self) -> NaiveTime {
        self.start
    }

    pub(crate) fn end(&self) -> NaiveTime {
        self.start + self.length
    }
}

impl Default for BellTimes {
    fn default() -> Self {
        let not_wednesday = &[
            NaiveTime::from_hms_opt(7, 55, 0).unwrap(),  // 0
            NaiveTime::from_hms_opt(8, 55, 0).unwrap(),  // 1
            NaiveTime::from_hms_opt(9, 50, 0).unwrap(),  // 2
            NaiveTime::from_hms_opt(10, 45, 0).unwrap(), // R
            NaiveTime::from_hms_opt(11, 5, 0).unwrap(),  // 3
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),  // 4
            NaiveTime::from_hms_opt(12, 55, 0).unwrap(), // L1
            NaiveTime::from_hms_opt(13, 15, 0).unwrap(), // L2
            NaiveTime::from_hms_opt(13, 35, 0).unwrap(), // 5
            NaiveTime::from_hms_opt(14, 30, 0).unwrap(), // 6
            NaiveTime::from_hms_opt(15, 25, 0).unwrap(), // 7
            NaiveTime::from_hms_opt(16, 20, 0).unwrap(), // 8
            NaiveTime::from_hms_opt(17, 15, 0).unwrap(), // End of 8
        ];

        let wednesday = &[
            NaiveTime::from_hms_opt(7, 55, 0).unwrap(),  // 0
            NaiveTime::from_hms_opt(8, 55, 0).unwrap(),  // 1
            NaiveTime::from_hms_opt(9, 50, 0).unwrap(),  // 2
            NaiveTime::from_hms_opt(10, 45, 0).unwrap(), // A
            NaiveTime::from_hms_opt(11, 5, 0).unwrap(),  // R
            NaiveTime::from_hms_opt(11, 25, 0).unwrap(), // 3
            NaiveTime::from_hms_opt(12, 20, 0).unwrap(), // L1
            NaiveTime::from_hms_opt(12, 40, 0).unwrap(), // L2
            NaiveTime::from_hms_opt(13, 0, 0).unwrap(),  // 4
            NaiveTime::from_hms_opt(13, 55, 0).unwrap(), // 5
            NaiveTime::from_hms_opt(14, 50, 0).unwrap(), // 6
            NaiveTime::from_hms_opt(15, 45, 0).unwrap(), // 7
            NaiveTime::from_hms_opt(16, 40, 0).unwrap(), // End of 7
        ];

        Self {
            monday: Day::new(not_wednesday),
            tuesday: Day::new(not_wednesday),
            wednesday: Day::new(wednesday),
            thursday: Day::new(not_wednesday),
            friday: Day::new(not_wednesday),
        }
    }
}

pub(crate) struct BellTime {
    start: NaiveTime,
    length: Duration,
}

pub(crate) struct Day {
    pub(crate) bell_times: Vec<BellTime>,
}

impl Day {
    fn new(times: &[NaiveTime]) -> Self {
        let mut bell_times = Vec::with_capacity(times.len());

        for (start, end) in times.iter().cloned().tuple_windows() {
            bell_times.push(BellTime {
                start,
                length: end - start,
            });
        }

        Self { bell_times }
    }
}
