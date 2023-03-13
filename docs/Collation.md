# Collation

Minecraft: Dungeons의 번역 애셋에는 같은 이름이지만 대소문자가 다른 네임페이스/번역 키가 있습니다.
따라서 대소문자를 구분하지 않는 Collation에서는 Unique Key Constraint를 위반하게 되는데요.
다음 SQL 구문을 활용해 Collation을 교체하면 이를 해결할 수 있습니다.

```sql
ALTER TABLE Word CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;
ALTER TABLE LanguageFile CONVERT TO CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;
```
