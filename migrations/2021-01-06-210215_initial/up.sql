CREATE TABLE mediathek_entries (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  station VARCHAR(255) NOT NULL,
  topic VARCHAR(255),
  title VARCHAR(255) NOT NULL,
  date VARCHAR(255),
  time VARCHAR(255),
  duration VARCHAR(255),
  size VARCHAR(255),
  description VARCHAR(255),
  url VARCHAR(255) NOT NULL,
  website VARCHAR(255),
  url_subtitles VARCHAR(255),
  url_rtmp VARCHAR(255),
  url_small VARCHAR(255),
  url_rtmp_small VARCHAR(255),
  url_hd VARCHAR(255),
  url_rtmp_hd VARCHAR(255),
  datuml VARCHAR(255),
  url_history VARCHAR(255),
  geo VARCHAR(255),
  new VARCHAR(255)
)
