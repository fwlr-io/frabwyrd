#!/bin/sh
sqlite3 ../frabwyrd.db <<EOF

create table if not exists list(
  word  text     not null primary key,
  seen  integer  default 0,
  keep  integer  default 0
);

create table if not exists dict(
  def   text  not null,
  llm   text  not null,
  word  text  not null references list,
  primary key(llm, word)
);

create table if not exists illust(
  path   text,
  word   text  not null references list,
  llm    text  not null,
  model  text  not null,
  foreign key(llm, word) references dict(llm, word)
);

EOF