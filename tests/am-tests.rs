use std::path::PathBuf;

#[macro_export]
macro_rules! add_test {
  ($name: ident) => {
    // #[test]
    // fn $name() {
    //   let mut root = String::from(env!("CARGO_MANIFEST_DIR"));
    //   root.push_str("/tests/resources/am-tests/");
    //   root.push_str(&stringify!($name).replace("_", "-"));
    //   root.push_str("-riscv64-nemu");

    //   println!("root: {}", root);

    //   // prepare the diff file
    //   let diff = Some(PathBuf::from(root.to_string() + ".diff"));

    //   // prepare the img file
    //   let img = PathBuf::from(root.to_string() + ".bin");

    //   // start the monitor
    //   let _ = load_img(img).unwrap();
    //   let cpu = &mut cpu::Cpu::new(diff);
    //   sdb::sdb_mainloop(cpu, true);

    //   // check the result
    //   assert_eq!(cpu.state, cpu::State::Ended);
    //   assert_eq!(cpu.halt.ret, 0);
    // }
  };
}

add_test!(add_longlong);
add_test!(add);
add_test!(bit);
add_test!(bubble_sort);
add_test!(div);
add_test!(dummy);
add_test!(fact);
add_test!(fib);
add_test!(goldbach);
add_test!(if_else);
add_test!(leap_year);
add_test!(load_store);
add_test!(matrix_mul);
add_test!(max);
add_test!(min3);
add_test!(mov_c);
add_test!(movsx);
add_test!(mul_longlong);
add_test!(pascal);
add_test!(prime);
add_test!(quick_sort);
add_test!(recursion);
add_test!(select_sort);
add_test!(shift);
add_test!(shuixianhua);
add_test!(sub_longlong);
add_test!(sum);
add_test!(switch);
add_test!(to_lower_case);
add_test!(unalign);
add_test!(wanshu);
