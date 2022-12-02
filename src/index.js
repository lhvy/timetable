const express = require("express");
const bodyParser = require("body-parser");
const dotenv = require("dotenv");
const flash = require("connect-flash");
const session = require("express-session");
const cookieParser = require("cookie-parser");
const rateLimit = require("express-rate-limit");
const bunyan = require("bunyan");

// Load env
dotenv.config();

const log = bunyan.createLogger({
  name: "timetable",
  streams: [
    {
      level: "info",
      stream: process.stdout,
    },
    {
      level: "info",
      path: "log.json",
    },
  ],
});

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

const limiter = rateLimit({
  windowMs: 60 * 60 * 1000, // 30 minutes
  max: 50, // Limit each IP to 50 requests per window
  standardHeaders: true, // Return rate limit info in the `RateLimit-*` headers
  legacyHeaders: false, // Disable the `X-RateLimit-*` headers
});

app.use(limiter);

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
      log.error({ id, first, last }, "Error downloading timetable");
      req.flash("error", "Could not find timetable");
      res.status(404).redirect("/");
    } else {
      log.info({ id, first, last }, "Downloaded timetable");
    }
  });
});

app.get("/tutorial", (req, res) => {
  res.render("tutorial");
});

app.listen(port, () => {
  console.log(`Listening on port ${port}`);
});
