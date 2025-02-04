use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ark_std::str::FromStr;
use banderwagon::{Element, msm_gotti::MSMPrecompWnafGotti, Fr};
use banderwagon::msm::MSMPrecompWnaf;

fn benchmark_precompute_mul(c: &mut Criterion) {
    // Create a vector of 256 elements, each being a multiple of the prime subgroup generator
    let basis_num = 1;
    let mut basic_crs = Vec::with_capacity(basis_num);
    for i in 0..basis_num {
        basic_crs.push(Element::prime_subgroup_generator() * Fr::from((i + 1) as u64));
    }

    let scalars = vec![
        Fr::from_str("13108968793781547619861935127046491459309155893440570251786403306729687672800").unwrap()
    ];
    let ts=[2,3];
    let bs=[6,7,8,9,10,11,12,13,14,15,16];

    for &t in &ts {
        for &b in &bs {
            let precompute = MSMPrecompWnafGotti::new_window(&basic_crs, t, b);
            let mem_byte_size=precompute.tables.len()*precompute.tables[0].len()*4*32;
            println!("precompute_byte_size: {:?}", mem_byte_size);

            c.bench_function(&format!("prempute_mul_t{}_b{}", t, b), |b| {
                b.iter(|| {
                    let result = precompute.mul_window(black_box(&scalars));
                    black_box(result);
                })
            });
        }
    }
}
fn benchmark_std_precompute_mul(c: &mut Criterion) {
    let basis_num = 1;
    let mut basic_crs = Vec::with_capacity(basis_num);
    for i in 0..basis_num {
        basic_crs.push(Element::prime_subgroup_generator() * Fr::from((i + 1) as u64));
    }

    let scalars = vec![
        Fr::from_str("13108968793781547619861935127046491459309155893440570251786403306729687672800").unwrap()
    ];
    let windows=[6,7,8,9,10,11,12,13,14,15,16];

    for &window in &windows {
        let precompute = MSMPrecompWnaf::new(&basic_crs, window);
        let mem_byte_size=precompute.tables.len()*precompute.tables[0].len()*4*32;
        println!("precompute_byte_size: {:?}", mem_byte_size);

        c.bench_function(&format!("std_prempute_mul_param_{}", window), |b| {
            b.iter(|| {
                let result = precompute.mul(black_box(&scalars));
                black_box(result);
            })
        });
    }
}
criterion_group!(benches, benchmark_precompute_mul,benchmark_std_precompute_mul);
criterion_main!(benches);