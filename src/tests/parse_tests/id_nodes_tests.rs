use crate::parse::nodes::id_nodes::CGet;

#[test]
fn test_id_simple() {
    let c_get_1 = CGet { name: String::from("hello") };
    let c_get_2 = CGet { name: String::from("world") };
    
    assert_ne!(c_get_1, c_get_2);
    
    let c_get_3 = CGet { name: String::from("hello") };
    
    assert_eq!(c_get_1, c_get_3);
}