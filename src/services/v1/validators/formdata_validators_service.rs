use crate::models::v1::forms::create_payload::FormRelationshipModelIdOrName;

#[derive(PartialEq)]
pub enum FormRelationshipModelStatus {
    None,
    Id,
    ItemName
}

pub struct FormDataValidatorService {
}

impl FormDataValidatorService {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn validate_relationship_model(&self, model: &dyn FormRelationshipModelIdOrName) -> FormRelationshipModelStatus {
        if model.get_id_field().is_none() &&  model.get_name_field().is_none() {
            return FormRelationshipModelStatus::None;
        }
        else if  model.get_id_field().is_some() {
            return FormRelationshipModelStatus::Id;
        }
        else {
            return  FormRelationshipModelStatus::ItemName;
        }
    }
}