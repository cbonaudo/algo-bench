// use test_project::CircBuf;

// #[test]
// fn test_new() {
//     CircBuf::new(5);
// }

// #[test]
// #[should_panic]
// fn test_new_panic() {
//     CircBuf::new(0);
// }

// #[test]
// fn test_push() {
//     let mut buffer = CircBuf::new(5);
//     buffer.push(1);
//     buffer.push(3);
//     buffer.push(5);
// }

// #[test]
// fn test_push_more() {
//     let mut buffer = CircBuf::new(3);
//     buffer.push(1);
//     buffer.push(2);
//     buffer.push(3);
//     buffer.push(4);
//     buffer.push(5);
// }

// #[test]
// fn test_values_full() {
//     let mut buffer = CircBuf::new(3);
//     buffer.push(1);
//     buffer.push(3);
//     buffer.push(5);
//     buffer.push(7);
//     assert_eq!(buffer.values(), vec![3, 5, 7]);
// }

// #[test]
// fn test_values_not_full() {
//     let mut buffer = CircBuf::new(3);
//     buffer.push(1);
//     buffer.push(3);
//     assert_eq!(buffer.values(), vec![1, 3]);
// }
