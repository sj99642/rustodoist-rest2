/// Used to define the creation of a new task.
///
/// `content` must be specified in the creation of a new task, but all other fields are optional.
#[derive(Debug)]
pub struct NewTask {
    pub content: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub section_id: Option<String>,
    pub parent_id: Option<String>,
    pub order: Option<i32>,
    pub labels: Option<Vec<String>>,
    pub priority: Option<u8>,
    pub due: Option<NewDue>,
    pub assignee_id: Option<String>,
    pub duration: Option<u32>,
    pub duration_unit: Option<String>,
}


// The following implementation is taken directly from the exapnded automatic implementation of
// Serialize. The only change is in the "due" field. Instead of recursing into the NewDue enum,
// representing this as a single compound structure under the "new" title, we instead check which
// variant of the enum is being used, and then serialize the appropriate field.
impl serde::Serialize for NewTask {
    fn serialize<__S>(
        &self,
        __serializer: __S,
    ) -> Result<__S::Ok, __S::Error>
        where
            __S: serde::Serializer,
    {
        let mut _serde_state = serde::Serializer::serialize_struct(
            __serializer,
            "NewTask",
            false as usize + 12,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "content", &self.content)?;
        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "description", &self.description)?;
        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "project_id", &self.project_id)?;
        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "section_id", &self.section_id)?;
        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "parent_id", &self.parent_id)?;
        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "order", &self.order)?;
        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "labels", &self.labels)?;
        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "priority", &self.priority)?;
        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "assignee_id", &self.assignee_id)?;
        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "duration", &self.duration)?;
        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "duration_unit", &self.duration_unit)?;
        if let Some(due) = &self.due {
            match due {
                NewDue::String { string, lang } => {
                    serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "due_string", &string)?;
                    if let Some(lang) = lang {
                        serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "due_lang", &lang)?;
                    }
                },
                NewDue::Date(date) => {
                    serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "due_date", &date)?;
                },
                NewDue::Datetime(datetime) => {
                    serde::ser::SerializeStruct::serialize_field(&mut _serde_state, "due_datetime", &datetime, )?;
                },
            }
        }
        serde::ser::SerializeStruct::end(_serde_state)
    }
}


impl NewTask {
    pub fn new(content: &str) -> NewTask {
        NewTask {
            content: content.to_string(),
            description: None,
            project_id: None,
            section_id: None,
            parent_id: None,
            order: None,
            labels: None,
            priority: None,
            due: None,
            assignee_id: None,
            duration: None,
            duration_unit: None,
        }
    }
}


#[derive(Debug)]
pub enum NewDue {
    String { string: String, lang: Option<String> },
    Date(String),
    Datetime(String),
}

