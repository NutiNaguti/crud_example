create table todos (
  id serial primary key,
  timestamp timestamp not null,
  end_at timestamp not null,
  importance smallint not null,
  status smallint not null,
  text varchar not null
);

create table content_links (
  id serial PRIMARY KEY,
  todo_id integer references todos (id),
  link varchar 
);
