# Crowdin Strife

> The EX-CROWDIN Bot for Minecraft Translation Support

## Features

### Search TM (TODO)

Minecraft와 관련된 제품의 번역 파일을 바탕으로, 대응 번역어 쌍을 검색할 수 있습니다.
각 명령어는 한글 포함 여부로 언어를 판단하며, src 옵셔을 통해 강제 지정할 수도 있습니다.

> /java [text: string] [src: 'en' | 'ko']

Minecraft: Java Edition 최신 릴리스에 포함된 번역 파일을 바탕으로 합니다.

> /bedrock [text: string] [src: 'en' | 'ko']

Minecraft for Windows에 포함된 번역 파일을 바탕으로 합니다.

> /dungeons [text: string] [src: 'en' | 'ko']

Minecraft Dungeons에 포함된 번역 파일을 바탕으로 합니다.

### 표준국어대사전 검색 (TODO)

> /stdict [text: string]

표준국어대사전에 등록된 단어를 검색하고, 품사 및 뜻을 알려줍니다.

### /issues (TODO)

최근 이슈 5개를 보여줍니다. 추후 기능 변경이 예정되어 있으며, API 실험용 명령어입니다.

### /discussions (TODO)

최근 토론 5개를 보여줍니다. 추후 기능 변경이 예정되어 있으며, API 실험용 명령어입니다.

## 돌려보기

### 애셋 설정하기

성공적으로 빌드하기 위해 `assets/lang/en_us.json`과 `assets/lang/ko_kr.json`을 만듭니다.
이후, 아래 명령어를 실행해 실제로 언어 파일을 내려받습니다.

```sh
cargo run --bin download_translations
```

### 명령어 등록하기

아래 명령어로 디스코드 봇에 명령어를 등록합니다.

```sh
cargo run --bin register_commands
```

### wrangler 설치

이 저장소는 yarn을 사용해 wrangler 패키지를 관리합니다.
다음 명령어를 실행해, wrangler를 설치할 수 있습니다.

```sh
yarn
```

### 개발 서버 열기

TODO

### 배포하기

TODO

#### 비밀 값 설정 (outdated)

Crowdin Strife 봇은 다음의 비밀값 설정을 요구합니다.

- `CROWDIN_TOKEN` : 크라우딘 공식 API 호출을 위한 토큰입니다. (현재 미사용)
- `DISCORD_PUBLIC_KEY` : 디스코드 봇 응답 인증을 위한 공개 키입니다.
- `DISCORD_APPLICATION_ID` : 디스코드 앱 ID입니다 (현재 미사용)
- `DISCORD_TOKEN` : 디스코드 봇 토큰입니다 (현재 미사용)
- `TELEGRAM_TOKEN` : 텔레그램 봇 토큰입니다 (현재 미사용)

각 값은 배포 이후, 다음 명령어로 설정할 수 있습니다.

```sh
yarn wrangler secret put <NAME>
(비밀 값 입력)
```

```sh
yarn wrangler secret put <NAME> --env production
(비밀 값 입력)
```
