use swagger::*;
fn main() {
    let f_name = "swagger4.json";
    let _swagger_str:Swagger = serde_json::from_str(&std::fs::read_to_string(f_name).unwrap()).unwrap();
    let swagger_value:serde_json::Value = serde_json::from_str(&std::fs::read_to_string(f_name).unwrap()).unwrap();
        let mut a = PassiveSwaggerScan::new(swagger_value).unwrap();
        a.run(ScanType::Full);
        a.print(0);
        //print_checks_table(&a);
        //print_alerts_table(&a);
    //let _sw = swagger_str.convert_to_map(swagger_value);
    //println!("{:?}",swagger_str);
    //println!("{:?}",swagger_value);
    //let swagger_str:Swagger = serde_yaml::from_str(&std::fs::read_to_string("swagger.yaml").unwrap()).unwrap();
    //println!("{:?}",swagger_str);
}
