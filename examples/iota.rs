use rivi_loader::DebugOption;

fn main() {
    let a = vec![1030u32];
    let input = &vec![vec![a]];
    let mut output = vec![0u32; 1030];

    let vk = rivi_loader::new(DebugOption::None).unwrap();

    let module = rivi_std::index_generator().unwrap();
    let shader = vk.load_shader(module, None).unwrap();

    vk.compute(input, &mut output, &shader).unwrap();

    println!("Result: {:?}", output);
}
