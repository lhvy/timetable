use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct StudentIdentity {
    pub(crate) first: String,
    pub(crate) last: String,
    pub(crate) id: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Timetable {
    pub(crate) week_a: Week,
    pub(crate) week_b: Week,
}

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Week {
    pub(crate) days: BTreeMap<DayOfWeek, Day>,
}

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Day {
    pub(crate) lessons: Vec<Lesson>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Lesson {
    Present {
        subject: Subject,
        teacher: Option<Teacher>,
        room: Option<String>,
        subject_code: String,
        class_code: String,
    },
    FreePeriod,
    /// Free period before the first class or after the last class
    AbsentPeriod,
    Recess,
    Lunch1,
    Lunch2,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct Subject {
    pub(crate) name: String,
    pub(crate) faculty: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Teacher {
    pub(crate) first_name: Option<String>,
    pub(crate) last_name: String,
}

impl Teacher {
    pub(crate) fn name(&self) -> String {
        let mut name = String::new();

        if let Some(ref first_name) = self.first_name {
            name.push_str(first_name);
            name.push(' ');
        }

        name.push_str(&self.last_name);

        name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[derive(Debug)]
pub(crate) struct DayOfWeekIter(Option<DayOfWeek>);

impl DayOfWeek {
    pub(crate) fn iter() -> DayOfWeekIter {
        DayOfWeekIter(None)
    }
}

impl Iterator for DayOfWeekIter {
    type Item = DayOfWeek;

    fn next(&mut self) -> Option<Self::Item> {
        let new_day = match self.0 {
            None => DayOfWeek::Monday,
            Some(DayOfWeek::Monday) => DayOfWeek::Tuesday,
            Some(DayOfWeek::Tuesday) => DayOfWeek::Wednesday,
            Some(DayOfWeek::Wednesday) => DayOfWeek::Thursday,
            Some(DayOfWeek::Thursday) => DayOfWeek::Friday,
            Some(DayOfWeek::Friday) => DayOfWeek::Monday,
        };
        *self = Self(Some(new_day));

        Some(new_day)
    }
}
