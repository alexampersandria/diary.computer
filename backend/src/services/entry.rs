use std::sync::LazyLock;

use crate::{
  establish_connection,
  schema::{self},
  services::{
    get_user,
    tag::{get_tag, Tag},
    Paginated, PaginationObject,
  },
  util::{self, APIError},
};
use diesel::{
  define_sql_function,
  dsl::sql,
  prelude::{Insertable, Queryable},
  sql_types::{Bool, Nullable, VarChar},
  ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl, RunQueryDsl,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub static DATE_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$").unwrap());

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = schema::entries)]
pub struct Entry {
  pub id: String,
  pub user_id: String,
  pub created_at: i64,
  pub mood: i32,
  pub entry: Option<String>,
  pub date: chrono::NaiveDate,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateEntry {
  #[validate(regex(path = *DATE_REGEX))]
  pub date: String,
  #[validate(range(min = 1, max = 5))]
  pub mood: i32,
  #[validate(length(min = 0, max = 1000))]
  pub entry: Option<String>,
  pub selected_tags: Vec<String>,
  #[validate(length(min = 1, max = 255))]
  pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EditEntry {
  #[validate(length(min = 1, max = 255))]
  pub id: String,
  #[validate(regex(path = *DATE_REGEX))]
  pub date: String,
  #[validate(range(min = 1, max = 5))]
  pub mood: i32,
  #[validate(length(min = 0, max = 1000))]
  pub entry: Option<String>,
  pub selected_tags: Vec<String>,
  #[validate(length(min = 1, max = 255))]
  pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryWithTags {
  pub id: String,
  pub user_id: String,
  pub date: chrono::NaiveDate,
  pub created_at: i64,
  pub mood: i32,
  pub entry: Option<String>,
  pub selected_tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = schema::entry_tags)]
pub struct EntryTag {
  pub id: String,
  pub entry_id: String,
  pub tag_id: String,
}

pub fn create_entry(entry: CreateEntry) -> Result<EntryWithTags, APIError> {
  match entry.validate() {
    Ok(_) => (),
    Err(_) => return Err(APIError::BadRequest),
  }

  let naive_date = match chrono::NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d") {
    Ok(date) => date,
    Err(_) => return Err(APIError::BadRequest),
  };

  if get_entry_by_date(naive_date, &entry.user_id).is_ok() {
    return Err(APIError::EntryAlreadyExistsForDate);
  }

  let mut tags: Vec<Tag> = Vec::new();

  for tag_id in &entry.selected_tags {
    let tag = get_tag(tag_id, &entry.user_id);

    if let Ok(tag) = tag {
      tags.push(tag);
    }
  }

  let user = get_user(&entry.user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let new_entry = Entry {
    id: Uuid::new_v4().to_string(),
    user_id: entry.user_id.clone(),
    date: naive_date,
    created_at: util::unix_ms(),
    mood: entry.mood,
    entry: entry.entry.clone(),
  };

  let result = diesel::insert_into(schema::entries::table)
    .values(&new_entry)
    .execute(&mut conn);

  if result.is_err() {
    return Err(APIError::DatabaseError);
  }

  for tag in &tags {
    let entry_tag = EntryTag {
      id: Uuid::new_v4().to_string(),
      entry_id: new_entry.id.clone(),
      tag_id: tag.id.clone(),
    };

    let tag_result = diesel::insert_into(schema::entry_tags::table)
      .values(&entry_tag)
      .execute(&mut conn);

    if tag_result.is_err() {
      return Err(APIError::DatabaseError);
    }
  }

  let entry_with_tags = EntryWithTags {
    id: new_entry.id,
    user_id: new_entry.user_id,
    date: new_entry.date,
    created_at: new_entry.created_at,
    mood: new_entry.mood,
    entry: new_entry.entry,
    selected_tags: entry.selected_tags,
  };

  Ok(entry_with_tags)
}

pub fn edit_entry(entry: EditEntry) -> Result<EntryWithTags, APIError> {
  match entry.validate() {
    Ok(_) => (),
    Err(_) => return Err(APIError::BadRequest),
  }

  let naive_date = match chrono::NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d") {
    Ok(date) => date,
    Err(_) => return Err(APIError::BadRequest),
  };

  if let Ok(entry_for_date) = get_entry_by_date(naive_date, &entry.user_id) {
    if entry_for_date.id != entry.id {
      return Err(APIError::EntryAlreadyExistsForDate);
    }
  }

  let mut tags: Vec<Tag> = Vec::new();

  for tag_id in &entry.selected_tags {
    let tag = get_tag(tag_id, &entry.user_id);

    if let Ok(tag) = tag {
      tags.push(tag);
    }
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let result = diesel::update(
    schema::entries::table
      .filter(schema::entries::id.eq(&entry.id))
      .filter(schema::entries::user_id.eq(&entry.user_id)),
  )
  .set((
    schema::entries::date.eq(&naive_date),
    schema::entries::mood.eq(entry.mood),
    schema::entries::entry.eq(&entry.entry),
  ))
  .execute(&mut conn);

  if result.is_err() {
    return Err(APIError::DatabaseError);
  }

  let delete_result = diesel::delete(
    crate::schema::entry_tags::table.filter(crate::schema::entry_tags::entry_id.eq(&entry.id)),
  )
  .execute(&mut conn);

  if delete_result.is_err() {
    return Err(APIError::DatabaseError);
  }

  for tag in &tags {
    let entry_tag = EntryTag {
      id: Uuid::new_v4().to_string(),
      entry_id: entry.id.to_string(),
      tag_id: tag.id.to_string(),
    };

    let tag_result = diesel::insert_into(crate::schema::entry_tags::table)
      .values(&entry_tag)
      .execute(&mut conn);

    if tag_result.is_err() {
      return Err(APIError::DatabaseError);
    }
  }

  get_entry_with_tags(&entry.id, &entry.user_id)
}

pub fn get_entry_by_date(date: chrono::NaiveDate, user_id: &str) -> Result<Entry, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let result = schema::entries::table
    .filter(schema::entries::date.eq(date))
    .filter(schema::entries::user_id.eq(user_id))
    .first::<Entry>(&mut conn);

  match result {
    Ok(entry) => Ok(entry),
    Err(_) => Err(APIError::EntryNotFound),
  }
}

pub fn get_entry_with_tags(entry_id: &str, user_id: &str) -> Result<EntryWithTags, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let entry_result = schema::entries::table
    .filter(schema::entries::id.eq(entry_id))
    .filter(schema::entries::user_id.eq(user_id))
    .first::<Entry>(&mut conn);

