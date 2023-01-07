# timetable

> Timetable scraper and generator for my old high school

## Web UI

The website to download timetables from (written in JavaScript) is accessible at [https://timetable.lhvy.dev](https://timetable.lhvy.dev).
It provides a `.timetable` file that can be imported into the [Class Timetable](https://classtimetable.app) app for iOS and Android.
_Note: The app is not affiliated with this project._

## Scraper & Generator

The scraper and generator pulls from the school's website and creates the `.timetable` files.
In order to run the scraper, you need `COOKIE`, `HOST` and `TIMETABLE_UUID` environment variables set accordingly.
All three can be obtained by logging into the school's timetable site.

![Screenshot of the timetable site](https://raw.githubusercontent.com/lhvy/i/master/timetable-preview.png)
