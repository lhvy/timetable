use self::bell_times::BellTimes;
use crate::types::{Day, DayOfWeek, Lesson, Timetable};
use chrono::Timelike;
use std::collections::BTreeSet;
mod bell_times;

pub(crate) fn gen_timetable_xml(t: &Timetable) -> String {
    let mut xml = String::new();
    xml += r#"<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd"><plist version="1.0"><dict>"#; // Header

    xml += "<key>Settings</key><dict><key>ColorSettings</key><dict>";
    let mut subjects = BTreeSet::new();

    let mut assembly_subject_class_codes = None;

    // Generate list of subjects for colors
    for day in t.week_a.days.values() {
        for lesson in &day.lessons {
            if let Lesson::Present {
                subject,
                class_code,
                subject_code,
                ..
            } = lesson
            {
                if subject.name == "Assembly" {
                    assembly_subject_class_codes = Some((subject_code, class_code));
                    continue;
                }

                subjects.insert(format!("{}.{}", subject_code, class_code));
            }
        }
    }

    let mut hue = 0;
    let step = 360 / (subjects.len() + 1);

    // Generate colors equal distance apart for each subject
    for subject in subjects {
        let srgb = gen_color(hue);
        xml += &format!(
            r#"<key>{}</key>
        <array>
            <real>{}</real>
            <real>{}</real>
            <real>{}</real>
        </array>"#,
            subject, srgb.r, srgb.g, srgb.b
        );
        hue += step;
    }

    // Set color for recess, lunch and assembly
    let oklab = tincture::Oklab {
        l: 0.5,
        a: 0.0,
        b: 0.0,
    };
    let lrgb: tincture::LinearRgb = tincture::convert(oklab);
    let srgb = tincture::Srgb::from(lrgb);

    let color = format!(
        "<array><real>{}</real><real>{}</real><real>{}</real></array>",
        srgb.r, srgb.g, srgb.b
    );

    xml += &format!("<key>Lunch</key>{color}<key>Recess</key>{color}<key>Free Period</key>{color}");

    if let Some((assembly_sc, assembly_cc)) = assembly_subject_class_codes {
        xml += &format!("<key>Assembly {}.{}</key>{color}", assembly_sc, assembly_cc);
    }

    // Add settings
    xml += r#"</dict>
    <key>NumberOfWeeks</key>
    <integer>2</integer>
    <key>SelectedWeek</key>
    <integer>1</integer>
    <key>SelectedWeekUpdateDate</key>
    <date>2021-11-10T22:27:02Z</date>
    <key>WeekendDaysAreActive</key>
    <false/>
    </dict>"#;

    xml += "<key>WeekEvents</key><array>";
    let bell_times = BellTimes::default();
    for (day_of_week, day) in &t.week_a.days {
        process_day(&mut xml, day_of_week, day, true, &bell_times);
    }
    for (day_of_week, day) in &t.week_b.days {
        process_day(&mut xml, day_of_week, day, false, &bell_times);
    }
    xml += "</array></dict></plist>";

    xml
}

fn gen_color(hue: usize) -> tincture::Srgb {
    let oklch = tincture::Oklch {
        l: if (95..=110).contains(&hue) {
            0.85
        } else {
            0.75
        },
        c: if (95..=110).contains(&hue) {
            0.14
        } else {
            0.125
        },
        h: tincture::Hue::from_degrees(hue as f32).unwrap(),
    };
    let oklab = tincture::Oklab::from(oklch);
    let lrgb: tincture::LinearRgb = tincture::convert(oklab);
    tincture::Srgb::from(lrgb)
}

fn process_day(
    xml: &mut String,
    day_of_week: &DayOfWeek,
    day: &Day,
    week_a: bool,
    bell_times: &BellTimes,
) {
    for (idx, lesson) in day.lessons.iter().enumerate() {
        if matches!(*lesson, Lesson::AbsentPeriod | Lesson::Lunch2) {
            continue;
        }
        let day = match day_of_week {
            DayOfWeek::Monday => &bell_times.monday,
            DayOfWeek::Tuesday => &bell_times.tuesday,
            DayOfWeek::Wednesday => &bell_times.wednesday,
            DayOfWeek::Thursday => &bell_times.thursday,
            DayOfWeek::Friday => &bell_times.friday,
        };
        let bell_times = &day.bell_times;

        *xml += &format!(
            r#"<dict>
        <key>dayNum</key>
        <integer>{}</integer>
        <key>time</key>
        <real>{}</real>
        <key>endTime</key>
        <real>{}</real>
        <key>title</key>
        <string>{}</string>{}
        <key>weekNum</key>
        <integer>{}</integer>
    </dict>"#,
            match day_of_week {
                DayOfWeek::Monday => 0,
                DayOfWeek::Tuesday => 1,
                DayOfWeek::Wednesday => 2,
                DayOfWeek::Thursday => 3,
                DayOfWeek::Friday => 4,
            },
            (bell_times[idx].start().hour() * 60 + bell_times[idx].start().minute()) * 60,
            if lesson == &Lesson::Lunch1 {
                (bell_times[idx + 1].end().hour() * 60 + bell_times[idx + 1].end().minute()) * 60
            } else {
                (bell_times[idx].end().hour() * 60 + bell_times[idx].end().minute()) * 60
            },
            match lesson {
                Lesson::Present {
                    subject,
                    subject_code,
                    class_code,
                    ..
                } if subject.name == "Assembly" => format!("Assembly {subject_code}.{class_code}"),
                Lesson::Present {
                    subject_code,
                    class_code,
                    ..
                } => format!("{subject_code}.{class_code}"),
                Lesson::FreePeriod => "Free Period".to_string(),
                Lesson::Recess => "Recess".to_string(),
                Lesson::Lunch1 => "Lunch".to_string(),
                Lesson::AbsentPeriod | Lesson::Lunch2 => unreachable!(),
            },
            match lesson {
                Lesson::Present {
                    room: Some(room), ..
                } => format!("<key>info</key><string>{}</string>", room),
                _ => String::new(),
            },
            if week_a { 0 } else { 1 }
        );
    }
}

#[cfg(test)]
#[test]
fn test_all_colours() {
    use tincture::ColorSpace;
    for hue in 0..=360 {
        assert!(gen_color(hue).in_bounds());
    }
}
