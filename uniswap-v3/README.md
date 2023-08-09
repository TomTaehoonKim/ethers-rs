## 유니스왑 v3를 수정하면서 테스트 할 수 있는 환경

`pip install eth-brownie`
`brownie run deploy -i # scripts/deploy.py가 실행됨`

## 참고사항
- `~/.brownie/network-config.yaml` 에서 rpc 를 추가하면 커스텀 체인 접속 가능
- 테스트 계정은 private key가 모두 `a`인 지갑과 브라우니 기본 지갑을 사용하여 커스텀 체인에서는 변경 필요
- 유니스왑의 `POOL_INIT_CODE_HASH`를 변경함 (변경하지 않고 빌드 방법을 못찾음), core와 periphery는 tag 1.0.0