# GitHub Issue 用途別 Agent サンプル集（複数 Agent ＋ モデル切替運用）

本ドキュメントでは、実務でよく使われる主要用途ごとに、
**そのまま登録・運用できる Agent 定義サンプル**をまとめます。

前提：

* GitHub Enterprise + Copilot Agents 環境
* Issue アサイン時に Agent を選択
* Agent ごとにモデル固定でコスト制御

---

## 1. 通常トリアージ用（最軽量・低コスト）

用途：

* Issue 分類
* 優先度推定
* 初期ラベル付与

```yaml
name: ai-triage-light
description: トリアージAgent（低コスト・分類／優先度付け）
provider: openai
model: gpt-4o-mini

capabilities:
  - issue-triage
  - label-suggestion

trigger:
  on_assign: true

behavior:
  post_comment: true
  auto_label: true
```

---

## 2. 要約専用 Agent（軽量・高速）

用途：

* 長文 Issue の要点整理
* コメント履歴の要約

```yaml
name: ai-summary
description: 要約Agent（低コスト・履歴／本文要約用）
provider: openai
model: gpt-4o-mini

capabilities:
  - issue-summary

trigger:
  on_assign: true

behavior:
  post_comment: true
  auto_label: false
```

---

## 3. バグ原因解析 Agent（中〜高品質）

用途：

* 再現条件整理
* 原因候補抽出
* 修正方針提案

```yaml
name: ai-bug-analysis
description: バグ解析Agent（標準品質・原因分析／修正方針）
provider: openai
model: gpt-4.1

capabilities:
  - root-cause-analysis
  - issue-summary

trigger:
  on_assign: true

behavior:
  post_comment: true
  auto_label: false
```

---

## 4. 高度バグ・難案件解析（Claude 高性能）

用途：

* 再現困難バグ
* 複雑依存関係
* 論理破綻の特定

```yaml
name: ai-bug-deep
description: Claude Agent（高コスト・難案件バグ解析用）
provider: anthropic
model: claude-3.5-sonnet

capabilities:
  - deep-analysis
  - root-cause-analysis

trigger:
  on_assign: true

preconditions:
  require_label: ai:heavy

permissions:
  write_comments: true
  write_labels: false
  write_assignees: false
  write_fields: false

behavior:
  post_comment: true
  add_label_on_run: ai:cost-high
```

---

## 5. 設計レビュー Agent（Claude 推奨）

用途：

* 設計妥当性チェック
* 抽象化レベル評価
* リスク指摘

```yaml
name: ai-design-review
description: Claude Agent（高コスト・設計レビュー専用）
provider: anthropic
model: claude-3.5-sonnet

capabilities:
  - design-review
  - deep-analysis

trigger:
  on_assign: true

preconditions:
  require_label: design-review

permissions:
  write_comments: true
  write_labels: false
  write_assignees: false
  write_fields: false

behavior:
  post_comment: true
  add_label_on_run: ai:cost-high
```

---

## 6. 仕様抽出・整理 Agent（Claude 推奨）

用途：

* 要件の構造化
* 箇条書き仕様化
* 曖昧点の洗い出し

```yaml
name: ai-spec-extract
description: Claude Agent（高コスト・仕様抽出／要件整理用）
provider: anthropic
model: claude-3.5-sonnet

capabilities:
  - spec-extraction
  - issue-summary

trigger:
  on_assign: true

preconditions:
  require_label: spec

permissions:
  write_comments: true
  write_labels: false
  write_assignees: false
  write_fields: false

behavior:
  post_comment: true
  add_label_on_run: ai:cost-high
```

---

## 7. PR / 実装レビュー Agent（標準品質）

用途：

* 実装観点レビュー
* 潜在バグ指摘
* 可読性改善提案

```yaml
name: ai-code-review
description: 実装レビューAgent（標準品質・コード観点）
provider: openai
model: gpt-4.1

capabilities:
  - design-review
  - issue-summary

trigger:
  on_assign: true

behavior:
  post_comment: true
  auto_label: false
```

---

## 8. プロダクトマネージャ向け整理 Agent

用途：

* 影響範囲整理
* 優先度提案
* リリース判断材料作成

```yaml
name: ai-pm-helper
description: PM支援Agent（影響整理・優先度提案用）
provider: openai
model: gpt-4.1

capabilities:
  - issue-triage
  - issue-summary

trigger:
  on_assign: true

behavior:
  post_comment: true
  auto_label: false
```

---

## 9. 運用上の推奨セット（最小構成）

実務でまず揃えるべき基本セット：

| 役割      | Agent 名          | モデル               |
| ------- | ---------------- | ----------------- |
| 通常トリアージ | ai-triage-light  | gpt-4o-mini       |
| 要約専用    | ai-summary       | gpt-4o-mini       |
| 標準解析    | ai-bug-analysis  | gpt-4.1           |
| 高度解析    | ai-bug-deep      | claude-3.5-sonnet |
| 設計レビュー  | ai-design-review | claude-3.5-sonnet |

---

## 10. 運用ルール例

推奨ルール：

* 通常 Issue → ai-triage-light
* 長文整理 → ai-summary
* バグ解析 → ai-bug-analysis
* 難案件・設計 → ai-bug-deep / ai-design-review

Claude Agent 使用条件：

* 必須ラベル（ai:heavy / design-review / spec）を付けてからアサイン
* 実行時に ai:cost-high を自動付与
* 月次で利用件数を監査

---

## 結論

* Agent を用途別に分けることで：

  * モデル切替を UI だけで実現
  * コストと品質を完全に制御
  * チーム運用が直感的に安定

本サンプル集は、
**そのまま Agent 定義テンプレートとして利用可能**な実務標準構成である。
