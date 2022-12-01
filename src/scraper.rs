use crate::types::{DayOfWeek, Lesson, StudentIdentity, Subject, Teacher, Timetable, Week};
use reqwest::blocking::Client;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name};
use std::mem;

pub(crate) fn scrape_student_identities(
    client: &Client,
    host: &str,
    timetable_uuid: &str,
) -> anyhow::Result<Vec<StudentIdentity>> {
    let document = Document::from(
        client
            .get(format!("{}/{}", host, timetable_uuid))
            .send()?
            .text()?
            .as_str(),
    );

    let student_entries = document.find(Attr("id", "student-entries")).next().unwrap();
    let student_identities = student_entries
        .find(Name("a"))
        .map(|n| scrape_student_identity(n, timetable_uuid))
        .collect();

    Ok(student_identities)
}

fn scrape_student_identity(node: Node<'_>, timetable_uuid: &str) -> StudentIdentity {
    let name = node.find(Name("span")).next().unwrap().text();
    let (first, last) = name.split_once(' ').unwrap();
    let id = node
        .attr("href")
        .unwrap()
        .strip_prefix(&format!("/{}/", timetable_uuid))
        .unwrap();

    StudentIdentity {
        first: first.to_string(),
        last: last.to_string(),
        id: id.to_string(),
    }
}

pub(crate) fn scrape_timetable_page(text: &str) -> Timetable {
    let document = Document::from(text);

    let mut tables = document.find(Class("table"));

    let week_a = tables.next().unwrap();
    let week_b = tables.next().unwrap();

    Timetable {
        week_a: scrape_week(week_a),
        week_b: scrape_week(week_b),
    }
}

fn scrape_week(node: Node<'_>) -> Week {
    let mut week = Week::default();

    const DAYS_OF_WEEK: [DayOfWeek; 5] = [
        DayOfWeek::Monday,
        DayOfWeek::Tuesday,
        DayOfWeek::Wednesday,
        DayOfWeek::Thursday,
        DayOfWeek::Friday,
    ];

    // Lessons are stored in table in rows with each column representing a day of the week.
    // Go across each row and add the lessons with the correct day of the week.
    for (i, lesson) in node.find(Name("td")).enumerate() {
        let lesson = scrape_lesson(lesson);

        let day_of_week = DAYS_OF_WEEK[i % 5];
        let day = week.days.entry(day_of_week).or_default();
        day.lessons.push(lesson);
    }

    for (day_of_week, day) in &mut week.days {
        // 0, 1, 2, Assembly, Break/PC, R, 3, 4, L1, L2, 5, 6, 7, 8

        // Break/PC row is always empty;
        // it carries no information so we remove it
        assert_eq!(day.lessons.remove(4), Lesson::FreePeriod);
        // Ends up with: 0, 1, 2, Assembly, R, 3, 4, L1, L2, 5, 6, 7, 8

        assert_eq!(
            mem::replace(&mut day.lessons[4], Lesson::Recess),
            Lesson::FreePeriod
        );

        // Some students have lessons during lunch,
        // so we check that lunch lessons are empty before replacing them
        if day.lessons[7] == Lesson::FreePeriod {
            day.lessons[7] = Lesson::Lunch1;
        }
        if day.lessons[8] == Lesson::FreePeriod {
            day.lessons[8] = Lesson::Lunch2;
        }

        // There is only assembly on Wednesday
        if *day_of_week != DayOfWeek::Wednesday {
            assert_eq!(day.lessons.remove(3), Lesson::FreePeriod);
        }
        // Changes non-wednesdays to 0, 1, 2, R, 3, 4, L1, L2, 5, 6, 7, 8

        // the website shows period 4 on a Wednesday before lunch,
        // when it actually happens after
        if *day_of_week == DayOfWeek::Wednesday {
            let period_4 = day.lessons.remove(6);
            // Ends up with: 0, 1, 2, Assembly, R, 3, L1, L2, 5, 6, 7, 8
            day.lessons.insert(8, period_4);
        }

        // Mark first period as absent if it is empty
        if day.lessons[0] == Lesson::FreePeriod {
            day.lessons[0] = Lesson::AbsentPeriod;
        };

        // Mark consecutive sequences of free periods at the end of the day as absent
        // TODO: Better handling, probably want to show frees until regular end times
        let last_class_idx = day
            .lessons
            .iter()
            .rposition(|lesson| *lesson != Lesson::FreePeriod)
            .unwrap();

        let absent_class_idxs = last_class_idx + 1..day.lessons.len();
        for idx in absent_class_idxs {
            day.lessons[idx] = Lesson::AbsentPeriod;
        }
    }

    week
}

fn scrape_lesson(node: Node<'_>) -> Lesson {
    if node.children().count() == 0 {
        return Lesson::FreePeriod;
    }

    let subject = node.find(Name("strong")).next().unwrap();
    let mut spans = node.find(Name("span"));
    spans.next(); // Skip first span which is the period number
    let teacher = spans.next().unwrap();
    let room = spans.next().unwrap();
    let lesson_code = node.find(Name("small")).next().unwrap().text();
    let (subject_code, class_code) = lesson_code.split_once(' ').unwrap();
    // Handle sports i.e. "SPTTennis 1"
    let subject_code = subject_code.strip_prefix("SPT").unwrap_or(subject_code);

    Lesson::Present {
        subject: scrape_subject(subject),
        teacher: scrape_teacher(teacher),
        room: scrape_room(room),
        subject_code: subject_code.to_string(),
        class_code: class_code.to_string(),
    }
}

fn scrape_subject(node: Node<'_>) -> Subject {
    // i.e. "HSIE | Commerce Yr9"
    let text = node.text();

    if text == "Pastoral Care Past_Car" {
        return Subject {
            name: "Assembly".to_string(),
            faculty: "Pastoral Care".to_string(),
        };
    } else if text == "Sport Sport" {
        return Subject {
            name: "Sport".to_string(),
            faculty: "Phys.Ed".to_string(),
        };
    }

    let (name, faculty) = if text.contains('|') {
        let mut components = text.split('|');
        let faculty = components.next().unwrap();

        let name = components.next().unwrap_or(faculty);

        // Strip year group off
        let last_space = name.rfind(' ').unwrap();
        let name = &name[..last_space];

        (name, faculty)
    } else {
        let last_space = text.rfind(' ').unwrap();
        let text = &text[..last_space];

        (text, text)
    };

    Subject {
        name: name.trim().to_string(),
        faculty: faculty.trim().to_string(),
    }
}

fn scrape_teacher(node: Node<'_>) -> Option<Teacher> {
    let text = node.text();

    if text == "~" {
        return None;
    }

    let mut names = text.split_whitespace();
    let last_name = names.next()?;
    let first_name = names.next();

    Some(Teacher {
        first_name: first_name.map(str::to_string),
        last_name: last_name.to_string(),
    })
}

fn scrape_room(node: Node<'_>) -> Option<String> {
    let text = node.text();

    if text == "~" {
        return None;
    }

    Some(text)
}
