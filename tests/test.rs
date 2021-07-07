use test_project::{get_common_prefix_igni, get_common_prefix_myo, get_common_prefix_sha};


#[test]
fn test_one() {
    assert_eq!(get_common_prefix_sha(["food"].to_vec()), "food");
}

#[test]
fn test_two() {
    assert_eq!(get_common_prefix_sha(["food", "fool"].to_vec()), "foo");
}

#[test]
fn test_no_match() {
    assert_eq!(
        get_common_prefix_sha(["food", "foal", "fool", "goal"].to_vec()),
        ""
    );
}

#[test]
fn test_match() {
    assert_eq!(
        get_common_prefix_sha(["food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong","food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong"].to_vec()),
        "fo"
    );
}

#[test]
fn test_2_one() {
    assert_eq!(get_common_prefix_igni(["food"].to_vec()), "food");
}

#[test]
fn test_2_two() {
    assert_eq!(get_common_prefix_igni(["food", "fool"].to_vec()), "foo");
}

#[test]
fn test_2_no_match() {
    assert_eq!(
        get_common_prefix_igni(["food", "foal", "fool", "goal"].to_vec()),
        ""
    );
}

#[test]
fn test_2_match() {
    assert_eq!(
        get_common_prefix_myo(["food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong","food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong"].to_vec()),
        "fo"
    );
}

#[test]
fn test_3_one() {
    assert_eq!(get_common_prefix_myo(["food"].to_vec()), "food");
}

#[test]
fn test_3_two() {
    assert_eq!(get_common_prefix_myo(["food", "fool"].to_vec()), "foo");
}

#[test]
fn test_3_no_match() {
    assert_eq!(
        get_common_prefix_myo(["food", "foal", "fool", "goal"].to_vec()),
        ""
    );
}

#[test]
fn test_3_match() {
    assert_eq!(
        get_common_prefix_myo(["food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong", "food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong","food", "fool", "foal", "foals", "fonnder","follythatthiswordistoolong"].to_vec()),
        "fo"
    );
}
