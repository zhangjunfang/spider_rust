//! 调度模块
//! 下个版本需要重新设计这个调度器

use downloader::{Request, Method};
use engine::Crawler;
use std::sync::Arc;
use engine::Task;

/// The scheduler. 存放入调度器中的顺序：先进先出.  此处有待优化
pub struct Scheduler<ItemType>{
    pub queue: Vec<Task<ItemType>>,
}

impl<ItemType> Scheduler<ItemType>{
    //创建一个调度器
    pub fn new() -> Self{
        Scheduler{
            queue: vec![],
        }
    }
    //传输内容入队列  下个版本修改方法名为 add_task
    pub fn enqueue(&mut self, task: Task<ItemType>){
        self.queue.push(task);
    }
    //出队列。弹出队列 下个版本修改方法名为 remove_task
    pub fn dequeue(&mut self) -> Option<Task<ItemType>>{
        self.queue.pop()
    }
}
