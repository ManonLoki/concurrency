use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
/// M
const M: usize = 4;
/// N
const N: usize = 2;

fn main() -> Result<()> {
    // 追踪
    let metrics = Metrics::new();

    for i in 0..M {
        task_work(i, metrics.clone())?;
    }

    for _ in 0..N {
        request_work(metrics.clone())?;
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("{}", metrics);
    }
}

fn task_work(idx: usize, metircs: Metrics) -> Result<()> {
    std::thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            std::thread::sleep(std::time::Duration::from_millis(rng.gen_range(500..3000)));
            // 添加计数
            metircs.inc(format!("thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

fn request_work(metircs: Metrics) -> Result<()> {
    std::thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            std::thread::sleep(std::time::Duration::from_millis(rng.gen_range(300..5000)));
            // 添加计数
            let req_idx = rng.gen_range(1..5);
            metircs.inc(format!("req.page.{}", req_idx)).ok();
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
