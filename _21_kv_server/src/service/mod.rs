use crate::storage::Storage;

mod command_service;

// 对 Command 的处理的抽象
// pub trait CommandService {
//     // 处理 command ，返回 Response
//     fn execute(self, store: & impl Storage) -> CommandResponse;
// }