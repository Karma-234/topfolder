pub fn my_enum() {
    enum Gender {
        Male(String),
        Female(String),
    }
    let male_gender: Gender = Gender::Male(String::from("Male"));
    let female_gender: Gender = Gender::Female(String::from("Female"));

    // println!("Male gender: {:?}", male_gender);
    // println!("Female gender: {:?}", female_gender);
    // struct GenderStruct {
    //     kind: Gender,
    //     name: String,
    // }
    // let my_inst_1 = GenderStruct {
    //     kind: male_gender,
    //     name: "John".to_string(),
    // };
}
