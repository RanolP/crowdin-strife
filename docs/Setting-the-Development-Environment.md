# 개발 환경 설정

## Prisma

프로젝트 경로에 `.cargo/config.toml`을 만든 다음, 아래 내용을 추가하면

```toml
[alias]
prisma = "run -p prisma --"
```

아래와 같이 편리하게 prisma cli를 사용할 수 있습니다.

```sh
cargo prisma ...
```

아래 항목들은 위 설정을 마쳤다는 전제 하에 서술됩니다.

### Rust 코드 생성

```sh
cargo prisma generate --schema crates/engine/prisma/schema.prisma
```

를 실행해 Prisma 스키마로 Rust 소스 코드를 생성해낼 수 있습니다.

### db push

Migration 관리 없이 데이터베이스를 푸시합니다. Planetscale 등 외부에서 Migration을 관리할 경우 유용합니다.

```sh
cargo prisma db push --schema crates/engine/prisma/schema.prisma
```

다만, prisma cli가 .env 파일만을 읽기 때문에, zsh 등의 셸에서는 다음과 같이 환경 변수를 평가해야 할 수 있습니다.

```sh
(eval `cat .env.development` cargo prisma db push --schema crates/engine/prisma/schema.prisma)
```

## Planetscale CLI

Planetscale에 배포한다면, Planetscale CLI를 설치해 편리하게 데이터베이스에 접속할 수 있습니다.

Arch Linux 기준, AUR에 올라와 있는 pscale-cli를 설치하고, (선택적으로) mysql-clients까지 설치해두면 좋습니다.

