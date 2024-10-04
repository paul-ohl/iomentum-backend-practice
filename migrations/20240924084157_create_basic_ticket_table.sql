create extension if not exists citext;
create extension if not exists "uuid-ossp";

create table tickets (
  id uuid default uuid_generate_v4(),
  owner_name text not null,

  concert_name text not null,
  concert_date timestamptz not null,
  barcode_data text not null,
  price float not null,

  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  primary key (id)
);
