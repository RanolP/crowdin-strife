# Crowdin Strife

> The EX-CROWDIN Bot for Minecraft Translation Support

## Commands

### /e2k [text: string] (TODO)

주어진 텍스트를 포함하는 원본 문자열들의 현재 번역 문자열을 보여줍니다.

### /k2e [text: string] (TODO)

주어진 텍스트를 포함하는 현재 번역 문자열들의 원본 문자열을 보여줍니다.

### /issues (TODO)

최근 이슈 5개를 보여줍니다. 추후 기능 변경이 예정되어 있으며, API 실험용 명령어입니다.

### /discussions (TODO)

최근 토론 5개를 보여줍니다. 추후 기능 변경이 예정되어 있으며, API 실험용 명령어입니다.

## 돌려보기

### wrangler 설치

이 저장소는 yarn을 사용해 wrangler 패키지를 관리합니다.
다음 명령어를 실행해, wrangler를 설치할 수 있습니다.

```sh
yarn
```

### 개발 서버 열기

다음 명령어를 사용해 개발 서버를 열 수 있습니다.

```sh
yarn dev
```

### 배포하기

다음 명령어 중 하나를 사용해 배포할 수 있습니다.

```sh
yarn deploy
```

```sh
yarn deploy --env production
```

#### 비밀 값 설정

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
