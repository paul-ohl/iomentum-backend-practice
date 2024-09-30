create extension if not exists citext;
create extension if not exists "uuid-ossp";

create table users (
  id uuid default uuid_generate_v4(),
  username text unique not null,
  password_hash text not null,
  role text not null,

  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  primary key (id)
);

alter table tickets
  drop column owner_name,
  add column owner_id uuid not null,
  add constraint fk_owner foreign key (owner_id) references users(id);