  let entry = match entry_result {
    Ok(entry) => entry,
    Err(_) => return Err(APIError::EntryNotFound),
  };

  let entry_tags_result = schema::entry_tags::table
    .filter(schema::entry_tags::entry_id.eq(entry_id))
    .load::<EntryTag>(&mut conn);

  let entry_tags = match entry_tags_result {
    Ok(entry_tags) => entry_tags,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let tag_ids: Vec<String> = entry_tags.into_iter().map(|et| et.tag_id).collect();

  let entry_with_tags = EntryWithTags {
    id: entry.id,
    user_id: entry.user_id,
    date: entry.date,
    created_at: entry.created_at,
    mood: entry.mood,
    entry: entry.entry,
    selected_tags: tag_ids,
  };

  Ok(entry_with_tags)
}

pub fn delete_entry(entry_id: &str, user_id: &str) -> Result<bool, APIError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let result = diesel::delete(
    schema::entries::table
      .filter(schema::entries::id.eq(entry_id))
      .filter(schema::entries::user_id.eq(user_id)),
  )
  .execute(&mut conn);

  match result {
    Ok(count) => Ok(count > 0),
    Err(_) => Err(APIError::DatabaseError),
  }
}

#[derive(Debug, Deserialize)]
pub enum EntryOptionsOrder {
  DateAsc,
  DateDesc,
  MoodAsc,
  MoodDesc,
}

#[derive(Debug, Deserialize)]
pub struct GetEntriesOptions {
  pub from_date: Option<String>,
  pub to_date: Option<String>,
  pub tags: Option<Vec<String>>,
  pub from_mood: Option<i32>,
  pub to_mood: Option<i32>,
  pub order: Option<EntryOptionsOrder>,
  pub limit: Option<i64>,
  pub offset: Option<i64>,
}

impl Default for GetEntriesOptions {
  fn default() -> Self {
    GetEntriesOptions {
      from_date: None,
      to_date: None,
      tags: None,
      from_mood: None,
      to_mood: None,
      order: Some(EntryOptionsOrder::DateDesc),
      limit: Some(31),
      offset: Some(0),
    }
  }
}

