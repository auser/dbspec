#![cfg(feature = "fake")]
use chrono::DateTime;
use chrono::Utc;
use fakeit::person::gender;
use fakeit::words;
use fakeit::{datetime, unique};
use rand::Rng;

pub trait Faking {
    fn fake() -> String;
}

#[macro_export]
macro_rules! faking {
    ($($struct_name:ident, $field_name:ident: $field_type:ty, $fn:expr);*;) => {
        // Implement the Faking trait for each struct
        $(
            #[derive(Debug)]
            pub struct $struct_name;

            impl Faking for $struct_name {
                fn fake() -> String {
                    $fn().to_string()
                }
            }
        )*

        #[allow(dead_code)]
        pub fn match_struct(name: &str) -> String {
            match name {
                $(
                    stringify!($field_name) => <$struct_name as Faking>::fake(),
                )*
                _ => panic!("Unknown struct type requested"),
            }
        }
    };
}

faking!(
    Id, id: String, || unique::uuid_v4();
    CreatedAt, created_at: chrono::DateTime<Utc>, random_datetime;
    UpdatedAt, updated_at: chrono::DateTime<Utc>, random_datetime;
    Gender, gender: String, || gender();
    Age, age: i32, || random_number_between(18, 100);
    Ethnicity, ethnicity: String, || random_number_between(1, 5).to_string();
    Note, note: String, || words::sentence(20);
    SessionId, session_id: String, || unique::uuid_v4();
);

fn random_number_between(start: i32, end: i32) -> i32 {
    rand::thread_rng().gen_range(start..=end)
}
fn random_datetime() -> DateTime<Utc> {
    let data = datetime::date();
    DateTime::from_timestamp(data.secs, data.nsecs).expect("invalid or out-of-range datetime")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faking() {
        let res = match_struct("id");
        assert_eq!(res.len(), 36);
    }
}
