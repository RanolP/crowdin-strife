use kal::Command;

/// 현재 실행 중인 봇의 버전을 가져옵니다.
#[derive(Command)]
#[command(rename = "버전")]
pub struct Version;

impl Version {
    pub async fn execute(self, env: &dyn Env) -> String {
        format!(
            "버전 : {}",
            env.var("VERSION")
                .unwrap_or_else(|| "알 수 없음".to_string())
        )
    }
}
