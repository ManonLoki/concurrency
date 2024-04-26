use anyhow::Result;
use concurrency::AmapMetrics;
use rand::Rng;
/// M
const M: usize = 2;
/// N
const N: usize = 4;

fn main() -> Result<()> {
    // 追踪
    let metrics = AmapMetrics::new(&[
        "thread.worker.0",
        "thread.worker.1",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
    ]);

    for i in 0..M {
        task_work(i, metrics.clone());
    }

    for _ in 0..N {
        request_work(metrics.clone());
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("{}", metrics);
    }
}

fn task_work(idx: usize, metircs: AmapMetrics) {
    std::thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            std::thread::sleep(std::time::Duration::from_millis(rng.gen_range(500..3000)));
            // 添加计数
            metircs.inc(format!("thread.worker.{}", idx).as_str());
        }
    });
}

fn request_work(metircs: AmapMetrics) {
    std::thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            std::thread::sleep(std::time::Duration::from_millis(rng.gen_range(300..5000)));
            // 添加计数
            let req_idx = rng.gen_range(1..5);
            metircs.inc(format!("req.page.{}", req_idx).as_str());
        }
    });
}
