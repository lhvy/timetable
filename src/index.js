const express = require("express");
const bodyParser = require("body-parser");
const dotenv = require("dotenv");
const flash = require("connect-flash");
const session = require("express-session");
const cookieParser = require("cookie-parser");

// Load env
dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

const timetables_path = process.env.TIMETABLES_PATH || "./timetables";

// Support JSON-encoded bodies (form POST)
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

// Allow flashing messages to the user
app.use(cookieParser(process.env.COOKIE_SECRET || "cookie"));
app.use(
  session({
    secret: process.env.SESSION_SECRET || "express",
    cookie: { maxAge: 60000 },
    resave: false,
    saveUninitialized: false,
  })
);
app.use(flash());
app.use(function (req, res, next) {
  res.locals.messages = req.flash();
  next();
});

app.set("view engine", "ejs");

app.get("/", (req, res) => {
  res.render("index");
});

app.post("/", (req, res) => {
  const id = req.body.student;
  const first = req.body.first;
  const last = req.body.last;

  const filename = `${id}+${first}+${last}.timetable`;
  const filepath = `${timetables_path}/${filename}`;

  res.download(filepath, filename, (err) => {
    if (err) {
      req.flash("error", "Could not find timetable");
      res.status(404).redirect("/");
    }
  });
});

app.listen(port, () => {
  console.log(`Listening on port ${port}`);
});
