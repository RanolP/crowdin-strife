use engine::env::Env;
use kal::Command;

use crate::message::StructuredMessageBox;

/// 현재 실행 중인 봇의 버전을 가져옵니다.
#[derive(Command)]
#[command(rename = "버전")]
pub struct Version;

impl Version {
    pub async fn execute(self, env: &(dyn Env + Sync + Send)) -> StructuredMessageBox {
        Box::new(format!(
            "버전 : {}",
            env.get("VERSION")
                .unwrap_or_else(|| "알 수 없음".to_string())
        ))
    }
}