pub fn get_entries(
  user_id: &str,
  options: Option<GetEntriesOptions>,
) -> Result<Paginated<EntryWithTags>, APIError> {
  define_sql_function!(
    #[aggregate]
    fn array_agg(x: Nullable<VarChar>) -> Array<Nullable<VarChar>>;
  );

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let selected_tags = array_agg(schema::entry_tags::tag_id.nullable());
  let row_count = sql::<diesel::sql_types::BigInt>("COUNT(*) OVER()");

  let mut pagination = PaginationObject {
    limit: 31,
    offset: 0,
    total_count: 0,
  };

  let mut query = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .left_join(schema::entry_tags::table.on(schema::entries::id.eq(schema::entry_tags::entry_id)))
    .group_by(schema::entries::id)
    .select((
      schema::entries::id,
      schema::entries::user_id,
      schema::entries::created_at,
      schema::entries::mood,
      schema::entries::entry,
      schema::entries::date,
      selected_tags,
      row_count,
    ))
    .into_boxed();

  match options {
    Some(options) => {
      if let Some(from_date) = options.from_date {
        let from_naive_date = match chrono::NaiveDate::parse_from_str(&from_date, "%Y-%m-%d") {
          Ok(date) => date,
          Err(_) => return Err(APIError::BadRequest),
        };
        query = query.filter(schema::entries::date.ge(from_naive_date));
      }

      if let Some(to_date) = options.to_date {
        let to_naive_date = match chrono::NaiveDate::parse_from_str(&to_date, "%Y-%m-%d") {
          Ok(date) => date,
          Err(_) => return Err(APIError::BadRequest),
        };
        query = query.filter(schema::entries::date.le(to_naive_date));
      }

      if let Some(from_mood) = options.from_mood {
        query = query.filter(schema::entries::mood.ge(from_mood));
      }

      if let Some(to_mood) = options.to_mood {
        query = query.filter(schema::entries::mood.le(to_mood));
      }

      if let Some(tags) = options.tags {
        if !tags.is_empty() {
          let tag_list = tags
            .iter()
            .map(|t| format!("'{}'", t.replace("'", "''")))
            .collect::<Vec<_>>()
            .join(", ");
          // build the having clause
          // when using diesels overlaps_with we get the error:
          // `operator does not exist: character varying[] && text[]`
          // because the tag_list is not of the same type as entry_tags.tag_id
          // "(Diesel does not currently support implicit coercions)."
          // #TODO: find a way to achieve this natively with diesel
          // but this is acceptable since it allows us to array_agg and filter/limit/offset in a single query
          let having_clause =
            format!("ARRAY_AGG(entry_tags.tag_id) @> ARRAY[{tag_list}]::varchar[]");
          query = query.having(sql::<Bool>(&having_clause));
        }
      }

      match options.order {
        Some(EntryOptionsOrder::DateAsc) => {
          query = query.order(schema::entries::date.asc());
        }
        Some(EntryOptionsOrder::DateDesc) => {
          query = query.order(schema::entries::date.desc());
        }
        Some(EntryOptionsOrder::MoodAsc) => {
          query = query.order((schema::entries::mood.asc(), schema::entries::date.asc()));
        }
        Some(EntryOptionsOrder::MoodDesc) => {
          query = query.order((schema::entries::mood.desc(), schema::entries::date.desc()));
        }
        None => query = query.order(schema::entries::date.desc()),
      }

      match options.limit {
        Some(limit) => {
          if limit > 0 {
            query = query.limit(limit);
          }
          pagination.limit = limit;
        }
        None => query = query.limit(pagination.limit),
      }

      match options.offset {
        Some(offset) => {
          query = query.offset(offset);
          pagination.offset = offset;
        }
        None => query = query.offset(pagination.offset),
      }
    }

    // no options provided, use defaults
    None => {
      query = query
        .order(schema::entries::date.desc())
        .limit(pagination.limit)
        .offset(pagination.offset);
    }
  }

  let result = query.load::<(
    String,
    String,
    i64,
    i32,
    Option<String>,
    chrono::NaiveDate,
    Vec<Option<String>>,
    i64,
  )>(&mut conn);

  let rows = match result {
    Ok(rows) => rows,
    Err(_) => return Err(APIError::DatabaseError),
  };

  if let Some(first_row) = &rows.first() {
    pagination.total_count = first_row.7;
  }

  let mut entries_with_tags: Vec<EntryWithTags> = Vec::new();

  for row in rows {
    let tag_ids = row.6.into_iter().flatten().collect();

    let entry_with_tags = EntryWithTags {
      id: row.0,
      user_id: row.1,
      created_at: row.2,
      mood: row.3,
      entry: row.4,
      date: row.5,
      selected_tags: tag_ids,
    };

    entries_with_tags.push(entry_with_tags);
  }

  Ok(Paginated {
    data: entries_with_tags,
    pagination,
  })
}
