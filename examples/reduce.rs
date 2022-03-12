
use rivi_loader::DebugOption;

fn main() {
    let a = vec![1.0f32; 1024];
    let input = &vec![vec![a]];
    let mut output = vec![0.0f32; 1024];

    let vk = rivi_loader::new(DebugOption::None).unwrap();

    let module = rivi_std::reduce().unwrap();
    let shader = vk.load_shader(module, Some(vec![vec![2]])).unwrap();

    vk.compute(input, &mut output, &shader).unwrap();

    println!("Result: {:?}", output);
}