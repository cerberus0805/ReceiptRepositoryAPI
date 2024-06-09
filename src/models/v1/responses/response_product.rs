pub struct ResponseProduct {
    pub id: i32,
    pub name: String,
    pub alias: Option<String>,
    pub specification_amount: Option<i32>,
    pub specification_unit: Option<String>,
    pub specification_size: Option<String>,
    pub brand: Option<String>
}
