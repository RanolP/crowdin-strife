# 개발 환경 설정

## cargo-make

이 프로젝트는 태스크 관리를 위해 cargo-make를 사용합니다.
아래 명령어를 실행해 cargo-make를 설치하세요.

```sh
cargo install cargo-make
```

아래 항목들은 위 설정을 마쳤다는 전제 하에 서술됩니다.

### Rust 코드 생성

```sh
cargo make prisma generate
```

를 실행해 Prisma 스키마로 Rust 소스 코드를 생성해낼 수 있습니다.

### db push

Migration 관리 없이 데이터베이스를 푸시합니다. Planetscale 등 외부에서 Migration을 관리할 경우 유용합니다.

```sh
cargo make prisma db push
```

## Planetscale CLI

Planetscale에 배포한다면, Planetscale CLI를 설치해 편리하게 데이터베이스에 접속할 수 있습니다.

Arch Linux 기준, AUR에 올라와 있는 pscale-cli를 설치하고, (선택적으로) mysql-clients까지 설치해두면 좋습니다.
