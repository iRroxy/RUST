use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// 闭包任务类型：能在线程间安全传递（Send）并执行一次（FnOnce）
type Job = Box<dyn FnOnce() + Send + 'static>;

/// 线程池结构体，负责管理工作线程与任务分发
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// 创建具有指定线程数量的新线程池。
    ///
    /// # Panics
    ///
    /// 当 `size` 等于 0 时抛出 panic。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "ThreadPool size must be greater than zero");

        let (sender, receiver) = mpsc::channel();

        // 使用 Arc<Mutex<...>> 包装 Receiver，以支持多线程安全共享和并发接收任务
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// 向线程池提交一个任务并异步执行。
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(ref sender) = self.sender {
            sender.send(job).unwrap();
        }
    }
}

/// 实现 Drop trait，实现优雅停机（Graceful Shutdown）
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 主动 drop sender，使所有 worker 线程的 recv() 收到 Err 并退出循环
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// 工作线程单元
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // 在独立表达式中获取锁并立即 recv，确保任务执行期间锁已释放
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(_) => {
                    println!("Worker {} disconnected; shutting down.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_thread_pool_execution() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            });
        }

        // drop pool 触发优雅停机（等待所有当前线程执行完毕）
        drop(pool);

        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }
}

