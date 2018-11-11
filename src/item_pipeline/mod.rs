//! 通道传输对象

use spider::Spider;

/// 处理itempipeline结果
pub enum ItemProduct<T>{
    /// 一个item将被传输到下一个itempipeline.
    Item(T),
    /// item再也不需要被处理.
    Ignore,
}
/// item通道
pub trait ItemPipeline: Send{
    type ItemType;
    /// item被调用时调用 ，有且只有一次
    fn process_item(&mut self, item: Self::ItemType) -> ItemProduct<Self::ItemType>;
    /// 当爬虫工具被打开调用，有且仅有一次
    fn open_spider(&mut self, spider: &Box<Spider<ItemType=Self::ItemType>>){
        // 默认情况下 ，什么也不做
    }
    /// 当爬虫工具被关闭时调用，有且仅有一次
    fn close_spider(&mut self, spider: &Box<Spider<ItemType=Self::ItemType>>){
        // 默认情况下 ，什么也不做
    }
}
