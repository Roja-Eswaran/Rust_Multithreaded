use criterion::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use encode_decode_multi_thread;
use libsteg;
fn criterion_benchmark(c: &mut Criterion) {
    let thread_count_encode = 8;
    let secret_file = "bible5.txt".to_string();
    let in_dir_encode = "aux/in".to_string();
    let out_dir_encode = "./out".to_string();
    let thread_count_decode = 8;
    let in_dir_decode = "./out".to_string();
    c.bench(
        "routines",
        Benchmark::new("routine_1", move |b| {
            b.iter(|| {
                
               encode_decode_multi_thread::encode_module(
                    thread_count_encode.clone(),
                    secret_file.clone(),
                    in_dir_encode.clone(),
                    out_dir_encode.clone(),
                )

               /*encode_decode_multi_thread::decode_module(
                    thread_count_decode.clone(),
                    in_dir_decode.clone(),
                )*/

                //encode_decode_multi_thread::decode_module(
                //    thread_count_decode.clone(),
                 //   in_dir_decode.clone(),
               // )
            })
        })
       .sample_size(10)
        ,
        
    );
    /* let mut group = c.benchmark_group("My Group");
    group.bench_function("Encoding", move |b| {
        b.iter(|| {
            encode_decode_multi_thread::encode_module(
                thread_count_encode.clone(),
                secret_file.clone(),
                in_dir_encode.clone(),
                out_dir_encode.clone(),
            )
        })
    });*/
    // group.throughput(Throughput::Bytes(8 as u64));
    /* group.bench_function("Decode", |b| {
        b.iter(|| {
            encode_decode_multi_thread::decode_module(
                thread_count_decode.clone(),
                in_dir_decode.clone(),
            )
        })
    });*/

    //   group.finish();
}

criterion_group!(benches, criterion_benchmark);
//criterion_group!(benches, encode_decode_multi_thread::bench);
criterion_main!(benches);
